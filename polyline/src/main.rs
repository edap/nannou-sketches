// http://openframeworks.cc:80/ofBook/chapters/lines.html
extern crate nannou;

use nannou::prelude::*;

struct Model {
    points: Vec<Point2>,
    store_points: bool,
}

fn main() {
    nannou::app(model).update(update).run();
}

fn model(app: &App) -> Model {
    let _window = app
        .new_window()
        .size(600, 600)
        .view(view)
        .mouse_pressed(mouse_pressed)
        .mouse_released(mouse_released)
        .key_pressed(key_pressed)
        .build()
        .unwrap();

    Model {
        points: Vec::new(),
        store_points: false,
    }
}

fn update(app: &App, model: &mut Model, _update: Update) {
    if model.store_points {
        match model.points.last() {
            None => {
                model.points.push(pt2(app.mouse.x, app.mouse.y));
            },
            Some(v) => {
                let old = pt2(v.x, v.y);
                let new = pt2(app.mouse.x, app.mouse.y);
                // avoid to store points that are in the same position
                if new.distance(old) > 1.0 {
                    model.points.push(new);
                }
            },
        }
    }
    randomize_points(model);
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().color(ORANGERED);

    draw.polyline()
        .join_round()
        .color(DARKSLATEBLUE)
        .stroke_weight(4.)
        .points(model.points.iter().cloned());

    draw.to_frame(app, &frame).unwrap();
}

fn mouse_pressed(_app: &App, model: &mut Model, _button: MouseButton) {
    model.store_points = true;
}
fn mouse_released(_app: &App, model: &mut Model, _button: MouseButton) {
    model.store_points = false;
}

fn key_pressed(_app: &App, model: &mut Model, key: Key) {
    if key == Key::Space {
        model.points.clear();
    }
}

fn randomize_points(model: &mut Model){
    for p in model.points.iter_mut(){
        p.x = p.x + (random_f32() - 0.5) * 2.0;
        p.y = p.y + (random_f32() - 0.5) * 2.0;
    };
}
