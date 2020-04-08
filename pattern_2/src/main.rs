
use nannou::prelude::*;
use nannou::rand::rngs::StdRng;
use nannou::rand::{Rng, SeedableRng};
use nannou::color::gradient::Gradient;
use edapx_colors::Palette;

fn main() {
    nannou::app(model).run();
}

struct Model {
    tile_count_x: usize,
    tile_count_y: usize,
    tile_width: f32,
    tile_height: f32,
    circle_count: usize,
    end_size: f32,
    end_offset: f32,
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
        .size(800, 800)
        .view(view)
        .key_pressed(key_pressed)
        .mouse_pressed(mouse_pressed)
        .mouse_moved(mouse_moved)
        .build()
        .unwrap();

    let scheme_id = 0;
    let palette = Palette::new();
    let scheme = palette.get_scheme(scheme_id);

    let gradient_one = Gradient::new(vec![Hsl::from(scheme[0]),Hsl::from(scheme[2])]);
    let gradient_two = Gradient::new(vec![Hsl::from(scheme[1]),Hsl::from(scheme[3])]);
    let gradient_three = Gradient::new(vec![Hsl::from(scheme[4]),Hsl::from(scheme[1])]);
    let tile_count_x = 2;
    let tile_count_y = 2;
    let win = app.window_rect();
    Model {
        tile_count_x,
        tile_count_y,
        tile_width: win.w() / tile_count_x as f32,
        tile_height: win.h() / tile_count_y as f32,
        circle_count: 0,
        end_size: 0.0,
        end_offset: 0.0,
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

fn mouse_moved(app: &App, model: &mut Model, pos: Point2) {
    let win = app.window_rect();
    model.circle_count = map_range(pos.x, win.left(), win.right(), 1, 30);

    model.end_size = map_range(pos.x, win.left(), win.right(), model.tile_width / 2.0, 0.0);
    model.end_offset = map_range(
        pos.y,
        win.bottom(),
        win.top(),
        0.0,
        (model.tile_width - model.end_size) / 2.0,
    );
}

fn view(app: &App, model: &Model, frame: Frame) {
    let win = app.window_rect();
    let tile_count = map_range(app.mouse.x, win.w()*-1.0, win.w(), 1, 8) as u32;;
    let mut rng = StdRng::seed_from_u64(model.act_random_seed);

    let blends = [
        BLEND_NORMAL,
        BLEND_ADD,
        BLEND_SUBTRACT,
        BLEND_LIGHTEST,
    ];
    let mut draw = app.draw().color_blend(blends[model.blend_id].clone());
    frame.clear(model.palette.get_scheme(model.scheme_id)[4]);

    let tot = tile_count * tile_count;;

    // println!("circle_count {} || end_size {} || end offset {}", model.circle_count, model.end_size, model.end_offset);
    //draw.x_y(model.tile_width / 2.0, model.tile_height / 2.0);

    for i in 0..tot {
        let tile_w = win.w() / tile_count as f32;
        let tile_h = win.h() / tile_count as f32;
        let x = (i % tile_count) as f32 * tile_w - win.w() * 0.5 + tile_w / 2.0;
        let y = (i / tile_count) as f32 * tile_h - win.h() * 0.5 + tile_h / 2.0;
        let mut draw = draw.x_y(x, y);
        println!("x {} || y {}", x, y);
        let scale = model.tile_width / model.tile_height;
        //draw = draw.scale(scale);
        let toggle = rng.gen_range(0, 4);
        let rotation = match toggle {
            0 => -(PI / 2.0),
            1 => 0.0,
            2 => PI / 2.0,
            3 => PI,
            _ => unreachable!(),
        };
        //draw = draw.rotate(rotation);

        //draw_poly(&app, &draw, -model.tile_width/2., 0.0, i, model.tile_width, model.tile_height, model, toggle);
        draw.ellipse()
            .x_y(0.0, -0.0)
            .radius(model.tile_width/2.0)
            //.color(BLACK);
            .no_fill()
            .stroke_weight(1.0 / scale)
            .stroke(rgba(0.0, 0.0, 0.0, 0.5));
    }
    println!("tot {} |", tot);
    // Write to the window frame.
    draw.to_frame(app, &frame).unwrap();
}

fn draw_poly(app: &App, draw: &Draw, x: f32, y: f32, _index: u32, tile_w: f32, tile_h: f32,  model: &Model, random: u8) {
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
