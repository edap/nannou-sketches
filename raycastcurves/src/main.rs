use edapx_colors::Palette;
use nannou::prelude::*;
use nannou::ui::prelude::*;
use rayon::prelude::*;

mod gui;
mod types;
mod ray_light;
pub mod wraycaster;
use crate::types::Curve;
mod mondrian;
pub use crate::mondrian::Square;
mod bouncing;
pub use crate::bouncing::BouncingRay2D;
mod ray_helper;
use crate::ray_helper::make_raycasters;
mod wall_helper;
use crate::wall_helper::make_walls;
mod raycaster;
pub use crate::raycaster::Raycaster;

const EPSILON: f32 = 0.05;
const ARROW_LENGTH: f32 = 40.0;

// TODO implement whitted cast_ray function. Change max_bounces to max depth. Assumes that all your
// Curves in the scene are transparents.
// Calculate the color of the collision points on the curves. Assume white light (at the beginning). Save the depth level for each collision(?)
// Draw the polygon grouping the points by depth level.
// Add a bounding box for the curves.

fn main() {
    nannou::app(model).update(update).run();
}

struct Model {
    main_window: WindowId,
    walls: Vec<Curve>,
    tile_count_w: u32,
    n_caster: u32,
    rays: Vec<Raycaster>,
    ui: Ui,
    ids: gui::Ids,
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
    clear_interval: usize,
}

fn model(app: &App) -> Model {
    let tile_count_w = 8;
    let main_window = app
        .new_window()
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
    let mut rays: Vec<Raycaster> = Vec::new();
    //l = app.window_rect();
    let win = app.window(main_window).unwrap().rect();

    // Create the UI.
    let ui_window = app
        .new_window()
        .title(app.exe_name().unwrap() + " controls")
        .size(gui::WIN_W, gui::WIN_H)
        .view(ui_view)
        .event(ui_event)
        .key_pressed(key_pressed)
        .build()
        .unwrap();

    let mut ui = app.new_ui().window(ui_window).build().unwrap();
    let ids = gui::Ids::new(ui.widget_id_generator());
    ui.clear_with(color::DARK_CHARCOAL);
    let mut theme = ui.theme_mut();
    theme.label_color = color::WHITE;
    theme.shape_color = color::CHARCOAL;

    let ray_width = 3.0;
    let wall_width = 2.0;
    let wall_split = 0.3;
    let wall_padding = 0.07;
    let hole_pct = 0.25;
    let hole_n = 2;
    let n_caster = 2;
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
    );
    make_raycasters(&mut rays, &win, tile_count_w, n_caster);
    let show_walls = true;
    let animation = true;
    let draw_arrows = true;
    let animation_speed = 2.0;
    let animation_time = 0.0;
    let draw_polygon = true;
    let polygon_contour_weight = 5.0;
    let draw_tex_overlay = false;

    let mut the_model = Model {
        main_window,
        walls,
        n_caster,
        tile_count_w,
        rays,
        max_bounces,
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
    };
    ui_event(&app, &mut the_model, WindowEvent::Focused);
    the_model
}

fn update(app: &App, model: &mut Model, _update: Update) {
    let time = app.time;
    let rot = model.rotation;
    let anim = model.animation;
    let anim_speed = model.animation_speed;
    let wallss = &model.walls;
    let win = app.window(model.main_window).unwrap().rect();

    if model.animation {
        model
            .rays
            .par_iter_mut()
            .for_each(|r| r.bounce_horizontally(&win, anim_speed))
    }

    model
        .rays
        .par_iter_mut()
        .for_each(|ray| ray.collide(rot, anim, anim_speed, time, wallss, win));
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

    // for r in &model.rays {
    //     r.draw(
    //         &draw,
    //         model.polygon_contour_weight,
    //         model.ray_width,
    //         *model.palette.get_first(model.scheme_id, model.color_off),
    //         *model.palette.get_second(model.scheme_id, model.color_off),
    //     );
    // }

    for r in &model.rays {
        r.draw_inside(
            &draw,
            model.polygon_contour_weight,
            model.ray_width,
            *model.palette.get_first(model.scheme_id, model.color_off),
            *model.palette.get_second(model.scheme_id, model.color_off),
            *model.palette.get_third(model.scheme_id, model.color_off),
            *model.palette.get_fourth(model.scheme_id, model.color_off),
            *model.palette.get_fifth(model.scheme_id, model.color_off),
        );
    }

    // if model.draw_arrows {
    //     draw.arrow()
    //         .color(model.palette.get_first(model.scheme_id, model.color_off))
    //         .start(r.ray.orig)
    //         .stroke_weight(model.ray_width)
    //         .end(r.ray.orig + r.ray.dir.with_magnitude(ARROW_LENGTH));
    // }

    // if r.collisions.len() > 3 && model.collision_radius > 0.0 {
    //     for (&c, &i) in r.collisions.iter().zip(r.refl_intensity.iter()) {
    //         draw.ellipse()
    //             .no_fill()
    //             .stroke(model.palette.get_third(model.scheme_id, model.color_off))
    //             .stroke_weight(3.0)
    //             .x_y(c.x, c.y)
    //             .w_h(model.collision_radius * i, model.collision_radius * i);
    //     }
    // }

    // let mut col = rgba(0.0, 0.0, 0.0, 0.0);
    // //let win = app.window_rect();
    // let ppp = r
    //     .collisions
    //     .iter()
    //     .zip(r.reflections.iter())
    //     .map(|(&co, &re)| {
    //         if re.x > 0.0 {
    //             col = model.palette.get_third(model.scheme_id, model.color_off)
    //         } else {
    //             col = model.palette.get_fourth(model.scheme_id, model.color_off)
    //         }
    //         // let xc = map_range(co.x, win.left(), win.right(), 0.0, 1.0);
    //         // let xy = map_range(co.y, win.bottom(), win.top(), 0.0, 1.0);
    //         // let tex_coords = [xc, xy];
    //         // (pt2(co.x, co.y), tex_coords)
    //         (pt2(co.x, co.y), col)
    //     });

    // if model.draw_polygon {
    //     if ppp.len() > 3 {
    //         draw.polygon()
    //             .stroke(model.palette.get_second(model.scheme_id, model.color_off))
    //             .stroke_weight(model.polygon_contour_weight)
    //             .join_round()
    //             .points_colored(ppp);
    //         //draw.polygon().points_textured(&model.texture, ppp);
    //     }
    // };

    // if r.collisions.len() > 3 {
    //     draw.path()
    //         .stroke()
    //         .caps_round()
    //         .stroke_weight(model.ray_width)
    //         .points(r.collisions.iter().cloned())
    //         .color(model.palette.get_first(model.scheme_id, model.color_off));
    // }

    // for (&c, &r) in r.collisions.iter().zip(r.reflections.iter()) {
    //     if model.draw_arrows {
    //         draw.arrow()
    //             .start(c)
    //             .end(c + r.with_magnitude(40.0))
    //             .stroke_weight(model.ray_width)
    //             .color(model.palette.get_first(model.scheme_id, model.color_off));
    //     }
    // }

    draw.to_frame(app, &frame).unwrap();
}

fn key_pressed(app: &App, model: &mut Model, key: Key) {
    match key {
        Key::S => match app.window(model.main_window) {
            Some(window) => {
                window.capture_frame(app.time.to_string() + ".png");
            }
            None => {}
        },
        _other_key => {}
    }
}

fn ui_event(_app: &App, model: &mut Model, _event: WindowEvent) {
    let ui = &mut model.ui.set_widgets();
    {
        for value in gui::slider(model.wall_width as f32, 1.0, 15.0)
            .top_left_with_margin(10.0)
            .label("wall width")
            .set(model.ids.wall_width, ui)
        {
            model.wall_width = value;
        }

        for value in gui::slider(model.wall_split as f32, 0.0, 1.0)
            .label("wall split")
            .set(model.ids.wall_split, ui)
        {
            model.wall_split = value;
        }

        for value in gui::slider(model.wall_padding as f32, 0.2, 0.02)
            .label("wall padding")
            .set(model.ids.wall_padding, ui)
        {
            model.wall_padding = value;
        }

        for value in gui::slider(model.hole_pct as f32, 0.0, 0.9)
            .label("hole")
            .set(model.ids.hole_pct, ui)
        {
            model.hole_pct = value;
        }

        for value in gui::slider(model.hole_n as f32, 0.0, 6.0)
            .label("hole_n")
            .set(model.ids.hole_n, ui)
        {
            model.hole_n = value as usize;
        }

        for value in gui::slider(model.n_caster as f32, 1.0, 50.0)
            .label("n_caster ")
            .set(model.ids.n_caster, ui)
        {
            model.n_caster = value as u32;
        }

        for value in gui::slider(model.tile_count_w as f32, 1.0, 20.0)
            .label("tile_count_w")
            .set(model.ids.tile_count_w, ui)
        {
            model.tile_count_w = value as u32;
        }

        for _click in gui::button()
            .label("Regenerate Walls")
            .set(model.ids.button, ui)
        {
            let win = _app.window(model.main_window).unwrap().rect();
            make_walls(
                &mut model.walls,
                &win,
                model.tile_count_w,
                model.wall_split,
                model.wall_padding,
                model.hole_pct,
                model.hole_n,
            );
            make_raycasters(&mut model.rays, &win, model.tile_count_w, model.n_caster)
        }

        for value in gui::slider(model.collision_radius as f32, 0.0, 185.0)
            .label("collision radius")
            .set(model.ids.collision_radius, ui)
        {
            model.collision_radius = value;
        }

        for value in gui::slider(model.ray_width, 1.0, 10.0)
            .label("ray width")
            .set(model.ids.ray_width, ui)
        {
            model.ray_width = value;
        }
        for value in gui::slider(model.rays_prob as f32, 0.0, 1.0)
            .label("rays prob.")
            .set(model.ids.rays_prob, ui)
        {
            model.rays_prob = value;
        }
        for value in gui::slider(model.max_bounces as f32, 1.0, 400.0)
            .label("max_bounces")
            .set(model.ids.max_bounces, ui)
        {
            model.max_bounces = value as usize;
        }

        for value in gui::slider(model.clear_interval as f32, 5.0, 20.0)
            .label("clear_interval")
            .set(model.ids.clear_interval, ui)
        {
            model.clear_interval = value as usize;
        }

        for val in gui::slider(model.rotation, -PI, PI)
            .label("Rotation")
            .set(model.ids.rotation, ui)
        {
            model.rotation = val;
        }

        for value in gui::slider(model.scheme_id as f32, 0.0, 5.0)
            .label("scheme_id")
            .set(model.ids.scheme_id, ui)
        {
            model.scheme_id = value as usize;
        }

        for value in gui::slider(model.blend_id as f32, 0.0, 3.0)
            .label("blend_id")
            .set(model.ids.blend_id, ui)
        {
            model.blend_id = value as usize;
        }

        for value in gui::slider(model.color_off as f32, 0.0, 4.0)
            .label("color_off")
            .set(model.ids.color_off, ui)
        {
            model.color_off = value as usize;
        }

        for value in gui::slider(model.polygon_contour_weight, 1.0, 30.0)
            .label("polygon cont weight")
            .set(model.ids.polygon_contour_weight, ui)
        {
            model.polygon_contour_weight = value;
        }

        for v in gui::toggle(model.draw_polygon as bool)
            .label("Draw poly")
            .set(model.ids.draw_polygon, ui)
        {
            model.draw_polygon = v;
        }

        for v in gui::toggle(model.draw_arrows as bool)
            .label("Draw Arrows")
            .set(model.ids.draw_arrows, ui)
        {
            model.draw_arrows = v;
        }

        for v in gui::toggle(model.draw_tex_overlay as bool)
            .label("Draw Overlay")
            .set(model.ids.draw_tex_overlay, ui)
        {
            model.draw_tex_overlay = v;
        }

        for v in gui::toggle(model.animation as bool)
            .label("Animation")
            .set(model.ids.animation, ui)
        {
            model.animation = v;
        }

        for value in gui::slider(model.animation_speed as f32, 80.0, 0.01)
            .label("animation speed")
            .set(model.ids.animation_speed, ui)
        {
            model.animation_speed = value;
        }

        for v in gui::toggle(model.show_walls as bool)
            .label("Show wall")
            .set(model.ids.show_walls, ui)
        {
            model.show_walls = v;
        }
    }
}

fn ui_view(app: &App, model: &Model, frame: Frame) {
    model.ui.draw_to_frame_if_changed(app, &frame).unwrap();
}
