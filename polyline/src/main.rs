extern crate nannou;

use nannou::prelude::*;

struct Model {
    points: Vec<Point2>,
}

fn main() {
    nannou::app(model).update(update).run();
}

// this fn initialize the model. is the setup() in OF and processing
fn model(app: &App) -> Model {
    // it also create the window
    let _window = app
        .new_window()
        .with_dimensions(600, 600)
        .view(view)
        //.mouse_pressed(mouse_pressed)
        //.key_released(key_released)
        .build()
        .unwrap();
    // my points
    let mut points = Vec::new();
    let p1 = pt2(100.0, 100.0);
    let p2 = pt2(500.0, 200.0);
    points.push(p1);
    points.push(p2);
    Model{ points }
}

// Update the state of your application here. By default, this gets called right before `view`.
fn update(_app: &App, _model: &mut Model, _update: Update) {
    // here my logic should happen
}

fn view(app: &App, model: &Model, frame: &Frame) {
    let draw = app.draw();

    // test
    let mut points = Vec::new();
    let p1 = pt2(100.0, 100.0);
    let p2 = pt2(500.0, 200.0);
    points.push(p1);
    points.push(p2);
    draw.background().color(RED);
    //


    draw.polyline()
    .join_round()
    .points(points); // ok
    //.points(&model.points); //not ok


    // Write to the window frame.
    draw.to_frame(app, &frame).unwrap();
}

