use nannou::prelude::*;
use svg::Svg;

fn main() {
    nannou::app(model).run();
}

struct Model {
    svg: Svg,
}

fn model(app: &App) -> Model {
    app.new_window().size(1200, 800).view(view).build().unwrap();

    let assets = app.assets_path().unwrap();
    let svg_path = assets.join("svgs").join("tiger.svg");
    let svg = Svg::load(svg_path.to_path_buf()).expect("failed to load svg");

    Model { svg }
}

fn view(app: &App, m: &Model, frame: Frame) {
    frame.clear(WHITE);
    let draw = app.draw();

    m.svg.paths.iter().for_each(|p| {
        draw.path()
            .stroke()
            //.x_y(app.mouse.x, app.mouse.y)
            .stroke_weight(1.0)
            .color(BLACK)
            .events(p.events.iter().cloned());

        if let Some(color) = p.fill_color {
            draw.path()
                .fill()
                //.x_y(app.mouse.x, app.mouse.y)
                .color(color)
                .events(p.events.iter().cloned());
        }
        if let Some(ref stroke) = p.stroke_style {
            draw.path()
                .stroke()
                //.x_y(app.mouse.x, app.mouse.y)
                .stroke_weight(stroke.weight)
                .color(stroke.color)
                .join(stroke.line_join)
                .caps(stroke.line_cap)
                .events(p.events.iter().cloned());
        }
    });

    // Write the result of our drawing to the window's frame.
    draw.to_frame(app, &frame).unwrap();
}
