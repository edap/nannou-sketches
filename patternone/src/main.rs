use nannou::prelude::*;

fn main() {
    nannou::sketch(view).run();
    //nannou::app(model).update(update).run();
}

fn view(app: &App, frame: Frame) {
    let def = 3; // points per tile
    let draw = app.draw();
    let draw = draw.color_blend(BLEND_ADD);
    let win = app.window_rect();
    let tile_count = 12;
    let t = app.time;
    frame.clear(PURPLE);
    let tot = tile_count * tile_count * def;

    let points = (0..tot).map(|i| {
        let tile_w = win.w() / tile_count as f32;
        let tile_h = win.h() / tile_count as f32;
        let cell = i / def;
        let x = (cell % tile_count) as f32 * tile_w - win.w() * 0.5 + tile_w / 2.0;
        let y = (cell / tile_count) as f32 * tile_h - win.h() * 0.5 + tile_h / 2.0;
        let fract = cell as f32 / tot as f32;

        let cell_pt_idx = (i % def) as f32;
        let angle = map_range(cell_pt_idx, 0.0, def as f32, 0.0, TAU);
        let off_x = angle.sin() * tile_w / 3.0;
        let off_y = angle.cos() * tile_h / 3.0;
        let r = (t + fract) % 1.0;
        let g = ((t + 1.0 - fract) * angle) % 1.0;
        let b = ((t + 0.5 + fract) * angle) % 1.0;
        //pt2(x+off_x, y+off_y)
        (pt2(x + off_x, y+off_y), rgb(r, g, b))
    });
    draw.polygon().points_colored(points);
    //draw.polygon().points(points);


    for i in 0..tot {
        let tile_w = win.w() / tile_count as f32;
        let tile_h = win.h() / tile_count as f32;
        let cell = i / def;
        let x = (cell % tile_count) as f32 * tile_w - win.w() * 0.5 + tile_w / 2.0;
        let y = (cell / tile_count) as f32 * tile_h - win.h() * 0.5 + tile_h / 2.0;

        let cell_pt_idx = (i % def) as f32;
        let angle = map_range(cell_pt_idx, 0.0, def as f32, 0.0, TAU);

        draw.ellipse()
            .color(RED)
            .radius(5.)
            .no_fill()
            .x(x)
            .y(y);

        draw.ellipse()
            .color(YELLOW)
            .radius(4.)
            .x(x+ angle.sin() * tile_h / 3.)
            .y(y+ angle.cos() * tile_h / 3.);
    }

    draw.to_frame(app, &frame).unwrap();
}
