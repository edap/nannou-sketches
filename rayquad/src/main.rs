use edapx_colors::Palette;
use nannou::color::gradient::Gradient;
use nannou::prelude::*;
use nannou::rand::rngs::StdRng;
use nannou::rand::{Rng, SeedableRng};
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
    color: Rgb,
    position: Point2,
    scheme_id: usize,
    palette: Palette,
    gradient_one: Gradient<Hsl>,
    gradient_two: Gradient<Hsl>,
    gradient_three: Gradient<Hsl>,
    blend_id: usize,
    act_random_seed: u64,
}

widget_ids! {
    struct Ids {
        ray_width,
        wall_width,
        rotation,
        random_color,
        position,

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
    let position = pt2(0.0, 0.0);
    let color = rgb(1.0, 0.0, 1.0);
    let scheme_id = 0;
    let palette = Palette::new();
    let scheme = palette.get_scheme(scheme_id);
    let gradient_one = Gradient::new(vec![Hsl::from(scheme[0]), Hsl::from(scheme[2])]);
    let gradient_two = Gradient::new(vec![Hsl::from(scheme[1]), Hsl::from(scheme[3])]);
    let gradient_three = Gradient::new(vec![Hsl::from(scheme[4]), Hsl::from(scheme[1])]);

    Model {
        walls,
        draw_gui,
        ui,
        ids,
        ray_width,
        wall_width,
        rotation,
        position,
        color,
        scheme_id,
        palette,
        gradient_one,
        gradient_two,
        gradient_three,
        blend_id: 0,
        act_random_seed: 0,
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

    for _click in widget::Button::new()
        .down(10.0)
        .w_h(200.0, 60.0)
        .label("Random Color")
        .label_font_size(15)
        .rgb(0.3, 0.3, 0.3)
        .label_rgb(1.0, 1.0, 1.0)
        .border(0.0)
        .set(model.ids.random_color, ui)
    {
        model.color = rgb(random(), random(), random());
    }

    for (x, y) in widget::XYPad::new(
        model.position.x,
        -200.0,
        200.0,
        model.position.y,
        -200.0,
        200.0,
    )
    .down(10.0)
    .w_h(200.0, 200.0)
    .label("Position")
    .label_font_size(15)
    .rgb(0.3, 0.3, 0.3)
    .label_rgb(1.0, 1.0, 1.0)
    .border(0.0)
    .set(model.ids.position, ui)
    {
        model.position = Point2::new(x, y);
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    let win = app.window_rect();
    let tile_count_w = map_range(app.mouse.x, win.w() * -1.0, win.w(), 1, 8) as u32;
    let tile_count_h = (win.h() * 2.0).abs() as u32 / tile_count_w;
    let mut rng = StdRng::seed_from_u64(model.act_random_seed);

    let blends = [BLEND_NORMAL, BLEND_ADD, BLEND_SUBTRACT, BLEND_LIGHTEST];
    let draw = app.draw().color_blend(blends[model.blend_id].clone());
    frame.clear(model.palette.get_scheme(model.scheme_id)[4]);

    let tot = tile_count_w * tile_count_h;

    // for i in 0..tot {
    //     let tile_size = win.w() / tile_count_w as f32;
    //     let x = (i % tile_count_w) as f32 * tile_size - win.w() * 0.5 + tile_size / 2.0;
    //     let y = (i / tile_count_w) as f32 * tile_size - win.h() * 0.5 + tile_size / 2.0;
    //     let mut draw = draw.x_y(x, y);
    //     let toggle = rng.gen_range(0, 2);
    //     let rotation = match toggle {
    //         0 => -PI,
    //         1 => 0.0,
    //         _ => unreachable!(),
    //     };
    //     draw = draw.rotate(rotation);
    //     draw.ellipse()
    //         .x_y(0.0, 0.0)
    //         .radius(tile_size / 2.0)
    //         //.color(BLACK);
    //         .no_fill()
    //         .stroke_weight(3.0)
    //         .stroke(rgba(0.0, 0.0, 0.0, 0.5));
    // }

    let mut r = Ray2D::new();
    //r.look_at(app.mouse.x, app.mouse.y);
    r.set_dir_from_angle(model.rotation);
    r.draw(&draw, 200.0, model.ray_width, rgb(0.3, 0.3, 0.3));

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
        Key::Key1 => {
            model.scheme_id = 0;
        }
        Key::Key2 => {
            model.scheme_id = 1;
        }
        Key::Key3 => {
            model.scheme_id = 2;
        }
        Key::Key4 => {
            model.scheme_id = 3;
        }
        Key::Key5 => {
            model.scheme_id = 4;
        }
        Key::Key6 => {
            model.scheme_id = 5;
        }
        Key::Q => {
            model.blend_id = 0;
        }
        Key::W => {
            model.blend_id = 1;
        }
        Key::E => {
            model.blend_id = 2;
        }
        Key::R => {
            model.blend_id = 3;
        }
        Key::S => {
            app.main_window()
                .capture_frame(app.time.to_string() + ".png");
            //.capture_frame(app.exe_name().unwrap() + ".png");
        }
        Key::G => model.draw_gui = !model.draw_gui,
        _other_key => {}
    }
}
