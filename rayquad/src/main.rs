use nannou::prelude::*;
use ray2d::Ray2D;

const N_WALL: usize = 10;

fn main() {
    nannou::app(model).run();
}

struct Model {
    walls: Vec<Vector2>,
}

fn model(app: &App) -> Model {
    app.new_window()
        .size(800, 800)
        .view(view)
        .key_pressed(key_pressed)
        .build()
        .unwrap();

    let mut walls: Vec<Vector2> = Vec::new();
    let win = app.window_rect();
    println!("{}", win.w());

    make_walls(&mut walls, &win);

    // let test = vec2(2.0, 2.0);
    // let testa = vec2(12.0, 12.0);

    Model { walls }
}

fn make_walls(walls: &mut Vec<Vector2>, win: &Rect) {
    while walls.len() < N_WALL * 2 {
        let start_p = vec2(
            random_range(-win.w() / 2.0, win.w() / 2.0),
            random_range(-win.h() / 2.0, win.h() / 2.0),
        );
        let end_p = vec2(
            random_range(-win.w() / 2.0, win.w() / 2.0),
            random_range(-win.h() / 2.0, win.h() / 2.0),
        );
        walls.push(start_p);
        walls.push(end_p);
    }
}

fn key_pressed(app: &App, _model: &mut Model, key: Key) {
    match key {
        Key::S => {
            app.main_window()
                .capture_frame(app.time.to_string() + ".png");
            //.capture_frame(app.exe_name().unwrap() + ".png");
        }
        _other_key => {}
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().color(PLUM);

    let mut r = Ray2D::new();
    r.look_at(app.mouse.x, app.mouse.y);
    r.draw(&draw, 200.0, 3.0, rgb(0.3, 0.3, 0.3));

    let mut collision: Vector2 = vec2(0.0, 0.0);
    let mut distance: f32 = Float::infinity();
    let mut surface_normal: Vector2 = vec2(0.0, 0.0);

    // find the closest intersection point between the ray and the walls
    for index in (0..N_WALL).step_by(2) {
        draw.line()
            .color(STEELBLUE)
            .start(model.walls[index])
            .caps_round()
            .end(model.walls[index + 1]);

        if let Some(collision_distance) = r.intersect_segment(
            model.walls[index].x,
            model.walls[index].y,
            model.walls[index + 1].x,
            model.walls[index + 1].y,
        ) {
            if collision_distance < distance {
                distance = collision_distance;
                collision = r.orig + r.dir.with_magnitude(collision_distance);
                let segment_dir = (model.walls[index] - model.walls[index + 1]).normalize();
                surface_normal = vec2(segment_dir.y, -segment_dir.x);
            }
        }
    }

    if distance < Float::infinity() {
        // collision point
        draw.ellipse()
            .color(GREEN)
            .x_y(collision.x, collision.y)
            .w_h(10.0, 10.0);

        // reflection
        let refl = r.reflect(surface_normal);
        draw.line()
            .color(YELLOW)
            .start(collision)
            .caps_round()
            .end(collision + refl.with_magnitude(100.0));

        // refraction
        let refr = r.refract(surface_normal, 1.2);
        draw.line()
            .color(INDIGO)
            .start(collision)
            .caps_round()
            .end(collision + refr.with_magnitude(100.0));
    };

    draw.to_frame(app, &frame).unwrap();
}
