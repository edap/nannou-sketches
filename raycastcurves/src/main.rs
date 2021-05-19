use edapx_colors::Palette;
use nannou::prelude::*;
use nannou::ui::prelude::*;
use rayon::prelude::*;

mod types;
use crate::types::Curve;
mod mondrian;
use crate::mondrian::split_squares;
pub use crate::mondrian::Square;
mod bouncing;
pub use crate::bouncing::BouncingRay2D;
mod ray_helper;
use crate::ray_helper::make_rays;

const EPSILON: f32 = 0.05;
const ARROW_LENGTH: f32 = 40.0;

fn main() {
    nannou::app(model).update(update).run();
}

struct Model {
    walls: Vec<Curve>,
    tile_count_w: u32,
    n_caster: u32,
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
        n_caster,
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
    let hole_pct = 0.25;
    let hole_n = 2;
    let n_caster = 20;
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
        &win,
        tile_count_w,
        wall_split,
        wall_padding,
        hole_pct,
        hole_n,
        rays_prob,
        rotation,
        n_caster,
    );
    make_rays(&mut rays, &win, tile_count_w, n_caster);
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
        n_caster,
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

        for value in slider(model.n_caster as f32, 1.0, 50.0)
            .down(3.0)
            .label("n_caster ")
            .set(model.ids.n_caster, ui)
        {
            model.n_caster = value as u32;
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
                &win,
                model.tile_count_w,
                model.wall_split,
                model.wall_padding,
                model.hole_pct,
                model.hole_n,
                model.rays_prob,
                model.rotation,
                model.n_caster,
            );
            make_rays(&mut model.rays, &win, model.tile_count_w, model.n_caster)
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

    let time = _app.time;
    let rot = model.rotation;
    let anim = model.animation;
    let anim_speed = model.animation_speed;
    let wallss = &model.walls;
    let win = _app.window_rect();
    model
        .rays
        .par_iter_mut()
        .for_each(|ray| ray_collides(ray, rot, anim, anim_speed, time, wallss, win));
}

fn ray_collides(
    r: &mut BouncingRay2D,
    rotation: f32,
    animation: bool,
    animation_speed: f32,
    time: f32,
    walls: &Vec<Curve>,
    win: geom::Rect,
) {
    r.collisions.clear();
    r.reflections.clear();
    r.refl_intensity.clear();

    if animation {
        if r.primary_ray.dir.x > 0.0 {
            r.primary_ray.orig.x += 0.1 * animation_speed;
        } else {
            r.primary_ray.orig.x -= 0.1 * animation_speed;
        }
        //r.primary_ray.orig = r.primary_ray.orig + r.primary_ray.dir.with_magnitude(animation_speed);
        if r.primary_ray.orig.x >= win.right() as f32 {
            r.primary_ray.orig.x = win.left();
        } else if r.primary_ray.orig.x <= win.left() as f32 {
            r.primary_ray.orig.x = win.right();
        }
    }

    while !r.max_bounces_reached() {
        let collision: Vector2;
        let mut distance: f32 = Float::infinity();
        let mut surface_normal: Vector2 = vec2(0.0, 0.0);
        // find the closest intersection point between the ray and the walls
        for curve in walls.iter() {
            if let Some(collision) = r.ray.intersect_polyline(&curve.points) {
                // save the closest possible collision
                if collision.0 < distance {
                    distance = collision.0;
                    surface_normal = collision.1;
                }
            }
        }
        if distance < Float::infinity() {
            // collision point
            collision = r.ray.orig + r.ray.dir.with_magnitude(distance);
            r.bounces += 1;
            let refl = r.ray.reflect(surface_normal);
            r.refl_intensity.push(r.ray.dir.dot(refl).abs());
            r.ray.orig = collision + refl.with_magnitude(0.03);
            r.ray.dir = refl;
            r.collisions.push(collision);
            r.reflections.push(refl);
        } else {
            break;
        };
    }
    r.reset();
    //r.ray.set_dir_from_angle(rotation);
}

fn view(app: &App, model: &Model, frame: Frame) {
    let blends = [BLEND_NORMAL, BLEND_ADD, BLEND_SUBTRACT, BLEND_LIGHTEST];
    let draw = app.draw().color_blend(blends[model.blend_id].clone());
    frame.clear(model.palette.get_fifth(model.scheme_id, model.color_off));

    // // draw the walls
    // if model.show_walls {
    //     for curve in model.walls.iter() {
    //         //println!("{:?}", curve.points.len());
    //         draw.polyline()
    //             .weight(model.wall_width)
    //             .color(model.palette.get_second(model.scheme_id, model.color_off))
    //             // look at points_colored
    //             .points(curve.points.clone());
    //         //.caps_round();
    //     }
    // }

    // for r in &model.rays {
    //     draw.arrow()
    //         .color(model.palette.get_scheme(model.scheme_id)[0])
    //         .start(r.ray.orig)
    //         .weight(model.ray_width * 2.0)
    //         .end(r.ray.orig + r.ray.dir.with_magnitude(20.0));
    //     for (&c, &i) in r.collisions.iter().zip(r.refl_intensity.iter()) {
    //         draw.ellipse()
    //             .no_fill()
    //             .stroke(model.palette.get_scheme(model.scheme_id)[2])
    //             .stroke_weight(3.0)
    //             .x_y(c.x, c.y)
    //             .w_h(model.collision_radius * i, model.collision_radius * i);
    //     }

    //     let mut col = rgba(0.0, 0.0, 0.0, 0.0);
    //     println!("collision dd{:?}", r.collisions.len());
    //     let ppp = r
    //         .collisions
    //         .iter()
    //         .zip(r.reflections.iter())
    //         .map(|(&co, &re)| {
    //             if re.x > 0.0 {
    //                 col = model.palette.get_scheme(model.scheme_id)[2]
    //             } else {
    //                 col = model.palette.get_scheme(model.scheme_id)[3]
    //             }
    //             (pt2(co.x, co.y), col)
    //         });

    //     if model.draw_polygon {
    //         draw.polygon().points_colored(ppp);
    //     };

    //     draw.path()
    //         .stroke()
    //         .stroke_weight(model.ray_width)
    //         .caps_round()
    //         .points(r.collisions.iter().cloned())
    //         .color(model.palette.get_scheme(model.scheme_id)[0]);

    //     for (&c, &r) in r.collisions.iter().zip(r.reflections.iter()) {
    //         draw.arrow()
    //             .start(c)
    //             .end(c + r.with_magnitude(20.0))
    //             .stroke_weight(model.ray_width)
    //             .color(model.palette.get_scheme(model.scheme_id)[4]);
    //     }

    //}

    // draw the walls
    if model.show_walls {
        for curve in model.walls.iter() {
            //println!("{:?}", curve.points.len());
            draw.polyline()
                .weight(model.wall_width)
                .color(model.palette.get_second(model.scheme_id, model.color_off))
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
    let margin: i32 = 100;
    let step = (win.w() as f32) as u32 / tile_count_w;

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
        let padding = step as f32 * perc_padding;
        create_curvedwalls_from_square(&square, walls, mode, padding, hole_pct, hole_n);
    }
}

fn create_curvedwalls_from_square(
    square: &Square,
    walls: &mut Vec<Curve>,
    mode: u32,
    padding: f32,
    hole: f32,
    hole_n: usize,
) {
    create_curve_from_square(square, mode, padding, hole, hole_n, walls);
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
    let mut points = vec![];

    let mut wall_length = 360;
    if hole_n > 0 {
        wall_length = 360 / hole_n;
    }

    let pad = (wall_length as f32 * hole) as usize;
    let mut start_from = 0;
    let mut end_to = start_from + wall_length - pad;

    for i in (0..=360).step_by(1) {
        let rad = deg_to_rad(i as f32);
        //points.push(center + vec2(rad.sin() * radius, rad.cos() * radius));
        let x = (square.width / 2.0 - padding) * rad.cos();
        let y = (square.height / 2.0 - padding) * rad.sin();

        if i >= start_from && i < end_to {
            points.push(center + vec2(x, y))
        }

        if i == end_to {
            points.push(center + vec2(x, y));
            walls.push(Curve {
                points: points.clone(),
            });
            points.clear();
            start_from = i + pad;
            end_to = start_from + wall_length - pad;
        }
    }
}

fn key_pressed(app: &App, model: &mut Model, key: Key) {
    match key {
        Key::S => {
            app.main_window()
                .capture_frame(app.time.to_string() + ".png");
        }
        Key::G => model.draw_gui = !model.draw_gui,
        _other_key => {}
    }
}
