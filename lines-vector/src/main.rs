// http://openframeworks.cc:80/ofBook/chapters/lines.html
extern crate nannou;

use nannou::prelude::*;

struct Line {
    start_p: Point2,
    end_p: Point2
}

struct Model {
    points: Vec<Point2>,
    store_points: bool,
    lines: Vec<Line>
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
        .mouse_moved(mouse_moved)
        .key_pressed(key_pressed)
        .build()
        .unwrap();

    Model {
        points: Vec::new(),
        lines: Vec::new(),
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

    for l in model.lines.iter() {
        draw.line()
            .start(l.start_p)
            .end(l.end_p)
            .weight(1.0)
            .color(DARKSLATEBLUE);
        //.stroke_weight(4.)
    }

    draw.to_frame(app, &frame).unwrap();
}

fn mouse_pressed(_app: &App, model: &mut Model, _button: MouseButton) {
    model.store_points = true;
}
fn mouse_released(_app: &App, model: &mut Model, _button: MouseButton) {
    model.store_points = false;
}
fn mouse_moved(app: &App, model: &mut Model, pos: Vector2) {
    if model.store_points {
        for p in model.points.iter_mut() {
            let dist = p.distance(pos);
            if dist < 50.0 {
                let line = Line {
                    start_p: pt2(p.x, p.y),
                    end_p: pos,
                };
                model.lines.push(line);
            }
        }

        match model.points.last() {
            None => {
                model.points.push(pt2(app.mouse.x, app.mouse.y));
            },
            Some(v) => {
                let old = pt2(v.x, v.y);
                let new = pt2(app.mouse.x, app.mouse.y);
                // do not add points that are too close to each other
                if new.distance(old) > 10.0 {
                    model.points.push(new);
                }
            }
        }
    }
    
}

fn key_pressed(_app: &App, model: &mut Model, key: Key) {
    if key == Key::Space {
        model.points.clear();
        model.lines.clear();
    }
}

fn randomize_points(model: &mut Model){
    for p in model.points.iter_mut(){
        p.x = p.x + (random_f32() - 0.5) * 2.0;
        p.y = p.y + (random_f32() - 0.5) * 2.0;
    };
}
