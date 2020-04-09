
use nannou::prelude::*;
use nannou::rand::rngs::StdRng;
use nannou::rand::{Rng, SeedableRng};
use nannou::color::gradient::Gradient;
use edapx_colors::Palette;

fn main() {
    nannou::app(model).run();
}

struct Model {
    scheme_id: usize,
    palette: Palette,
    gradient_one: Gradient<Hsl>,
    gradient_two: Gradient<Hsl>,
    gradient_three: Gradient<Hsl>,
    blend_id: usize,
    act_random_seed: u64,
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
    let win = app.window_rect();
    let tile_count_w = map_range(app.mouse.x, win.w()*-1.0, win.w(), 1, 8) as u32;
    let tile_count_h = (win.h() * 2.0).abs() as u32 / tile_count_w;
    let mut rng = StdRng::seed_from_u64(model.act_random_seed);

    let blends = [
        BLEND_NORMAL,
        BLEND_ADD,
        BLEND_SUBTRACT,
        BLEND_LIGHTEST,
    ];
    let draw = app.draw().color_blend(blends[model.blend_id].clone());
    frame.clear(model.palette.get_scheme(model.scheme_id)[4]);

    let tot = tile_count_w * tile_count_h;

    for i in 0..tot {
        let tile_size = win.w() / tile_count_w as f32;
        let x = (i % tile_count_w) as f32 * tile_size - win.w() * 0.5 + tile_size / 2.0;
        let y = (i / tile_count_w) as f32 * tile_size - win.h() * 0.5 + tile_size / 2.0;
        let mut draw = draw.x_y(x, y);
        let toggle = rng.gen_range(0, 2);
        let rotation = match toggle {
            0 => -PI,
            1 => 0.0,
            _ => unreachable!(),
        };
        draw = draw.rotate(rotation);
        draw_poly(&app, &draw, 0.0, 0.0, i, tile_size, tile_size, model, toggle);
        draw.ellipse()
            .x_y(0.0, 0.0)
            .radius(tile_size/2.0)
            //.color(BLACK);
            .no_fill()
            .stroke_weight(3.0)
            .stroke(rgba(0.0, 0.0, 0.0, 0.5));
    }

    draw.to_frame(app, &frame).unwrap();
}

fn draw_poly(_app: &App, draw: &Draw, x: f32, y: f32, _index: u32, tile_w: f32, tile_h: f32,  model: &Model, _random: u8) {
    let def = 9;
    let spaces = def -1;
    let parts = spaces + def;
    let thickness = tile_w / 9.0;
    let tot = (parts*2)+2;

    let points = (0..tot).map(|i| {
        let mut y2 = map_range(i, 1, tot, y-tile_h/2.0, y+tile_h/2.0);
        let mut x2 = map_range(i, 1, tot, x-tile_w/2.0, x+tile_w/2.0 - thickness);
        let p = (i, tot);
        match p {
            (i, total) if i == total-1 => {
                x2 = x + tile_w / 2.0;
                y2 = y + tile_h / 2.0;
            },
            (0, _)  => {
                x2 = x + tile_w / 2.0;
                y2 = y - tile_h / 2.0;
            },
            (i, total) if (i % 4 == 0) | (i % 4 == 3) && i != total => {
                x2 = x + tile_w/2.0 - thickness;
                y2 = y2.floor();
            }
            (i, total) if (i  == total) => {
                x2 = x + tile_w / 2.0;
                y2 = y - tile_h / 2.0;
            },
            _ => {},
        };
        let pick_gradient = map_range(y2, y-tile_h/2.0, y+tile_h/2.0, 0.0, 1.0);
        let g = model.gradient_one.get(pick_gradient);
        (pt2(x2, y2), g)
    });
    draw.polygon().points_colored(points);
}
