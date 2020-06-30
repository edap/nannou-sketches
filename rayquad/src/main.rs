
use nannou::prelude::*;
use nannou::rand::rngs::StdRng;
use nannou::rand::{Rng, SeedableRng};
use nannou::color::gradient::Gradient;
use edapx_colors::Palette;

fn main() {
    nannou::app(model).run();
}

struct Ray2D {
    orig: Vector2,
    dir: Vector2,
}

impl Ray2D{
    pub fn new() -> Self {
        Ray2D{
            orig: vec2(0.0, 0.0),
            dir: vec2(1.0, 0.0),
        }
    }

    pub fn debug_ray(&self, draw: &Draw, mag: f32) {
        draw.line().color(RED).start(self.orig).end(self.dir.with_magnitude(mag));
    }


    pub fn look_at(&mut self, x: f32, y: f32) {
        self.dir.x = (x - self.orig.x);
        self.dir.y = (y - self.orig.y);
        self.dir.normalize();
    }

    pub fn intersect(&self, x1: f32, y1: f32, x2: f32, y2: f32) -> Option<Vector2> {
        let x3 = self.orig.x;
        let y3 = self.orig.y;
        let x4 = self.orig.x + self.dir.x;
        let y4 = self.orig.y + self.dir.y;
        let den = (x1 - x2) * (y3 - y4) - (y1 - y2) * (x3 - x4);

        let tri = (
            den,
            ((x1 - x3) * (y3 - y4) - (y1 - y3) * (x3 - x4)) / den,
            -((x1 - x2) * (y1 - y3) - (y1 - y2) * (x1 - x3)) / den,
        );


        match tri {
            (d, t, u) if d != 0.0 && t > 0.0 && t < 1.0 && u > 0.0 =>
                Some(vec2(x1 + t * (x2 - x1), y1 + t * (y2 - y1))),
            _ => None,

        }

        // if (t > 0.0 && t < 1.0 && u > 0.0) {
        //   Some(vec2(x1 + t * (x2 - x1), y1 + t * (y2 - y1)));
        // } else {
        //   None;
        // }

    }
}

struct Model {
    scheme_id: usize,
    palette: Palette,
    gradient_one: Gradient<Hsl>,
    gradient_two: Gradient<Hsl>,
    gradient_three: Gradient<Hsl>,
    blend_id: usize,
    act_random_seed: u64,
    point: Vector2
}

fn model(app: &App) -> Model {
    app.new_window()
        //.size(800, 800)
        .view(view)
        .key_pressed(key_pressed)
        .mouse_pressed(mouse_pressed)
        .build()
        .unwrap();

    let scheme_id = 0;
    let palette = Palette::new();
    let scheme = palette.get_scheme(scheme_id);

    let gradient_one = Gradient::new(vec![Hsl::from(scheme[0]),Hsl::from(scheme[2])]);
    let gradient_two = Gradient::new(vec![Hsl::from(scheme[1]),Hsl::from(scheme[3])]);
    let gradient_three = Gradient::new(vec![Hsl::from(scheme[4]),Hsl::from(scheme[1])]);
    Model {
        scheme_id,
        palette,
        gradient_one,
        gradient_two,
        gradient_three,
        blend_id: 0,
        act_random_seed: 0,
        point: vec2(0.0,0.0),
    }
}

fn key_pressed(app: &App, model: &mut Model, key: Key) {
    match key {
        Key::Key1 => {
            model.scheme_id = 0;
        }
        Key::Key2 => {
            model.scheme_id = 1;
        }
        Key::Key3 => {
            model.scheme_id = 2;
        }
        Key::Key4 => {
            model.scheme_id = 3;
        }
        Key::Key5 => {
            model.scheme_id = 4;
        }
        Key::Key6 => {
            model.scheme_id = 5;
        }
        Key::Q => {
            model.blend_id = 0;
        }
        Key::W => {
            model.blend_id = 1;
        }
        Key::E => {
            model.blend_id = 2;
        }
        Key::R => {
            model.blend_id = 3;
        }

        Key::S => {
            app.main_window()
                .capture_frame(app.time.to_string() + ".png");
            //.capture_frame(app.exe_name().unwrap() + ".png");
        }
        _other_key => {}
    }

    let scheme_id = model.scheme_id;
    let scheme = model.palette.get_scheme(scheme_id);
    model.gradient_one = Gradient::new(vec![Hsl::from(scheme[0]),Hsl::from(scheme[2])]);
    model.gradient_two = Gradient::new(vec![Hsl::from(scheme[3]),Hsl::from(scheme[4])]);
    model.gradient_three = Gradient::new(vec![Hsl::from(scheme[4]),Hsl::from(scheme[0])]);
}

fn mouse_pressed(_app: &App, model: &mut Model, _button: MouseButton) {
    model.act_random_seed = (random_f32() * 100000.0) as u64;
}

fn view(app: &App, model: &Model, frame: Frame) {
    let start_line = vec2(300.0,100.0);
    let end_line = vec2(300.0,-300.0);
    let mut r = Ray2D::new();
    r.look_at(app.mouse.x, app.mouse.y);




    let draw = app.draw();
    let win = app.window_rect();
    let tile_count_w = map_range(app.mouse.x, win.w()*-1.0, win.w(), 1, 8) as u32;
    let tile_count_h = (win.h() * 2.0).abs() as u32 / tile_count_w;
    let mut rng = StdRng::seed_from_u64(model.act_random_seed);

    let tot = tile_count_w * tile_count_h;


    draw.background().color(PLUM);

    draw.ellipse().color(STEELBLUE).x_y(app.mouse.x, app.mouse.y);
    r.debug_ray(&draw, 200.0);
    draw.line().color(STEELBLUE).start(start_line).end(end_line);

    if let r.intersect(start_line.x,start_line.y, end_line.x, end_line.y) = collision{
        draw.ellipse().color(GREEN).x_y(collision.x, collision.y);
    };

    draw.to_frame(app, &frame).unwrap();
}

