use nannou::prelude::*;
use nannou::ui::prelude::*;
use ray2d::Ray2D;

const N_WALL: usize = 10;

fn main() {
    nannou::app(model).update(update).run();
}

struct Model {
    walls: Vec<Vector2>,
    draw_gui: bool,
    ui: Ui,
    ids: Ids,
    ray_width: f32,
    wall_width: f32,
    rotation: f32,
}

widget_ids! {
    struct Ids {
        ray_width,
        wall_width,
        rotation,
    }
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
    make_walls(&mut walls, &win);

    let draw_gui = true;

    // Create the UI.
    let mut ui = app.new_ui().build().unwrap();

    // Generate some ids for our widgets.
    let ids = Ids::new(ui.widget_id_generator());

    let ray_width = 6.0;
    let wall_width = 2.0;
    let rotation = 0.0;

    Model {
        walls,
        draw_gui,
        ui,
        ids,
        ray_width,
        wall_width,
        rotation,
    }
}

fn update(_app: &App, model: &mut Model, _update: Update) {
    // Calling `set_widgets` allows us to instantiate some widgets.
    let ui = &mut model.ui.set_widgets();

    fn slider(val: f32, min: f32, max: f32) -> widget::Slider<'static, f32> {
        widget::Slider::new(val, min, max)
            .w_h(200.0, 30.0)
            .label_font_size(15)
            .rgb(0.3, 0.3, 0.3)
            .label_rgb(1.0, 1.0, 1.0)
            .border(0.0)
    }

    for value in slider(model.wall_width as f32, 1.0, 5.0)
        .top_left_with_margin(20.0)
        .label("wall width")
        .set(model.ids.wall_width, ui)
    {
        model.wall_width = value;
    }

    for value in slider(model.ray_width, 1.0, 10.0)
        .down(10.0)
        .label("ray width")
        .set(model.ids.ray_width, ui)
    {
        model.ray_width = value;
    }

    for value in slider(model.rotation, -PI, PI)
        .down(10.0)
        .label("Rotation")
        .set(model.ids.rotation, ui)
    {
        model.rotation = value;
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().color(ORANGERED);

    let mut r = Ray2D::new();
    //r.orig = model.position;
    //r.look_at(app.mouse.x, app.mouse.y);
    r.set_dir_from_angle(model.rotation);
    r.draw(&draw, 60.0, model.ray_width, rgb(0.3, 0.3, 0.3));

    let mut collision: Vector2 = vec2(0.0, 0.0);
    let mut distance: f32 = Float::infinity();
    let mut surface_normal: Vector2 = vec2(0.0, 0.0);

    // find the closest intersection point between the ray and the walls
    for index in (0..N_WALL).step_by(2) {
        draw.line()
            .weight(model.wall_width)
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

    if model.draw_gui {
        model.ui.draw_to_frame(app, &frame).unwrap();
    }
}

fn make_walls(walls: &mut Vec<Vector2>, win: &geom::Rect) {
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

fn key_pressed(app: &App, model: &mut Model, key: Key) {
    match key {
        Key::S => {
            app.main_window()
                .capture_frame(app.time.to_string() + ".png");
            //.capture_frame(app.exe_name().unwrap() + ".png");
        }
        Key::G => model.draw_gui = !model.draw_gui,
        _other_key => {}
    }
}
