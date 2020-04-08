use nannou::prelude::*;
mod colors;
use crate::colors::Palette;
use nannou::color::gradient::Gradient;
use nannou::rand::rngs::StdRng;
use nannou::rand::{Rng, SeedableRng};

struct Model {
    scheme_id: usize,
    palette: Palette,
    gradient_one: Gradient<Hsl>,
    gradient_two: Gradient<Hsl>,
    gradient_three: Gradient<Hsl>,
    blend_id: usize,
    act_random_seed: u64,
}

fn main() {
    //nannou::sketch(view).run();
    nannou::app(model).run();
}

fn model(app: &App) -> Model {
    app.new_window()
        .key_pressed(key_pressed)
        .mouse_pressed(mouse_pressed)
        .view(view)
        .build()
        .unwrap();

    let scheme_id = 0;
    let palette = Palette::new();
    let scheme = palette.get_scheme(scheme_id);

    let scheme_id_a = (scheme_id + 1) % 5;
    let scheme_id_b = (scheme_id + 4) % 5;
    let scheme_id_c = (scheme_id + 2) % 5;
    let scheme_id_d = (scheme_id + 3) % 5;

    let gradient_one = Gradient::new(vec![Hsl::from(scheme[scheme_id]),Hsl::from(scheme[scheme_id_b])]);
    let gradient_two = Gradient::new(vec![Hsl::from(scheme[scheme_id_a]),Hsl::from(scheme[scheme_id_c])]);
    let gradient_three = Gradient::new(vec![Hsl::from(scheme[scheme_id_a]),Hsl::from(scheme[scheme_id_d])]);

    Model {
        scheme_id,
        palette,
        gradient_one,
        gradient_two,
        gradient_three,
        blend_id: 0,
        act_random_seed: 0
    }
}
fn mouse_pressed(_app: &App, model: &mut Model, _button: MouseButton) {
    model.act_random_seed = (random_f32() * 100000.0) as u64;
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
    let scheme_id_a = (scheme_id + 1) % 5;
    let scheme_id_b = (scheme_id + 2) % 5;
    let scheme_id_c = (scheme_id + 3) % 5;
    let scheme_id_d = (scheme_id + 4) % 5;
    let scheme = model.palette.get_scheme(scheme_id);
    model.gradient_one = Gradient::new(vec![Hsl::from(scheme[scheme_id_a]),Hsl::from(scheme[scheme_id_b])]);
    model.gradient_two = Gradient::new(vec![Hsl::from(scheme[scheme_id_c]),Hsl::from(scheme[scheme_id_d])]);
    model.gradient_three = Gradient::new(vec![Hsl::from(scheme[scheme_id_d]),Hsl::from(scheme[scheme_id])]);
}

fn view(app: &App, model: &Model, frame: Frame) {
    let mut rng = StdRng::seed_from_u64(model.act_random_seed);
    let win = app.window_rect();
    let tile_count = map_range(app.mouse.x, win.w()*-1.0, win.w(), 1, 8) as u32;

    let blends = [
        BLEND_NORMAL,
        BLEND_ADD,
        BLEND_SUBTRACT,
        BLEND_LIGHTEST,
    ];
    let mut draw = app.draw().color_blend(blends[model.blend_id].clone());
    frame.clear(model.palette.get_scheme(model.scheme_id)[4]);


    // Add gui to scale n tile
    // Add random
    let tot = tile_count * tile_count;

    for i in 0..tot {
        let tile_w = win.w() / tile_count as f32;
        let tile_h = win.h() / tile_count as f32;
        let x = (i % tile_count) as f32 * tile_w - win.w() * 0.5 + tile_w / 2.0;
        let y = (i / tile_count) as f32 * tile_h - win.h() * 0.5 + tile_h / 2.0;
        let coin = rng.gen_range(0, 4);
        draw_circles(&app, &draw, x, y, i, tile_w, tile_h, model, coin);
        draw_poly(&app, &draw, x, y, i, tile_w, tile_h, model, coin);

        let rotation = match coin {
            0 => -(PI / 2.0),
            1 => 0.0,
            2 => PI / 2.0,
            3 => PI,
            _ => unreachable!(),
        };
        draw = draw.rotate(rotation);
    }
    draw.to_frame(app, &frame).unwrap();
}

fn draw_circles(app: &App, draw: &Draw, x: f32, y: f32, index: u32, tile_w: f32, tile_h: f32, model: &Model, random: u8) {
    let amp = 10.0;
    let wave = (app.time * 0.7).cos() * amp;

    draw.ellipse()
        .color(model.gradient_one.get(0.5 + (app.time * 0.4).cos() * 0.5))
        .radius(tile_h / 2.0)
        .x(x + wave * (index % 3) as f32)
        .y(y);
    // shadow
    let n_circles = 8;
    //let circle_thickness = 4.0;
    let circle_thickness = app.mouse.y / 10.;
    for n in 1..n_circles {
        let alpha = map_range(n, 1, n_circles, 1.0, 0.0);
        let offset = map_range(n,1, n_circles, circle_thickness, circle_thickness/2.0 * n_circles as f32);
        draw.ellipse()
            .resolution(64)
            .no_fill()
            .stroke_weight(circle_thickness)
            .stroke(model.gradient_two.get(alpha))
            .x(x + wave * (index % 8) as f32)
            .y(y)
            .radius(tile_h / 2.0 + offset - circle_thickness);
    }

    let rad = tile_h/8.0 - app.mouse.y / 14.;
    draw.ellipse()
        .x(x-tile_w/2.0 + rad * 1.2)
        .y((y+tile_h/2.0 - rad * 1.1) - wave * (index % 5) as f32)
        .radius(rad)
        .color(model.palette.get_scheme(model.scheme_id)[0])
        .stroke_weight(3.)
        .stroke_color(model.palette.get_scheme(model.scheme_id)[1]);
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
