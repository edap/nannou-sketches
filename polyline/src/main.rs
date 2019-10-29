extern crate nannou;

use nannou::prelude::*;

struct Model {
    points: Vec<Point2>,
    store_points: bool,
    random_mag: f32,
}

fn main() {
    nannou::app(model).update(update).run();
}

fn model(app: &App) -> Model {
    let _window = app
        .new_window()
        .with_dimensions(600, 600)
        .view(view)
        .mouse_pressed(mouse_pressed)
        .mouse_released(mouse_released)
        .key_pressed(key_pressed)
        .build()
        .unwrap();

    Model {
        points: Vec::new(),
        store_points: false,
        random_mag: 30.0,
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
                let random = pt2(new.x + random_f32() * model.random_mag, new.y + random_f32() * model.random_mag);
                if new.distance(old) > 0.1 {
                    model.points.push(random);
                }
            },
        }
    }
}

fn view(app: &App, model: &Model, frame: &Frame) {
    let draw = app.draw();
    draw.background().color(RED);

    draw.polyline()
        .join_round()
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
