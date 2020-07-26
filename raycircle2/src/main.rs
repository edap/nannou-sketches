use edapx_colors::Palette;
use nannou::prelude::*;
use nannou::ui::prelude::*;

use ray2d::Ray2D;

fn main() {
    nannou::app(model).update(update).run();
}

struct Circle {
    pos: Vector2,
    radius: f32,
}

struct RayBall {
    pos: Vector2,
    rays: Vec<Ray2D>,
    collisions: Vec<Vector2>,
}

struct Model {
    tile_count_w: u32,
    balls: Vec<Circle>,
    rays_balls: Vec<RayBall>,
    draw_gui: bool,
    ui: Ui,
    ids: Ids,
    ray_width: f32,
    wall_width: f32,
    collision_radius: f32,
    rotation: f32,
    scheme_id: usize,
    max_bounces: usize,
    blend_id: usize,
    color_off: usize,
    palette: Palette,
    show_balls: bool,
    draw_tex_overlay: bool,
    animation: bool,
    animation_speed: f32,
    draw_refl: bool,
    draw_polygon: bool,
    polygon_contour_weight: f32,
    texture: wgpu::Texture,
    padding: f32,
}

widget_ids! {
    struct Ids {
        wall_width,
        ray_width,
        max_bounces,
        collision_radius,
        rotation,
        scheme_id,
        blend_id,
        color_off,
        draw_refl,
        draw_polygon,
        polygon_contour_weight,
        animation,
        padding,
        animation_speed,
        show_balls,
        draw_tex_overlay
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

    let mut balls: Vec<Circle> = Vec::new();
    let mut rays_balls: Vec<RayBall> = Vec::new();
    let win = app.window_rect();

    let draw_gui = true;

    // Create the UI.
    let mut ui = app.new_ui().build().unwrap();

    // Generate some ids for our widgets.
    let ids = Ids::new(ui.widget_id_generator());

    let ray_width = 3.0;
    let wall_width = 2.0;
    let max_bounces = 5;
    let rotation = 0.0;
    let collision_radius = 3.0;

    let scheme_id = 5;
    let blend_id = 0;
    let color_off = 4;
    let palette = Palette::new();
    let padding = 0.44;
    let rad = (win.w() / tile_count_w as f32) * 0.3;
    make_balls(
        &mut balls,
        &mut rays_balls,
        &win,
        tile_count_w,
        padding,
        rad,
        11,
    );
    let show_balls = true;
    let animation = false;
    let animation_speed = 1.0;
    let draw_refl = true;
    let draw_polygon = false;
    let polygon_contour_weight = 5.0;
    let draw_tex_overlay = false;

    // texture
    // Load the image from disk and upload it to a GPU texture.
    let assets = app.assets_path().unwrap();
    let img_path = assets.join("images").join("noise-texture1-tr.png");
    //let img_path = assets.join("images").join("grunge-halftone-tr.png");
    let texture = wgpu::Texture::from_path(app, img_path).unwrap();

    Model {
        tile_count_w,
        balls,
        rays_balls,
        max_bounces,
        draw_gui,
        ui,
        ids,
        wall_width,
        collision_radius,
        ray_width,
        rotation,
        scheme_id,
        blend_id,
        color_off,
        palette,
        show_balls,
        animation,
        animation_speed,
        draw_refl,
        draw_polygon,
        padding,
        polygon_contour_weight,
        draw_tex_overlay,
        texture,
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

        for value in slider(model.collision_radius as f32, 3.0, 85.0)
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
        for value in slider(model.max_bounces as f32, 1.0, 50.0)
            .down(10.0)
            .label("max_bounces")
            .set(model.ids.max_bounces, ui)
        {
            model.max_bounces = value as usize;
        }

        for val in slider(model.rotation, -PI, PI)
            .down(10.0)
            .label("Rotation")
            .set(model.ids.rotation, ui)
        {
            model.rotation = val;
        }

        for value in slider(model.scheme_id as f32, 0.0, 5.0)
            .down(10.0)
            .label("scheme_id")
            .set(model.ids.scheme_id, ui)
        {
            model.scheme_id = value as usize;
        }

        for value in slider(model.blend_id as f32, 0.0, 3.0)
            .down(10.0)
            .label("blend_id")
            .set(model.ids.blend_id, ui)
        {
            model.blend_id = value as usize;
        }

        for value in slider(model.color_off as f32, 0.0, 4.0)
            .down(10.0)
            .label("color_off")
            .set(model.ids.color_off, ui)
        {
            model.color_off = value as usize;
        }

        for v in toggle(model.draw_refl as bool)
            .label("Draw reflection")
            .set(model.ids.draw_refl, ui)
        {
            model.draw_refl = v;
        }

        for value in slider(model.polygon_contour_weight, 1.0, 30.0)
            .down(10.0)
            .label("polygon cont weight")
            .set(model.ids.polygon_contour_weight, ui)
        {
            model.polygon_contour_weight = value;
        }

        for v in toggle(model.draw_polygon as bool)
            .label("Draw poly")
            .set(model.ids.draw_polygon, ui)
        {
            model.draw_polygon = v;
        }

        for v in toggle(model.draw_tex_overlay as bool)
            .label("Draw Overlay")
            .set(model.ids.draw_tex_overlay, ui)
        {
            model.draw_tex_overlay = v;
        }

        for v in toggle(model.animation as bool)
            .label("Animation")
            .set(model.ids.animation, ui)
        {
            model.animation = v;
        }

        for value in slider(model.animation_speed as f32, 2.0, 0.1)
            .down(10.0)
            .label("animation speed")
            .set(model.ids.animation_speed, ui)
        {
            model.animation_speed = value;
        }

        for v in toggle(model.show_balls as bool)
            .label("Show wall")
            .set(model.ids.show_balls, ui)
        {
            model.show_balls = v;
        }
    }

    for rbal in model.rays_balls.iter_mut() {
        // find the closest intersection
        let sensibility = 200.0;

        for (r, c) in rbal.rays.iter().zip(rbal.collisions.iter_mut()) {
            let mut distance: f32 = Float::infinity();
            let mut coll = vec2(0.0, 0.0);
            for b in model.balls.iter_mut() {
                //if b.pos.distance(rbal.pos) {
                if let Some(collision_distance) = r.intersect_circle(b.pos, b.radius) {
                    println!("coll");
                    println!("{:?}", distance);

                    if collision_distance < distance {
                        distance = collision_distance;
                        coll = r.orig + r.dir.with_magnitude(collision_distance);
                    }
                }
                //}
            }
            if distance < Float::infinity() && distance < sensibility {
                // there was a close collision, draw it
                *c = coll;
            } else {
                let cc = r.orig + r.dir.with_magnitude(sensibility);
                *c = cc;
            }
        }

        if model.animation {
            let win = _app.window_rect();
            let side = win.w() as f32 / model.tile_count_w as f32;
            let padding = side * model.padding;
            rbal.pos.y = rbal.pos.y + (_app.time * model.animation_speed).cos() * (padding / 6.0);
        }
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    let blends = [BLEND_NORMAL, BLEND_ADD, BLEND_SUBTRACT, BLEND_LIGHTEST];
    let draw = app.draw().color_blend(blends[model.blend_id].clone());
    frame.clear(model.palette.get_scheme(model.scheme_id)[4]);
    //let draw = app.draw();
    draw.background()
        .color(model.palette.get_fifth(model.scheme_id, model.color_off));

    // draw the balls
    if model.show_balls {
        for c in &model.balls {
            draw_circle_polygon(&draw, &model, &c.pos, &c.radius, &vec2(1.0, 1.0), &10.0);
            // draw.ellipse()
            //     .x_y(c.pos.x, c.pos.y)
            //     .w_h(c.radius * 2.0, c.radius * 2.0)
            //     .color(model.palette.get_second(model.scheme_id, model.color_off));
        }
    }

    for ray_ball in &model.rays_balls {
        //
        draw_ray_ball(&draw, &model, &ray_ball);
    }

    if model.draw_tex_overlay {
        draw.texture(&model.texture).w_h(800.0, 800.0);
    }
    draw.to_frame(app, &frame).unwrap();

    if model.draw_gui {
        model.ui.draw_to_frame(app, &frame).unwrap();
    }
}

fn make_balls(
    balls: &mut Vec<Circle>,
    ray_balls: &mut Vec<RayBall>,
    win: &geom::Rect,
    tile_count_w: u32,
    pad: f32,
    radius: f32,
    resolution: u8, // 0 even, 1 random rotation, 2 one in the middle, 4 diamond
) {
    let side = win.w() as u32 / tile_count_w;
    let mut xpos = win.left();
    let mut ypos = win.bottom();
    let padding = side as f32 * pad;
    let mut index = 2;
    for _x in 0..tile_count_w {
        for _y in 0..=(win.h() as u32 / side as u32) {
            let c_x = xpos + side as f32 / 2.0;
            let c_y = ypos + side as f32 / 2.0;

            let mut r_y = c_y - radius;
            let r_padding = radius * 2.0 / (resolution as f32 + 1.0);
            let mut rays: Vec<Ray2D> = Vec::new();
            let mut collisions: Vec<Vector2> = Vec::new();
            for i in (0..=360).step_by(resolution as usize) {
                let rad = deg_to_rad(i as f32);
                let mut r = Ray2D::new();
                r.orig = vec2(c_x + padding, r_y);
                r.dir = Vector2::from_angle(rad);
                let coll = r.orig + r.dir.with_magnitude(200.0);
                collisions.push(coll);
                rays.push(r);
            }
            println!("{:?}", rays);

            ray_balls.push(RayBall {
                pos: vec2(c_x + radius, c_y),
                rays: rays,
                collisions: collisions,
            });
            balls.push(Circle {
                pos: vec2(c_x, c_y),
                radius: radius,
            });

            ypos += side as f32;
            index += 1;
        }
        index += 1;
        ypos = win.bottom();
        xpos += side as f32;
    }
}

fn draw_ray_ball(draw: &Draw, model: &Model, ray_ball: &RayBall) {
    for (r, c) in ray_ball.rays.iter().zip(ray_ball.collisions.iter()) {
        //println!("{:?}", c);
        draw.line().start(r.orig).end(*c);
    }
}

fn draw_circle_polygon(
    draw: &Draw,
    model: &Model,
    center: &Vector2,
    radius: &f32,
    lightDir: &Vector2,
    offset: &f32,
) {
    let points = (0..=360).step_by(2).map(|i| {
        let rad = deg_to_rad(i as f32);
        (
            *center + vec2(rad.sin() * *radius, rad.cos() * *radius),
            model.palette.get_second(model.scheme_id, model.color_off),
        )
    });
    draw.polygon()
        .stroke_weight(model.polygon_contour_weight)
        //.stroke()
        .points_colored(points);
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
