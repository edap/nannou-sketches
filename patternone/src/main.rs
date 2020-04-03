use nannou::prelude::*;
mod colors;
use crate::colors::Palette;
use nannou::color::gradient::Gradient;

struct Model {
    scheme_id: usize,
    palette: Palette,
    gradient_one: Gradient<Hsl>,
    gradient_two: Gradient<Hsl>,
    gradient_three: Gradient<Hsl>,
}

fn main() {
    //nannou::sketch(view).run();
    nannou::app(model).run();
}

fn model(app: &App) -> Model {
    app.new_window()
    //.size(720, 720)
    .key_pressed(key_pressed)
    .view(view)
    .build()
    .unwrap();

    let scheme_id = 0;
    let palette = Palette::new();
    let scheme = palette.get_scheme(scheme_id);

    let g_one = Gradient::new(vec![Hsl::from(scheme[0]),Hsl::from(scheme[4])]);
    let g_two = Gradient::new(vec![Hsl::from(scheme[1]),Hsl::from(scheme[2])]);
    let g_three = Gradient::new(vec![Hsl::from(scheme[1]),Hsl::from(scheme[3])]);

    Model {
        scheme_id: scheme_id,
        palette,
        gradient_one: g_one,
        gradient_two: g_two,
        gradient_three: g_three,
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
        Key::S => {
            app.main_window()
                .capture_frame(app.exe_name().unwrap() + ".png");
        }
        _other_key => {}
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    let win = app.window_rect();
    let tile_count = map_range(app.mouse.x, win.w()*-1.0, win.w(), 2, 14) as u32;

    let draw = app.draw();
    //let draw = draw.color_blend(BLEND_SUBTRACT);
    let draw = draw.color_blend(BLEND_ADD);
    frame.clear(model.palette.get_scheme(model.scheme_id)[4]);

    // TODO Work on single tile
    // Add mouse interaction
    // Add gui to scale n tile
    // Add random
    let tot = tile_count * tile_count;

    for i in 0..tot {
        let tile_w = win.w() / tile_count as f32;
        let tile_h = win.h() / tile_count as f32;
        let x = (i % tile_count) as f32 * tile_w - win.w() * 0.5 + tile_w / 2.0;
        let y = (i / tile_count) as f32 * tile_h - win.h() * 0.5 + tile_h / 2.0;
        draw_circle(&app, &draw, x, y, i, tile_w, tile_h, model);
        draw_poly(&app, &draw, x, y, i, tile_w, tile_h, model.palette.get_scheme(model.scheme_id));
    }

    draw.to_frame(app, &frame).unwrap();
}

fn draw_circle(app: &App, draw: &Draw, x: f32, y: f32, index: u32, _tile_w: f32, tile_h: f32, model: &Model) {
    let palette = model.palette.get_scheme(model.scheme_id);
    let amp = 10.0;
    let wave = (app.time * 0.7).cos() * amp;

    draw.ellipse()
        .color(model.gradient_two.get((app.time * 0.7).cos().abs()))
        .radius(tile_h / 2.0)
        .x(x + wave * (index % 8) as f32)
        .y(y);

    // shadow
    let n_circles = 12;
    let circle_thickness = 4.0;
    for n in(1..n_circles) {
        let alpha = map_range(n, 1, n_circles, 1.0, 0.0);
        let offset = map_range(n,1, n_circles, circle_thickness, ((circle_thickness/2.0) * n_circles as f32));
        draw.ellipse()
            .resolution(64)
            .no_fill()
            .stroke_weight(circle_thickness)
            .stroke(model.gradient_one.get(alpha))
            .x(x + wave * (index % 8) as f32)
            .y(y)
            .radius(tile_h / 2.0 + offset - circle_thickness);
    }
}

fn draw_poly(app: &App, draw: &Draw, x: f32, y: f32, _index: u32, tile_w: f32, tile_h: f32, palette: &[Rgb]) {
    let def = 3;
    let t = app.time * 0.2;

    let points = (0..def).map(|i| {
        let angle = map_range(i, 0, def, 0.0, TAU);

        let off_x = angle.sin() * tile_w / 3.0;
        let off_y = angle.cos() * tile_h / 3.0;
        let fract = 0.5;

        let r = (t + fract) % 1.0;
        let g = ((t + 1.0 - fract) * angle) % 1.0;
        let b = ((t + 0.5 + fract) * angle) % 1.0;

        (pt2(x + off_x, y+off_y), rgb(r, g, b))
    });

    draw.polygon().points_colored(points);
}
