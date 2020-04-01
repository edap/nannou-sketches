use nannou::prelude::*;

fn main() {
    nannou::sketch(view).run();
    //nannou::app(model).update(update).run();
}

fn view(app: &App, frame: Frame) {
    let def = 3;
    let draw = app.draw();
    let draw = draw.color_blend(BLEND_ADD);
    let win = app.window_rect();
    let tile_count = 12;
    frame.clear(PURPLE);
    // TODO decouple def from tot
    // Add Palette
    // Work on single tile
    // Add mouse interaction
    // Add gui to scale n tile
    // Add random
    let tot = tile_count * tile_count * def;

    for i in 0..tot {
        let tile_w = win.w() / tile_count as f32;
        let tile_h = win.h() / tile_count as f32;
        let cell = i / def;
        let x = (cell % tile_count) as f32 * tile_w - win.w() * 0.5 + tile_w / 2.0;
        let y = (cell / tile_count) as f32 * tile_h - win.h() * 0.5 + tile_h / 2.0;


        draw_circle(&app, &draw, x, y, i);
        draw_poly(&app, &draw, x, y, i);
    }

    draw.to_frame(app, &frame).unwrap();
}

fn draw_circle(app: &App, draw: &Draw, x: f32, y: f32, index: u32) {
    let radius = 25.0;
    let amp = 10.0;
    let wave = (app.time * 0.1).cos() * amp;

    draw.ellipse()
        .color(RED)
        .radius(radius)
        .x(x + wave * (index % 8) as f32)
        .y(y);
}

fn draw_poly(app: &App, draw: &Draw, x: f32, y: f32, _index: u32) {
    let def = 4;
    let t = app.time * 0.2;

    let points = (0..def).map(|i| {
        let tile_w = 40.0;
        let tile_h = 60.0;

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
