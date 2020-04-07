use nannou::prelude::*;
mod colors;
use crate::colors::Palette;
use nannou::color::gradient::Gradient;
use nannou::geom::graph::edge::Axis::Y;

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
    let win = app.window_rect();
    println!("FPS:{:?}", app.fps());
    let tile_count = map_range(app.mouse.x, win.w()*-1.0, win.w(), 1, 6) as u32;

    let draw = app.draw();
    let draw = draw.color_blend(BLEND_SUBTRACT);
    //let draw = draw.color_blend(BLEND_ADD);
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
        draw_poly(&app, &draw, x, y, i, tile_w, tile_h, model);
    }

    draw.to_frame(app, &frame).unwrap();


}

fn draw_circle(app: &App, draw: &Draw, x: f32, y: f32, index: u32, _tile_w: f32, tile_h: f32, model: &Model) {
    let amp = 10.0;
    let wave = (app.time * 0.7).cos() * amp;

    draw.ellipse()
        .color(model.gradient_one.get((app.time * 0.7).cos().abs()))
        .radius(tile_h / 2.0)
        .x(x + wave * (index % 8) as f32)
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
}

fn draw_poly(app: &App, draw: &Draw, x: f32, y: f32, _index: u32, tile_w: f32, tile_h: f32,  model: &Model) {
    let def = 9;
    let spaces = def -1;
    let parts = spaces + def;
    let gambo_thick = tile_w / 9.0;
    let v_padding = tile_h / parts as f32;
    let t = app.time * 0.2;
    let tot = (parts*2)+2;

    let points = (1..tot).map(|i| {
        let mut y2 = map_range(i, 1, tot, y-tile_h/2.0, y+tile_h/2.0);
        let mut x2 = map_range(i, 1, tot, x-tile_w/2.0, x+tile_w/2.0);
        let p = (i, tot);
        match p {
            (i, total) if i == total => {
                x2 = x + tile_w / 2.0;
                y2 = y + tile_h / 2.0;
            },
            (1, _)  => {
                x2 = x + tile_w / 2.0;
                y2 = y - tile_h / 2.0;
            },
            (i, total) if (i % 4 == 0) | (i % 4 == 1) && i != total => {
                x2 = x + tile_w/2.0 - gambo_thick;
                y2 = y2.floor();
            }
            _ => {},
        };
       // draw.ellipse().x(x-tile_w/2.0 + 20.).y(y+tile_h/2.0).radius(40.).color(RED);
        //draw.ellipse().x(x2).y(y2).radius(10.).color(RED);
        println!("x: {:?}, y: {:?}", x2, y2);
        (pt2(x2, y2), model.gradient_two.get(p.0 as f32 / p.1 as f32))
    });
    println!("============tot: {:?}", tot);


    // let points = (0..def).map(|i| {
    //     let angle = map_range(i, 0, def, 0.0, TAU);
    //
    //     let off_x = angle.sin() * tile_w / 3.0;
    //     let off_y = angle.cos() * tile_h / 3.0;
    //     let fract = 0.5;
    //
    //     let r = (t + fract) % 1.0;
    //     let g = ((t + 1.0 - fract) * angle) % 1.0;
    //     let b = ((t + 0.5 + fract) * angle) % 1.0;
    //
    //     (pt2(x + off_x, y+off_y), palette[0])
    // });

    draw.ellipse().x(x+tile_w/2.0 +20.).y(y-tile_h/2.0).radius(40.).color(STEELBLUE);
    draw.polygon().points_colored(points);
}
