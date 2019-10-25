extern crate nannou;

use nannou::prelude::*;

struct Model {}

fn main() {
    nannou::app(model)
        .simple_window(view)
        .run();
}

fn model(_app: &App) -> Model {
    Model{}
}

fn view(app: &App, _model: &Model, frame: &Frame) {
    let draw = app.draw();


    // Write to the window frame.
    draw.to_frame(app, &frame).unwrap();
}

