use edapx_colors::Palette;
use nannou::prelude::*;
use nannou::ui::prelude::*;
use rayon::prelude::*;
use types::Material;
use wall_helper::change_surface_walls;

mod gui;
mod ray_light;
mod types;
pub mod wraycaster;
use crate::types::Curve;
use crate::types::SurfaceType;
mod mondrian;
pub use crate::mondrian::Square;
mod bouncing;
pub use crate::bouncing::BouncingRay2D;
mod ray_helper;
use crate::ray_helper::make_raycasters;
mod wall_helper;
use crate::wall_helper::change_color_walls;
use crate::wall_helper::make_walls;
mod raycaster;
pub use crate::wraycaster::Wraycaster;
pub mod capturer;
pub use crate::capturer::Capturer;
use ray2d::BoundingVolume;

const EPSILON: f32 = 0.05;

// TODO

// Draw the polygon grouping the points by depth level.
// Add a bounding box for the curves.

fn main() {
    nannou::app(model).update(update).exit(exit).run();
}

struct Model {
    canvas_rect: geom::Rect,
    walls: Vec<Curve>,
    tile_count_w: u32,
    n_caster: u32,
    raycaster_density: usize,
    rays: Vec<Wraycaster>,
    rays_position_mode: usize,
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
    palette_alpha: f32,
    light_color_pct: f32,
    palette: Palette,
    show_walls: bool,
    clean_bg: bool,
    transparent_bg: bool,
    draw_arrows: bool,
    draw_rays: bool,
    draw_not_colliding_rays: bool,
    animation: bool,
    animation_mode: usize,
    animation_speed: f32,
    animation_time: f32,
    draw_polygon: bool,
    draw_polygon_mode: usize,
    polygon_contour_weight: f32,
    clear_interval: usize,
    capturer: Capturer,
    material: Material,
}

fn model(app: &App) -> Model {
    // we render on a 4k texture
    let texture_size = [3_840, 2_160];
    //let texture_size = [2_160, 2_160];
    // Create the window, that is 4 times smaller than the texture
    let [win_w, win_h] = [texture_size[0] / 4, texture_size[1] / 4];
    // we also draw on a 4k canvas
    let canvas_rect = geom::Rect::from_w_h(texture_size[0] as f32, texture_size[1] as f32);

    let tile_count_w = 8;
    let main_window_id = app
        .new_window()
        .size(win_w, win_h)
        .view(view)
        .key_pressed(key_pressed)
        .build()
        .unwrap();

    // set up the capturer
    let sample_count = app.window(main_window_id).unwrap().msaa_samples();
    let path = capture_directory(app);
    let capturer = Capturer::new(
        texture_size,
        sample_count,
        app.window(main_window_id).unwrap().swap_chain_device(),
        path,
        false,
    );
    // end capturer

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

    // initialize the fields of the model
    let mut walls: Vec<Curve> = Vec::new();
    let mut rays: Vec<Wraycaster> = Vec::new();
    let ray_width = 3.0;
    let wall_width = 2.0;
    let wall_split = 1.0;
    let wall_padding = 0.07;
    let hole_pct = 0.25;
    let hole_n = 2;
    let n_caster = 2;
    let max_bounces = 4;
    let rotation = 0.0;
    let collision_radius = 3.0;
    let rays_prob = 0.8;
    let rays_position_mode = 1;

    let scheme_id = 5;
    let blend_id = 0;
    let color_off = 4;
    let light_color_pct: f32 = 0.5;
    let palette = Palette::new();
    let clear_interval = 14;
    let max_depth = 4;
    let raycaster_density = 6;
    let material = Material::default();
    make_walls(
        &mut walls,
        &canvas_rect,
        tile_count_w,
        wall_split,
        wall_padding,
        hole_pct,
        hole_n,
        palette.get_first(scheme_id, color_off),
        palette.get_second(scheme_id, color_off),
        &material,
    );
    make_raycasters(
        &mut rays,
        &canvas_rect,
        tile_count_w,
        n_caster,
        max_depth,
        raycaster_density,
        &walls,
        rays_position_mode,
        rays_prob,
    );
    // walls: & Vec<Curve>,
    // rays_position_mode: usize,
    // rays_probability: f32,
    let show_walls = true;
    let animation = false;
    let draw_arrows = false;
    let animation_speed = 2.0;
    let animation_mode = 0;
    let animation_time = 0.0;
    let draw_polygon = true;
    let draw_polygon_mode = 0;
    let draw_rays = false;
    let polygon_contour_weight = 5.0;
    let draw_not_colliding_rays = false;
    let clean_bg = true;
    let transparent_bg = false;
    let palette_alpha = 1.0;

    let mut the_model = Model {
        canvas_rect,
        walls,
        n_caster,
        raycaster_density,
        tile_count_w,
        rays,
        rays_position_mode,
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
        palette_alpha,
        color_off,
        light_color_pct,
        palette,
        show_walls,
        animation,
        animation_speed,
        animation_mode,
        animation_time,
        draw_polygon,
        draw_polygon_mode,
        draw_rays,
        draw_arrows,
        clean_bg,
        transparent_bg,
        polygon_contour_weight,
        draw_not_colliding_rays,
        clear_interval,
        capturer,
        material,
    };
    ui_event(&app, &mut the_model, WindowEvent::Focused);
    the_model
}

fn update(app: &App, model: &mut Model, _update: Update) {
    // Use the frame number to animate, ensuring we get a constant update time.
    let elapsed_frames = app.main_window().elapsed_frames();
    let time = elapsed_frames as f32 / 60.0;
    // let time = app.time;

    let rot = model.rotation;
    let anim = model.animation;
    let anim_speed = model.animation_speed;
    let walls = &model.walls;
    let canvas_rect = model.canvas_rect;
    let animation_mode = model.animation_mode;

    if model.animation {
        // Animate raycaster
        model
            .rays
            .par_iter_mut()
            .for_each(|r| r.animate(&canvas_rect, anim_speed, animation_mode, time))
    }

    model
        .rays
        .par_iter_mut()
        .for_each(|ray| ray.collide(rot, anim, anim_speed, time, walls, canvas_rect));

    // Because we draw in the texture, all the code that usually goes in the view method has to be moved into the update
    // function.

    // VIEW
    // First, reset the `draw` state.
    let d = &model.capturer.draw;
    d.reset();
    let blends = [BLEND_NORMAL, BLEND_ADD, BLEND_SUBTRACT, BLEND_LIGHTEST];
    let draw = d.color_blend(blends[model.blend_id].clone());

    if model.transparent_bg {
        let mut color = model.palette.get_fifth(model.scheme_id, model.color_off);
        color.alpha = 0.0;
        draw.background().color(color);
    }

    if model.clean_bg && !model.transparent_bg {
        let mut color = model.palette.get_fifth(model.scheme_id, model.color_off);
        color.alpha = 1.0;
        draw.background().color(color);
    }

    if model.show_walls {
        for curve in model.walls.iter() {
            //println!("{:?}", curve.points.len());
            draw.polyline()
                .weight(model.wall_width)
                .color(curve.material.coloration)
                .points(curve.points.clone());
            if let Some(c) = curve.bounding_volume {
                match c {
                    BoundingVolume::Circle { position, radius } => {
                        draw.ellipse()
                            .no_fill()
                            .x_y(position.x, position.y)
                            .w_h(radius * 2.0, radius * 2.0)
                            .color(curve.material.coloration)
                            .stroke_weight(model.wall_width);
                    }
                    _ => {}
                }
            }
        }
    }

    for r in &model.rays {
        if model.draw_polygon {
            r.draw_polygon(
                &draw,
                model.polygon_contour_weight,
                model.ray_width,
                model.draw_not_colliding_rays,
                model.draw_polygon_mode,
            );
        }

        if model.draw_arrows {
            r.draw_arrows(&draw, model.ray_width);
        }

        if model.draw_rays {
            r.draw_rays(&draw, model.ray_width, model.draw_not_colliding_rays);
        }
    }
    // Render our drawing to the texture.
    let window = app.main_window();
    let device = window.swap_chain_device();
    model.capturer.update(&window, &device, elapsed_frames);
}

fn view(_app: &App, model: &Model, frame: Frame) {
    model.capturer.view(frame);
}

fn key_pressed(_app: &App, model: &mut Model, key: Key) {
    match key {
        Key::S => model.capturer.take_screenshot(),
        Key::R => model.capturer.start_recording(),
        Key::P => model.capturer.stop_recording(),
        // Key::S => match app.window(model.main_window_id) {
        //     Some(window) => {
        //         window.capture_frame(app.time.to_string() + ".png");
        //     }
        //     None => {}
        // },
        _other_key => {}
    }
}

fn ui_event(_app: &App, model: &mut Model, _event: WindowEvent) {
    let ui = &mut model.ui.set_widgets();
    {
        // WALLS
        for value in gui::slider(model.wall_width as f32, 1.0, 15.0)
            .top_left_with_margin(20.0)
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
        for value in gui::slider(model.tile_count_w as f32, 1.0, 20.0)
            .label("tile_count_w")
            .set(model.ids.tile_count_w, ui)
        {
            model.tile_count_w = value as u32;
        }

        for v in gui::toggle(model.show_walls as bool)
            .label("Show wall")
            .set(model.ids.show_walls, ui)
        {
            model.show_walls = v;
        }
        for _click in gui::button()
            .label("Walls Refl Refr")
            .set(model.ids.button_refl_refr, ui)
        {
            let surface = SurfaceType::ReflectiveAndRefractive {
                reflectivity: 1.0,
                ior: 1.4,
            };
            model.material.surface = surface;
            change_surface_walls(&mut model.walls, &surface)
        }
        for _click in gui::button()
            .label("Walls Refl")
            .set(model.ids.button_refl, ui)
        {
            let surface = SurfaceType::Reflective { reflectivity: 1.0 };
            model.material.surface = surface;
            change_surface_walls(&mut model.walls, &surface)
        }
        for _click in gui::button()
            .label("Walls Diffuse")
            .set(model.ids.button_diffuse, ui)
        {
            let surface = SurfaceType::Diffuse;
            model.material.surface = surface;
            change_surface_walls(&mut model.walls, &surface)
        }

        for _click in gui::button()
            .label("Regenerate Walls")
            .set(model.ids.button_regenerate, ui)
        {
            let canvas_rect = model.canvas_rect;
            make_walls(
                &mut model.walls,
                &canvas_rect,
                model.tile_count_w,
                model.wall_split,
                model.wall_padding,
                model.hole_pct,
                model.hole_n,
                model.palette.get_first(model.scheme_id, model.color_off),
                model.palette.get_second(model.scheme_id, model.color_off),
                &model.material,
            );

            make_raycasters(
                &mut model.rays,
                &canvas_rect,
                model.tile_count_w,
                model.n_caster,
                model.max_bounces,
                model.raycaster_density,
                &model.walls,
                model.rays_position_mode,
                model.rays_prob,
            )
        }

        // RAYCASTER
        for value in gui::slider(model.n_caster as f32, 1.0, 50.0)
            //.left(gui::PAD + gui::COL_W)
            //.top_left_with_margin_on(model.ids.wall_width, 30.0)
            .top_left_with_margins_on(model.ids.wall_width, 0.0, gui::PAD + gui::COL_W)
            //.top_right_with_margins_on(model.ids.wall_width, gui::PAD, 15.0)
            //.top_right_of(model.ids.wall_width)
            .label("n_caster ")
            .set(model.ids.n_caster, ui)
        {
            model.n_caster = value as u32;
        }

        for value in gui::slider(model.raycaster_density as f32, 1.0, 36.0)
            .label("raycaster_density ")
            .set(model.ids.raycaster_density, ui)
        {
            model.raycaster_density = value as usize;
        }

        for value in gui::slider(model.rays_position_mode as f32, 0.0, 1.0)
            .label("raycaster pos mode")
            .set(model.ids.rays_position_mode, ui)
        {
            model.rays_position_mode = value as usize;
        }

        for value in gui::slider(model.collision_radius as f32, 0.0, 185.0)
            .label("collision radius")
            .set(model.ids.collision_radius, ui)
        {
            model.collision_radius = value;
        }

        for value in gui::slider(model.ray_width, 0.5, 10.0)
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
        for value in gui::slider(model.max_bounces as f32, 1.0, 6.0)
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

        for v in gui::toggle(model.draw_rays as bool)
            .label("Draw rays")
            .set(model.ids.draw_rays, ui)
        {
            model.draw_rays = v;
        }

        for v in gui::toggle(model.draw_not_colliding_rays as bool)
            .label("Draw Not Colliding rays")
            .set(model.ids.draw_not_colliding_rays, ui)
        {
            model.draw_not_colliding_rays = v;
        }

        // COLORS
        for value in gui::slider(model.scheme_id as f32, 0.0, 5.0)
            .top_left_with_margins_on(model.ids.n_caster, 0.0, gui::PAD + gui::COL_W)
            .label("scheme_id")
            .set(model.ids.scheme_id, ui)
        {
            model.scheme_id = value as usize;
            change_color_walls(
                &mut model.walls,
                model.palette.get_first(model.scheme_id, model.color_off),
                model.palette.get_second(model.scheme_id, model.color_off),
            );
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
            change_color_walls(
                &mut model.walls,
                model.palette.get_first(model.scheme_id, model.color_off),
                model.palette.get_second(model.scheme_id, model.color_off),
            );
        }
        for value in gui::slider(model.light_color_pct as f32, 0.0, 1.0)
            .label("light color %")
            .set(model.ids.light_color_pct, ui)
        {
            model.light_color_pct = value;
        }

        for value in gui::slider(model.palette_alpha as f32, 0.0, 1.0)
            .label("palette_alpha")
            .set(model.ids.palette_alpha, ui)
        {
            model.palette_alpha = value;
            model.palette.set_alpha(value);
            change_color_walls(
                &mut model.walls,
                model.palette.get_first(model.scheme_id, model.color_off),
                model.palette.get_second(model.scheme_id, model.color_off),
            );
        }

        for value in gui::slider(model.polygon_contour_weight, 0.5, 30.0)
            .label("polygon cont weight")
            .set(model.ids.polygon_contour_weight, ui)
        {
            model.polygon_contour_weight = value;
        }

        for v in gui::toggle(model.clean_bg as bool)
            .label("Draw Bg")
            .set(model.ids.clean_bg, ui)
        {
            model.clean_bg = v;
        }
        for v in gui::toggle(model.transparent_bg as bool)
            .label("Transparent Bg")
            .set(model.ids.transparent_bg, ui)
        {
            model.transparent_bg = v;
        }

        for v in gui::toggle(model.draw_polygon as bool)
            .label("Draw poly")
            .set(model.ids.draw_polygon, ui)
        {
            model.draw_polygon = v;
        }

        for value in gui::slider(model.draw_polygon_mode as f32, 0.0, 2.0)
            .label("Draw poly mode")
            .set(model.ids.draw_polygon_mode, ui)
        {
            model.draw_polygon_mode = value as usize;
        }

        for v in gui::toggle(model.draw_arrows as bool)
            .label("Draw Arrows")
            .set(model.ids.draw_arrows, ui)
        {
            model.draw_arrows = v;
        }

        for v in gui::toggle(model.animation as bool)
            .label("Animation")
            .set(model.ids.animation, ui)
        {
            model.animation = v;
        }

        for value in gui::slider(model.animation_mode as f32, 0.0, 1.0)
            .label("animation_mode")
            .set(model.ids.animation_mode, ui)
        {
            model.animation_mode = value as usize;
        }

        for value in gui::slider(model.animation_speed as f32, 80.0, 0.01)
            .label("animation speed")
            .set(model.ids.animation_speed, ui)
        {
            model.animation_speed = value;
        }
    }
}

fn ui_view(app: &App, model: &Model, frame: Frame) {
    model.ui.draw_to_frame_if_changed(app, &frame).unwrap();
}

// The directory where we'll save the frames.
fn capture_directory(app: &App) -> std::path::PathBuf {
    app.project_path()
        .expect("could not locate project_path")
        .join(app.exe_name().unwrap())
}
// Wait for capture to finish.
fn exit(app: &App, model: Model) {
    println!("Waiting for PNG writing to complete...");
    let window = app.main_window();
    let device = window.swap_chain_device();
    model.capturer.exit(&device);
    println!("Done!");
}
