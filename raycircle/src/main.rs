use edapx_colors::Palette;
use nannou::prelude::*;
use nannou::ui::prelude::*;

mod bouncing;
pub use crate::bouncing::BouncingRay2D;

fn main() {
    nannou::app(model).update(update).run();
}

struct Circle {
    pos: Vector2,
    radius: f32,
}

struct Model {
    walls: Vec<Circle>,
    rays: Vec<BouncingRay2D>,
    draw_gui: bool,
    ui: Ui,
    ids: Ids,
    ray_width: f32,
    wall_width: f32,
    collision_radius: f32,
    rotation: f32,
    scheme_id: usize,
    blend_id: usize,
    palette: Palette,
    show_walls: bool,
    animation: bool,
    animation_speed: f32,
    draw_refl: bool,
    draw_polygon: bool,
}

widget_ids! {
    struct Ids {
        wall_width,
        ray_width,
        collision_radius,
        rotation,
        scheme_id,
        blend_id,
        draw_refl,
        draw_polygon,
        animation,
        animation_speed,
        show_walls
    }
}

fn model(app: &App) -> Model {
    let tile_count_w = 2;
    app.new_window()
        .size(800, 800)
        .view(view)
        .key_pressed(key_pressed)
        .build()
        .unwrap();

    let mut walls: Vec<Circle> = Vec::new();
    let mut rays: Vec<BouncingRay2D> = Vec::new();
    let win = app.window_rect();

    let draw_gui = true;

    // Create the UI.
    let mut ui = app.new_ui().build().unwrap();

    // Generate some ids for our widgets.
    let ids = Ids::new(ui.widget_id_generator());

    let ray_width = 2.0;
    let wall_width = 2.0;
    let rotation = 0.0;
    let collision_radius = 3.0;

    let scheme_id = 5;
    let blend_id = 2;
    let palette = Palette::new();
    make_circles(&mut walls, &mut rays, &win, tile_count_w, 4);
    let show_walls = true;
    let animation = false;
    let animation_speed = 0.01;
    let draw_refl = true;
    let draw_polygon = false;

    Model {
        walls,
        rays,
        draw_gui,
        ui,
        ids,
        wall_width,
        collision_radius,
        ray_width,
        rotation,
        scheme_id,
        blend_id,
        palette,
        show_walls,
        animation,
        animation_speed,
        draw_refl,
        draw_polygon,
    }
}

fn update(_app: &App, model: &mut Model, _update: Update) {
    // Calling `set_widgets` allows us to instantiate some widgets.
    let ui = &mut model.ui.set_widgets();
    {
        fn slider(val: f32, min: f32, max: f32) -> widget::Slider<'static, f32> {
            widget::Slider::new(val, min, max)
                .w_h(200.0, 30.0)
                .label_font_size(15)
                .rgb(0.3, 0.3, 0.3)
                .label_rgb(1.0, 1.0, 1.0)
                .border(0.0)
        }

        fn toggle(val: bool) -> widget::Toggle<'static> {
            widget::Toggle::new(val)
                .w_h(200.0, 30.0)
                .label_font_size(15)
                .rgb(0.3, 0.3, 0.3)
                .label_rgb(1.0, 1.0, 1.0)
                .border(0.0)
        }

        for value in slider(model.wall_width as f32, 1.0, 15.0)
            .top_left_with_margin(20.0)
            .label("wall width")
            .set(model.ids.wall_width, ui)
        {
            model.wall_width = value;
        }

        for value in slider(model.collision_radius as f32, 5.0, 45.0)
            .down(10.0)
            .label("collision radius")
            .set(model.ids.collision_radius, ui)
        {
            model.collision_radius = value;
        }

        for value in slider(model.ray_width, 1.0, 10.0)
            .down(10.0)
            .label("ray width")
            .set(model.ids.ray_width, ui)
        {
            model.ray_width = value;
        }

        for val in slider(model.rotation, -PI, PI)
            .down(20.0)
            .label("Rotation")
            .set(model.ids.rotation, ui)
        {
            model.rotation = val;
        }

        for value in slider(model.scheme_id as f32, 0.0, 5.0)
            .down(30.0)
            .label("scheme_id")
            .set(model.ids.scheme_id, ui)
        {
            model.scheme_id = value as usize;
        }

        for value in slider(model.blend_id as f32, 0.0, 3.0)
            .down(30.0)
            .label("blend_id")
            .set(model.ids.blend_id, ui)
        {
            model.blend_id = value as usize;
        }

        for v in toggle(model.draw_refl as bool)
            .label("Draw reflection")
            .set(model.ids.draw_refl, ui)
        {
            model.draw_refl = v;
        }

        for v in toggle(model.draw_polygon as bool)
            .label("Draw poly")
            .set(model.ids.draw_polygon, ui)
        {
            model.draw_polygon = v;
        }

        for v in toggle(model.animation as bool)
            .label("Animation")
            .set(model.ids.animation, ui)
        {
            model.animation = v;
        }

        for value in slider(model.animation_speed as f32, 1.0, 0.1)
            .down(30.0)
            .label("animation speed")
            .set(model.ids.animation_speed, ui)
        {
            model.animation_speed = value;
        }

        for v in toggle(model.show_walls as bool)
            .label("Show wall")
            .set(model.ids.show_walls, ui)
        {
            model.show_walls = v;
        }
    }

    for r in model.rays.iter_mut() {
        r.collisions.clear();
        r.reflections.clear();
        r.refl_intensity.clear();
        if model.animation {
            r.primary_ray.orig.x = (_app.time * 0.1).sin() * 300.0 * model.animation_speed;
        }

        // this two are not necessary but add a line more from the ray to the destination
        // r.collisions.push(r.ray.orig);
        // r.reflections.push(r.ray.dir);
        // r.refl_intensity.push(0.0);

        while !r.max_bounces_reached() {
            //println!("get {:?}", r.max_bounces);
            let mut collision: Vector2 = vec2(0.0, 0.0);
            let mut distance: f32 = Float::infinity();
            let mut surface_normal: Vector2 = vec2(0.0, 0.0);
            // find the closest intersection point between the ray and the walls
            for c in &model.walls {
                if let Some(collision_distance) = r.ray.intersect_circle(c.pos, c.radius) {
                    if collision_distance < distance {
                        distance = collision_distance;
                        collision = r.ray.orig + r.ray.dir.with_magnitude(collision_distance);
                        surface_normal = (collision - c.pos).normalize();
                    }
                }
            }
            if distance < Float::infinity() {
                // collision point
                r.bounces += 1;
                let refl = r.ray.reflect(surface_normal);
                r.refl_intensity.push(r.ray.dir.dot(refl).abs());
                r.ray.orig = collision + refl.with_magnitude(0.03);
                r.ray.dir = refl;
                r.collisions.push(collision);
                //r.refractions.push(r.ray.refract(surface_normal, 1.0));
                r.reflections.push(refl);
            } else {
                break;
            };
        }
        r.reset();
        // println!("{:?}", r.bounces);
        //println!("RESE");
        r.ray.set_dir_from_angle(model.rotation);
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    let blends = [BLEND_NORMAL, BLEND_ADD, BLEND_SUBTRACT, BLEND_LIGHTEST];
    let draw = app.draw().color_blend(blends[model.blend_id].clone());
    frame.clear(model.palette.get_scheme(model.scheme_id)[4]);
    //let draw = app.draw();
    draw.background()
        .color(model.palette.get_scheme(model.scheme_id)[4]);

    // draw the walls
    if model.show_walls {
        for c in &model.walls {
            draw.ellipse()
                .x_y(c.pos.x, c.pos.y)
                .w_h(c.radius * 2.0, c.radius * 2.0)
                .color(model.palette.get_scheme(model.scheme_id)[1]);
        }
    }

    for r in &model.rays {
        draw.arrow()
            .color(model.palette.get_scheme(model.scheme_id)[0])
            .start(r.ray.orig)
            .weight(model.ray_width * 2.0)
            .end(r.ray.orig + r.ray.dir.with_magnitude(20.0));
        for (&c, &i) in r.collisions.iter().zip(r.refl_intensity.iter()) {
            draw.ellipse()
                .no_fill()
                .stroke(model.palette.get_scheme(model.scheme_id)[2])
                .stroke_weight(3.0)
                .x_y(c.x, c.y)
                .w_h(model.collision_radius * i, model.collision_radius * i);
        }

        let mut col = rgba(0.0, 0.0, 0.0, 0.0);
        let ppp = r
            .collisions
            .iter()
            .zip(r.reflections.iter())
            .map(|(&co, &re)| {
                if re.x > 0.0 {
                    col = model.palette.get_scheme(model.scheme_id)[2]
                } else {
                    col = model.palette.get_scheme(model.scheme_id)[3]
                }
                (pt2(co.x, co.y), col)
            });

        if model.draw_polygon {
            draw.polygon().points_colored(ppp);
        };

        draw.path()
            .stroke()
            .stroke_weight(model.ray_width)
            .caps_round()
            .points(r.collisions.iter().cloned())
            .color(model.palette.get_scheme(model.scheme_id)[0]);

        for (&c, &r) in r.collisions.iter().zip(r.reflections.iter()) {
            draw.arrow()
                .start(c)
                .end(c + r.with_magnitude(20.0))
                .stroke_weight(model.ray_width)
                .color(model.palette.get_scheme(model.scheme_id)[4]);
        }
    }

    draw.to_frame(app, &frame).unwrap();

    if model.draw_gui {
        model.ui.draw_to_frame(app, &frame).unwrap();
    }
}

fn make_circles(
    walls: &mut Vec<Circle>,
    rays: &mut Vec<BouncingRay2D>,
    win: &geom::Rect,
    tile_count_w: u32,
    mode: u8, // 0 even, 1 random rotation, 2 one in the middle, 4 diamond
) {
    let side = win.w() as u32 / tile_count_w;
    let mut xpos = win.left();
    let mut ypos = win.bottom();

    for _x in 0..tile_count_w {
        for _y in 0..(win.h() as u32 / side as u32) {
            let coin = random_range(0.0, 1.0);
            let start_p;
            let end_p;
            let padding = 0.1 * side as f32;

            match mode {
                4 => {
                    if _x % 2 == 0 && _y % 2 == 0 {
                        start_p = vec2(xpos + padding, ypos + side as f32 - padding);
                        end_p = vec2(xpos + side as f32 - padding, ypos + padding);
                    // let mut r = BouncingRay2D::new();
                    // //r.primary_ray.dir = Vector2::from_angle(random_range(-PI, PI));
                    // r.primary_ray.dir = Vector2::from_angle(1.0);
                    // // r.primary_ray.orig = start_p;
                    // // r.ray.orig = start_p;
                    // let o = vec2(xpos + side as f32 / 2.0, ypos + side as f32 - padding);
                    // r.primary_ray.orig = o;
                    // r.ray.orig = o;
                    // if coin > 0.6 {
                    //     rays.push(r);
                    // }
                    } else if _y % 2 == 0 && _x % 2 != 0 {
                        start_p = vec2(xpos + padding, ypos + padding);
                        end_p = vec2(xpos + side as f32 - padding, ypos + side as f32 - padding);
                    } else if _x % 2 != 0 && _y % 2 != 0 {
                        start_p = vec2(xpos + padding, ypos + side as f32 - padding);
                        end_p = vec2(xpos + side as f32 - padding, ypos + padding);
                    } else {
                        start_p = vec2(xpos + padding, ypos + padding);
                        end_p = vec2(xpos + side as f32 - padding, ypos + side as f32 - padding);
                    }
                    // walls.push(Circle {
                    //     pos: start_p,
                    //     radius: 50.0,
                    // });

                    //}
                }
                _ => {}
            }

            ypos += side as f32;
        }
        ypos = win.bottom();
        xpos += side as f32;
    }
    walls.push(Circle {
        pos: vec2(0.0, 0.0),
        radius: 150.0,
    });
    let mut r = BouncingRay2D::new();
    //r.primary_ray.dir = Vector2::from_angle(random_range(-PI, PI));
    r.primary_ray.dir = Vector2::from_angle(1.0);
    // r.primary_ray.orig = start_p;
    // r.ray.orig = start_p;
    let o = vec2(10.0, 20.0);
    //let o = vec2(300.0, 20.0);
    r.primary_ray.orig = o;
    r.ray.orig = o;
    //if coin > 0.6 {
    println!("get {:?}", r);
    rays.push(r);
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
