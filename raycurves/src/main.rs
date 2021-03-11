use edapx_colors::Palette;
use nannou::prelude::*;
use nannou::ui::prelude::*;

mod bouncing;
mod mondrian;
pub use crate::bouncing::BouncingRay2D;
use crate::mondrian::split_squares;
pub use crate::mondrian::Square;

const EPSILON: f32 = 0.05;
const ARROW_LENGTH: f32 = 40.0;

fn main() {
    nannou::app(model).update(update).run();
}

struct Curve {
    points: Vec<Vector2>,
}

struct Model {
    walls: Vec<Curve>,
    tile_count_w: u32,
    wall_mode: u32,
    rays: Vec<BouncingRay2D>,
    draw_gui: bool,
    ui: Ui,
    ids: Ids,
    ray_width: f32,
    rays_prob: f32,
    wall_width: f32,
    wall_split: f32,
    hole_pct: f32,
    hole_n: usize,
    wall_padding: f32,
    collision_radius: f32,
    rotation: f32,
    scheme_id: usize,
    max_bounces: usize,
    blend_id: usize,
    color_off: usize,
    palette: Palette,
    show_walls: bool,
    draw_arrows: bool,
    draw_tex_overlay: bool,
    animation: bool,
    animation_speed: f32,
    animation_time: f32,
    draw_polygon: bool,
    polygon_contour_weight: f32,
    texture: wgpu::Texture,
    clear_interval: usize,
}

widget_ids! {
    struct Ids {
        wall_width,
        wall_split,
        wall_padding,
        hole_pct,
        hole_n,
        tile_count_w,
        button,
        wall_mode,
        ray_width,
        rays_prob,
        max_bounces,
        collision_radius,
        rotation,
        scheme_id,
        blend_id,
        color_off,
        animation_time,
        draw_polygon,
        draw_arrows,
        polygon_contour_weight,
        animation,
        animation_speed,
        show_walls,
        draw_tex_overlay,
        clear_interval
    }
}

fn model(app: &App) -> Model {
    let tile_count_w = 8;
    app.new_window()
        //.size(1280, 720)
        .size(1600, 900)
        //.size(1777, 1000)
        //.size(1920,1080)
        // .size( 3840,2160)
        // .size(2560, 1440) // 16:9
        .view(view)
        .key_pressed(key_pressed)
        .build()
        .unwrap();

    let mut walls: Vec<Curve> = Vec::new();
    let mut rays: Vec<BouncingRay2D> = Vec::new();
    let win = app.window_rect();

    let draw_gui = true;

    // Create the UI.
    let mut ui = app.new_ui().build().unwrap();

    // Generate some ids for our widgets.
    let ids = Ids::new(ui.widget_id_generator());

    let ray_width = 3.0;
    let wall_width = 2.0;
    let wall_split = 0.3;
    let wall_padding = 0.07;
    let hole_pct = 0.0;
    let hole_n = 1;
    let wall_mode = 2;
    let max_bounces = 10;
    let rotation = 0.0;
    let collision_radius = 3.0;
    let rays_prob = 0.0;

    let scheme_id = 5;
    let blend_id = 0;
    let color_off = 4;
    let palette = Palette::new();
    let clear_interval = 14;
    make_walls(
        &mut walls,
        &mut rays,
        &win,
        tile_count_w,
        wall_split,
        wall_padding,
        hole_pct,
        hole_n,
        rays_prob,
        rotation,
        wall_mode,
    );
    let show_walls = true;
    let animation = true;
    let draw_arrows = true;
    let animation_speed = 2.0;
    let animation_time = 0.0;
    let draw_polygon = true;
    let polygon_contour_weight = 5.0;
    let draw_tex_overlay = false;

    // texture
    // Load the image from disk and upload it to a GPU texture.
    let assets = app.assets_path().unwrap();
    let img_path = assets.join("images").join("noise-texture1-tr.png");
    //let img_path = assets.join("images").join("water.png");
    //let img_path = assets.join("images").join("grunge-halftone-tr.png");
    let texture = wgpu::Texture::from_path(app, img_path).unwrap();

    Model {
        walls,
        wall_mode,
        tile_count_w,
        rays,
        max_bounces,
        draw_gui,
        ui,
        ids,
        wall_width,
        wall_split,
        wall_padding,
        hole_pct,
        hole_n,
        collision_radius,
        ray_width,
        rays_prob,
        rotation,
        scheme_id,
        blend_id,
        color_off,
        palette,
        show_walls,
        animation,
        animation_speed,
        animation_time,
        draw_polygon,
        draw_arrows,
        polygon_contour_weight,
        draw_tex_overlay,
        clear_interval,
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
            .top_left_with_margin(10.0)
            .label("wall width")
            .set(model.ids.wall_width, ui)
        {
            model.wall_width = value;
        }

        for value in slider(model.wall_split as f32, 0.0, 1.0)
            .down(3.0)
            .label("wall split")
            .set(model.ids.wall_split, ui)
        {
            model.wall_split = value;
        }

        for value in slider(model.wall_padding as f32, 0.2, 0.02)
            .down(3.0)
            .label("wall padding")
            .set(model.ids.wall_padding, ui)
        {
            model.wall_padding = value;
        }

        for value in slider(model.hole_pct as f32, 0.0, 0.9)
            .down(3.0)
            .label("hole")
            .set(model.ids.hole_pct, ui)
        {
            model.hole_pct = value;
        }

        for value in slider(model.hole_n as f32, 0.0, 6.0)
            .down(1.0)
            .label("hole_n")
            .set(model.ids.hole_n, ui)
        {
            model.hole_n = value as usize;
        }

        for value in slider(model.wall_mode as f32, 1.0, 5.0)
            .down(3.0)
            .label("wall_mode ")
            .set(model.ids.wall_mode, ui)
        {
            model.wall_mode = value as u32;
        }

        for value in slider(model.tile_count_w as f32, 1.0, 20.0)
            .down(3.0)
            .label("tile_count_w")
            .set(model.ids.tile_count_w, ui)
        {
            model.tile_count_w = value as u32;
        }

        for _click in widget::Button::new()
            .down(3.0)
            //.w_h(200.0, 60.0)
            .label("Regenerate Walls")
            .label_font_size(15)
            .rgb(0.3, 0.3, 0.3)
            .label_rgb(1.0, 1.0, 1.0)
            .border(0.0)
            .set(model.ids.button, ui)
        {
            let win = _app.window_rect();
            make_walls(
                &mut model.walls,
                &mut model.rays,
                &win,
                model.tile_count_w,
                model.wall_split,
                model.wall_padding,
                model.hole_pct,
                model.hole_n,
                model.rays_prob,
                model.rotation,
                model.wall_mode,
            );
        }

        for value in slider(model.collision_radius as f32, 0.0, 185.0)
            .down(3.0)
            .label("collision radius")
            .set(model.ids.collision_radius, ui)
        {
            model.collision_radius = value;
        }

        for value in slider(model.ray_width, 1.0, 10.0)
            .down(3.0)
            .label("ray width")
            .set(model.ids.ray_width, ui)
        {
            model.ray_width = value;
        }
        for value in slider(model.rays_prob as f32, 0.0, 1.0)
            .down(3.0)
            .label("rays prob.")
            .set(model.ids.rays_prob, ui)
        {
            model.rays_prob = value;
        }
        for value in slider(model.max_bounces as f32, 1.0, 400.0)
            .down(3.0)
            .label("max_bounces")
            .set(model.ids.max_bounces, ui)
        {
            model.max_bounces = value as usize;
        }
        for value in slider(model.clear_interval as f32, 5.0, 20.0)
            .down(3.0)
            .label("clear_interval")
            .set(model.ids.clear_interval, ui)
        {
            model.clear_interval = value as usize;
        }

        for val in slider(model.rotation, -PI, PI)
            .down(3.0)
            .label("Rotation")
            .set(model.ids.rotation, ui)
        {
            model.rotation = val;
        }

        for value in slider(model.scheme_id as f32, 0.0, 5.0)
            .down(3.0)
            .label("scheme_id")
            .set(model.ids.scheme_id, ui)
        {
            model.scheme_id = value as usize;
        }

        for value in slider(model.blend_id as f32, 0.0, 3.0)
            .down(3.0)
            .label("blend_id")
            .set(model.ids.blend_id, ui)
        {
            model.blend_id = value as usize;
        }

        for value in slider(model.color_off as f32, 0.0, 4.0)
            .down(3.0)
            .label("color_off")
            .set(model.ids.color_off, ui)
        {
            model.color_off = value as usize;
        }

        for value in slider(model.polygon_contour_weight, 1.0, 30.0)
            .down(3.0)
            .label("polygon cont weight")
            .set(model.ids.polygon_contour_weight, ui)
        {
            model.polygon_contour_weight = value;
        }

        for v in toggle(model.draw_polygon as bool)
            .down(3.0)
            .label("Draw poly")
            .set(model.ids.draw_polygon, ui)
        {
            model.draw_polygon = v;
        }

        for v in toggle(model.draw_arrows as bool)
            .down(3.0)
            .label("Draw Arrows")
            .set(model.ids.draw_arrows, ui)
        {
            model.draw_arrows = v;
        }

        for v in toggle(model.draw_tex_overlay as bool)
            .down(3.0)
            .label("Draw Overlay")
            .set(model.ids.draw_tex_overlay, ui)
        {
            model.draw_tex_overlay = v;
        }

        for v in toggle(model.animation as bool)
            .down(3.0)
            .label("Animation")
            .set(model.ids.animation, ui)
        {
            model.animation = v;
        }

        for value in slider(model.animation_speed as f32, 80.0, 0.01)
            .down(3.0)
            .label("animation speed")
            .set(model.ids.animation_speed, ui)
        {
            model.animation_speed = value;
        }

        for v in toggle(model.show_walls as bool)
            .down(3.0)
            .label("Show wall")
            .set(model.ids.show_walls, ui)
        {
            model.show_walls = v;
        }
    }

    for r in model.rays.iter_mut() {
        r.max_bounces = model.max_bounces;
        if _app.time.round() as usize % model.clear_interval == 0 && model.animation {
            r.collisions.clear();
            r.reflections.clear();
            r.refl_intensity.clear();
        }

        // if model.animation {
        //     model.animation_time = _app.time * model.animation_speed;
        // }
        //r.primary_ray.dir = r.ray.dir.rotate(model.rotation);
        r.primary_ray.set_dir_from_angle(model.rotation);
        // println!("{:?}", r.primary_ray.dir.x);
        // r.primary_ray.dir = r.primary_ray.dir.rotate(model.animation_time + model.rotation);

        let mut collision: Vector2 = vec2(0.0, 0.0);
        let mut distance: f32 = Float::infinity();
        let mut surface_normal: Vector2 = vec2(0.0, 0.0);
        // find the closest intersection point between the ray and the walls
        for curve in model.walls.iter() {
            if let Some(collision) = r.ray.intersect_polyline(&curve.points) {
                // save the closest possible collision
                if collision.0 < distance {
                    distance = collision.0;
                    surface_normal = collision.1;
                }
            }
        }

        if r.bounces < r.max_bounces {
            if (distance - ARROW_LENGTH) < EPSILON + model.animation_speed {
                // bounce!
                collision = r.ray.orig + r.ray.dir.with_magnitude(distance);
                r.bounces += 1;
                let refl = r.ray.reflect(surface_normal);
                r.refl_intensity.push(r.ray.dir.dot(refl).abs());
                r.ray.orig = collision + refl.with_magnitude(EPSILON); // avoid self intersection bouncing a bit more far away
                r.ray.dir = refl;
                r.collisions.push(collision);
                //r.refractions.push(r.ray.refract(surface_normal, 1.0));
                r.reflections.push(refl);
            } else {
                // keep moving
                if distance < Float::infinity() {
                    r.ray.orig = r.ray.orig + r.ray.dir.with_magnitude(model.animation_speed);
                } else {
                    r.reset();
                }
            }
        } else {
            r.reset();
        }
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    let blends = [BLEND_NORMAL, BLEND_ADD, BLEND_SUBTRACT, BLEND_LIGHTEST];
    let draw = app.draw().color_blend(blends[model.blend_id].clone());
    frame.clear(model.palette.get_fifth(model.scheme_id, model.color_off));
    //frame.clear(BLACK);
    //let draw = app.draw();
    // draw.background()
    //     .color(model.palette.get_fifth(model.scheme_id, model.color_off));

    // draw the walls
    if model.show_walls {
        for curve in model.walls.iter() {
            //println!("{:?}", curve.points.len());
            draw.polyline()
                .weight(model.wall_width)
                .color(model.palette.get_second(model.scheme_id, model.color_off))
                // look at points_colored
                .points(curve.points.clone());
            //.caps_round();
        }
    }

    for r in &model.rays {
        if model.draw_arrows {
            draw.arrow()
                .color(model.palette.get_first(model.scheme_id, model.color_off))
                .start(r.ray.orig)
                .stroke_weight(model.ray_width)
                .end(r.ray.orig + r.ray.dir.with_magnitude(ARROW_LENGTH));
        }

        if r.collisions.len() > 3 && model.collision_radius > 0.0 {
            for (&c, &i) in r.collisions.iter().zip(r.refl_intensity.iter()) {
                draw.ellipse()
                    .no_fill()
                    .stroke(model.palette.get_third(model.scheme_id, model.color_off))
                    .stroke_weight(3.0)
                    .x_y(c.x, c.y)
                    .w_h(model.collision_radius * i, model.collision_radius * i);
            }
        }

        let mut col = rgba(0.0, 0.0, 0.0, 0.0);
        //let win = app.window_rect();
        let ppp = r
            .collisions
            .iter()
            .zip(r.reflections.iter())
            .map(|(&co, &re)| {
                if re.x > 0.0 {
                    col = model.palette.get_third(model.scheme_id, model.color_off)
                } else {
                    col = model.palette.get_fourth(model.scheme_id, model.color_off)
                }
                // let xc = map_range(co.x, win.left(), win.right(), 0.0, 1.0);
                // let xy = map_range(co.y, win.bottom(), win.top(), 0.0, 1.0);
                // let tex_coords = [xc, xy];
                // (pt2(co.x, co.y), tex_coords)
                (pt2(co.x, co.y), col)
            });

        if model.draw_polygon {
            if ppp.len() > 3 {
                draw.polygon()
                    .stroke(model.palette.get_second(model.scheme_id, model.color_off))
                    .stroke_weight(model.polygon_contour_weight)
                    .join_round()
                    .points_colored(ppp);
                //draw.polygon().points_textured(&model.texture, ppp);
            }
        };

        // if model.draw_tex_overlay {
        //     if ppp.len() > 3 {
        //         draw.polygon().points_textured(&model.texture, ppp);
        //     }
        // }

        if r.collisions.len() > 3 {
            draw.path()
                .stroke()
                .caps_round()
                .stroke_weight(model.ray_width)
                .points(r.collisions.iter().cloned())
                .color(model.palette.get_first(model.scheme_id, model.color_off));
        }

        for (&c, &r) in r.collisions.iter().zip(r.reflections.iter()) {
            if model.draw_arrows {
                draw.arrow()
                    .start(c)
                    .end(c + r.with_magnitude(40.0))
                    .stroke_weight(model.ray_width)
                    .color(model.palette.get_first(model.scheme_id, model.color_off));
            }
        }
        if model.draw_tex_overlay {
            draw.texture(&model.texture).w_h(800.0, 800.0);
        }
    }

    draw.to_frame(app, &frame).unwrap();

    if model.draw_gui {
        model.ui.draw_to_frame(app, &frame).unwrap();
    }
}

fn make_walls(
    walls: &mut Vec<Curve>,
    rays: &mut Vec<BouncingRay2D>,
    win: &geom::Rect,
    tile_count_w: u32,
    wall_split: f32,
    perc_padding: f32,
    hole_pct: f32,
    hole_n: usize,
    rays_prob: f32,
    rot: f32,
    mode: u32, // 0 even, 1 random rotation, 2 one in the middle, 4 diamond
) {
    walls.clear();
    rays.clear();
    let margin: i32 = 100;
    let step = (win.w() as f32) as u32 / tile_count_w;

    //let step = 200;

    let mut squares: Vec<Square> = Vec::new();
    squares.push(Square {
        x: win.left() + (margin as f32 / 2.0),
        y: win.bottom() + (margin as f32 / 2.0),
        width: (win.w() - margin as f32),
        height: (win.h() - margin as f32),
    });
    for i in (win.left() as i32..win.right() as i32).step_by(step as usize) {
        split_squares(i as f32, i as f32, &mut squares, wall_split);
    }
    for square in &squares {
        match mode {
            1 => {
                let padding = step as f32 * perc_padding;
                let mut r = BouncingRay2D::new();
                r.primary_ray.dir = Vector2::from_angle(random_range(-PI, PI));
                r.primary_ray.orig = vec2(
                    square.x + square.width * 0.8,
                    square.y + square.height * 0.3,
                );
                r.reset();
                rays.push(r);
                create_curvedwalls_from_square(&square, walls, mode, padding, hole_pct, hole_n);
            }
            2 => {
                let padding = step as f32 * perc_padding;
                if random_range(0.0, 1.0) > rays_prob {
                    let mut r = BouncingRay2D::new();
                    //r.primary_ray.dir = Vector2::from_angle(random_range(-PI, PI));
                    r.primary_ray.dir = Vector2::from_angle(rot);
                    //r.primary_ray.set_dir_from_angle(model.rotation);
                    r.primary_ray.orig = vec2(
                        square.x + square.width * 0.5,
                        square.y + square.height * 0.5,
                    );
                    r.reset();
                    rays.push(r);
                }
                create_curvedwalls_from_square(&square, walls, mode, padding, hole_pct, hole_n);
            }
            _ => {}
        }
    }

    //println!("{:?}", walls.len());
    //println!("{:?}", squares);
}

fn create_curvedwalls_from_square(
    square: &Square,
    walls: &mut Vec<Curve>,
    mode: u32,
    padding: f32,
    hole: f32,
    hole_n: usize,
) {
    //let padding = square.width * 0.1;
    match mode {
        // Diagonal?
        1 => {
            let points = vec![
                vec2(square.x + padding, square.y + padding),
                vec2(
                    square.x + square.width,
                    square.y - padding + square.height - padding,
                ),
            ];
            let curve = Curve { points: points };
            walls.push(curve);
        }
        // closed square
        2 => {
            // let bottom_points = vec![
            //     vec2(square.x + padding + hole, square.y + padding),
            //     vec2(
            //         square.x - hole + square.width - padding * 2.0,
            //         square.y + padding,
            //     )];
            // let bottom_curve = Curve{points: bottom_points};
            // walls.push(bottom_curve);

            // // top
            // let top_points = vec![
            // vec2(
            //     square.x + padding + hole,
            //     square.y + square.height - padding * 2.0,
            // ),
            // vec2(
            //     square.x - hole + square.width - padding * 2.0,
            //     square.y + square.height - padding * 2.0,
            // )];
            // let top_curve = Curve{points: top_points};
            // walls.push(top_curve);

            // // left
            // let left_points = vec![
            // vec2(
            //     square.x + square.width - padding * 2.0,
            //     square.y + padding + hole,
            // ),
            // vec2(
            //     square.x + square.width - padding * 2.0,
            //     square.y - hole + square.height - padding * 2.0,
            // )];
            // let left_curve = Curve{points: left_points};
            // walls.push(left_curve);

            // // right
            // let right_points = vec![
            // vec2(square.x + padding, square.y + padding + hole),
            // vec2(
            //     square.x + padding,
            //     square.y - hole + square.height - padding * 2.0,
            // )];
            // let right_curve = Curve{points: right_points};
            // walls.push(right_curve);

            create_curve_from_square(square, mode, padding, hole, hole_n, walls);
        }
        // un modo coi rombi e con alcuni muri aperti.
        _ => {}
    }
}

fn create_curve_from_square(
    square: &Square,
    mode: u32,
    padding: f32,
    hole: f32,
    hole_n: usize,
    walls: &mut Vec<Curve>,
) {
    let center = vec2(
        square.x + square.width / 2.0,
        square.y + square.height / 2.0,
    );
    //let radius = square.width / 2.0 - padding;
    let mut points = vec![];
    // let points = (0..=360).step_by(2).map(|i| {
    //     let rad = deg_to_rad(i as f32);
    //     (
    //         center + vec2(rad.sin() * radius, rad.cos() * radius),
    //         //model.palette.get_second(model.scheme_id, model.color_off),
    //     )
    // });

    let mut wall_length = 360.00;
    if hole_n > 0 {
        wall_length = 360.00 / hole_n as f32;
    }

    let pad = wall_length * hole;
    let mut start_from = 0.0;
    let mut end_to = start_from + wall_length - pad;
    println!("{:?}  hh", hole);
    println!("{:?}  wap", wall_length);
    // println!("{:?}  aap", pad);
    // println!("{:?} aas", start_from);
    // println!("{:?} aae", end_to);

    for i in (0..=360).step_by(2) {
        let rad = deg_to_rad(i as f32);
        //points.push(center + vec2(rad.sin() * radius, rad.cos() * radius));
        let x = (square.width / 2.0 - padding) * rad.cos();
        let y = (square.height / 2.0 - padding) * rad.sin();

        if i as f32 >= start_from && (i as f32) < end_to {
            points.push(center + vec2(x, y))
        }

        if i as f32 >= end_to {
            // println!("{:?}  p", padding);
            // println!("{:?} s", start_from);
            // println!("{:?} e", end_to);

            points.push(center + vec2(x, y));
            walls.push(Curve {
                points: points.clone(),
            });
            points.clear();
            start_from = i as f32;
            end_to = start_from + wall_length - pad;
        }
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
