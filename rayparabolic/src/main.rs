pub const WIN_W: u32 = 970;
pub const WIN_H: u32 = 350;
pub const ENABLE_4K_CAPTURE: bool = false;

use edapx_colors::Palette;
use nannou::prelude::*;
use nannou_egui::{self, egui, Egui};
use rayon::prelude::*;
use scene::Material;
use wall_helper::change_surface_walls;
use std::env;
use std::path::PathBuf;
use std::path::Path;

//mod gui;
mod ray_light;
mod scene;
pub mod wraycaster;
use crate::scene::Element;
use crate::scene::SurfaceType;
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
use nannou_ray2d::BoundingVolume;

const EPSILON: f32 = 0.05;

// TODO

// Draw the polygon grouping the points by depth level.

fn main() {
    nannou::app(model).update(update).exit(exit).run();
}

struct Settings {
    tile_count_w: u32,
    n_caster: u32,
    raycaster_density: usize,
    rays_position_mode: usize,
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
}

struct Model {
    canvas_rect: geom::Rect,
    scene: Vec<Element>,
    rays: Vec<Wraycaster>,
    palette: Palette,
    settings: Settings,
    egui: Egui,
    capturer: Capturer,
    material: Material,
}

fn model(app: &App) -> Model {
    // we render on a 4k texture
    //let texture_size = [2160, 2700];
    //let texture_size = [1587, 2245];
    //let texture_size = [3_840, 2_160];
    let texture_size = [1920, 1080];
    //let texture_size = [2_160, 2_160];
    // Create the window, that is 4 times smaller than the texture

    let mut win_w = texture_size[0];
    let mut win_h = texture_size[1];

    if ENABLE_4K_CAPTURE {
        win_w = texture_size[0] / 4;
        win_h = texture_size[1] / 4;
    }
    
    // we also draw on a 4k canvas
    let canvas_rect = geom::Rect::from_w_h(texture_size[0] as f32, texture_size[1] as f32);

    let tile_count_w = 8;
    let main_window_id = app
        .new_window()
        .size(win_w, win_h)
        .view(view)
        .raw_event(raw_window_event)
        .key_pressed(key_pressed)
        .build()
        .unwrap();

    // set up the capturer
    let sample_count = app.window(main_window_id).unwrap().msaa_samples();
    //let path = capture_directory(app);
    let path = app.assets_path().unwrap();
    //let path = env::current_dir()?;
    let capturer = Capturer::new(
        texture_size,
        sample_count,
        app.window(main_window_id).unwrap().device(),
        path,
        false,
    );
    // end capturer

    // Create the UI.
    let ui_window = app
        .new_window()
        .title(app.exe_name().unwrap() + " controls")
        .size(WIN_W, WIN_H)
        .view(ui_view)
        //.event(ui_event)
        .raw_event(raw_window_event)
        .key_pressed(key_pressed)
        .build()
        .unwrap();

    // initialize the fields of the model
    let mut scene: Vec<Element> = Vec::new();
    let mut rays: Vec<Wraycaster> = Vec::new();
    let settings = Settings {
        tile_count_w: 8,
        n_caster: 2,
        raycaster_density: 6,
        rays_position_mode: 1,
        ray_width: 3.0,
        rays_prob: 0.8,
        wall_width: 2.0,
        wall_split: 1.0,
        hole_pct: 0.25,
        hole_n: 2,
        wall_padding: 0.07,
        collision_radius: 3.0,
        rotation: 0.0,
        scheme_id: 5,
        max_bounces: 4,
        blend_id: 0,
        color_off: 4,
        palette_alpha: 0.9,
        light_color_pct: 0.5,
        show_walls: true,
        clean_bg: true,
        transparent_bg: true,
        draw_arrows: false,
        draw_rays: true,
        draw_not_colliding_rays: false,
        animation: false,
        animation_mode: 0,
        animation_speed: 0.5,
        animation_time: 0.8,
        draw_polygon: true,
        draw_polygon_mode: 1,
        polygon_contour_weight: 1.0,
        clear_interval: 14,
    };

    let palette = Palette::new();

    let max_depth = 4;

    let material = Material::default();
    make_walls(
        &mut scene,
        &canvas_rect,
        tile_count_w,
        settings.wall_split,
        settings.wall_padding,
        settings.hole_pct,
        settings.hole_n,
        palette.get_first(settings.scheme_id, settings.color_off),
        palette.get_second(settings.scheme_id, settings.color_off),
        &material,
    );
    make_raycasters(
        &mut rays,
        &canvas_rect,
        tile_count_w,
        settings.n_caster,
        max_depth,
        settings.raycaster_density,
        &scene,
        settings.rays_position_mode,
        settings.rays_prob,
    );
    // walls: & Vec<Curve>,
    // rays_position_mode: usize,
    // rays_probability: f32,

    let window = app.window(main_window_id).unwrap();
    let egui = Egui::from_window(&window);
    let the_model = Model {
        egui,
        canvas_rect,
        scene,
        settings,
        rays,
        palette,
        capturer,
        material,
    };
    //ui_event(&app, &mut the_model, WindowEvent::Focused);
    the_model
}

fn update(app: &App, model: &mut Model, update: Update) {
    // we call draw only if the redraw value is set to true.
    // only in the gui it is possible to set it to true
    let mut redraw = false;

    // egui
    let egui = &mut model.egui;
    let settings = &mut model.settings;

    let canvas_rect = model.canvas_rect;
    let mut material = &mut model.material;
    let palette = &model.palette;
    let mut rays = &mut model.rays;
    let mut scene = &mut model.scene;

    egui.set_elapsed_time(update.since_start);
    let ctx = egui.begin_frame();

    egui::SidePanel::left("Scene").show(&ctx, |ui| {
        ui.heading("Scene");
        ui.horizontal(|ui| {
            ui.label("n holes:");
            ui.add(egui::Slider::new(&mut settings.hole_n, 1..=10));
        });
        ui.horizontal(|ui| {
            ui.label("hole %:");
            ui.add(egui::Slider::new(&mut settings.hole_pct, 0.0..=0.9));
        });
        ui.horizontal(|ui| {
            ui.label("tiles count:");
            ui.add(egui::Slider::new(&mut settings.tile_count_w, 1..=20));
        });
        ui.horizontal(|ui| {
            ui.label("wall_split:");
            ui.add(egui::Slider::new(&mut settings.wall_split, 0.0..=1.0));
        });
        ui.horizontal(|ui| {
            ui.label("wall_padding:");
            ui.add(egui::Slider::new(&mut settings.wall_padding, 0.02..=0.2));
        });
        ui.horizontal(|ui| {
            ui.label("tiles count:");
            ui.add(egui::Slider::new(&mut settings.tile_count_w, 1..=20));
        });

        ui.heading("Wall Style");
        ui.horizontal(|ui| {
            ui.label("show walls");
            ui.checkbox(&mut settings.show_walls, "");
        });
        ui.horizontal(|ui| {
            ui.label("walls width");
            ui.add(egui::Slider::new(&mut settings.wall_width, 1.0..=15.0));
        });

        ui.horizontal(|ui| {
            ui.label("material");
            if ui
                .add(egui::RadioButton::new(
                    material.surface == SurfaceType::Diffuse,
                    "Diffuse",
                ))
                .clicked()
            {
                material.surface = SurfaceType::Diffuse;
                change_surface_walls(&mut scene, &material.surface)
            }
            if ui
                .add(egui::RadioButton::new(
                    material.surface
                        == SurfaceType::Reflective {
                            reflectivity: (1.0),
                        },
                    "Reflective",
                ))
                .clicked()
            {
                material.surface = SurfaceType::Reflective {
                    reflectivity: (1.0),
                }
            }
            if ui
                .add(egui::RadioButton::new(
                    material.surface
                        == SurfaceType::ReflectiveAndRefractive {
                            reflectivity: (1.0),
                            ior: (1.4),
                        },
                    "ReflectiveAndRefractive",
                ))
                .clicked()
            {
                material.surface = SurfaceType::ReflectiveAndRefractive {
                    reflectivity: (1.0),
                    ior: (1.4),
                }
            }
        });

        if ui.add(egui::Button::new("Regenerate Walls")).clicked() {
            regenerate_scene_and_rays(
                &mut rays,
                &mut scene,
                settings,
                canvas_rect,
                material,
                palette,
            );
        };

        // if ui.button("Click each year").clicked() {
        //     self.age += 1;
        // }
        //ui.label(format!("Hello '{}', age {}", self.name, self.age));
    });

    egui::SidePanel::left("Raycaster").show(&ctx, |ui| {
        ui.heading("Rays");
        ui.horizontal(|ui| {
            ui.label("N. raycasters:");
            ui.add(egui::Slider::new(&mut settings.n_caster, 1..=6));
        });
        ui.horizontal(|ui| {
            ui.label("raycaster density:");
            ui.add(egui::Slider::new(&mut settings.raycaster_density, 1..=36));
        });

        ui.horizontal(|ui| {
            ui.label("rays position mode:");
            ui.add(egui::Slider::new(&mut settings.rays_position_mode, 0..=1));
        });
        ui.horizontal(|ui| {
            ui.label("collision_radius:");
            ui.add(egui::Slider::new(
                &mut settings.collision_radius,
                0.0..=185.0,
            ));
        });
        ui.horizontal(|ui| {
            ui.label("rays position mode:");
            ui.add(egui::Slider::new(&mut settings.rays_position_mode, 0..=1));
        });
        ui.horizontal(|ui| {
            ui.label("ray width:");
            ui.add(egui::Slider::new(&mut settings.ray_width, 0.5..=10.0));
        });
        ui.horizontal(|ui| {
            ui.label("rays prob:");
            ui.add(egui::Slider::new(&mut settings.rays_prob, 0.0..=1.0));
        });
        ui.horizontal(|ui| {
            ui.label("max bounces:");
            ui.add(egui::Slider::new(&mut settings.max_bounces, 1..=6));
        });

        ui.horizontal(|ui| {
            ui.label("clear interval:");
            ui.add(egui::Slider::new(&mut settings.clear_interval, 5..=20));
        });

        ui.horizontal(|ui| {
            ui.label("rotation:");
            ui.add(egui::Slider::new(&mut settings.rotation, -PI..=PI));
        });

        ui.heading("Ray Style");
        ui.horizontal(|ui| {
            ui.label("draw rays");
            ui.checkbox(&mut settings.draw_rays, "");
        });
        ui.horizontal(|ui| {
            ui.label("draw not collyding rays");
            ui.checkbox(&mut settings.draw_not_colliding_rays, "");
        });
        ui.horizontal(|ui| {
            ui.label("draw arrows");
            ui.checkbox(&mut settings.draw_arrows, "");
        });
    });

    egui::SidePanel::left("Style").show(&ctx, |ui| {
        ui.heading("Animation");
        ui.horizontal(|ui| {
            ui.label("enable animation");
            ui.checkbox(&mut settings.animation, "");
        });
        ui.horizontal(|ui| {
            ui.label("animation mode");
            ui.add(egui::Slider::new(&mut settings.animation_mode, 0..=1));
        });


        ui.heading("General Colors");
        ui.horizontal(|ui| {
            ui.label("scheme id");
            ui.add(egui::Slider::new(&mut settings.scheme_id, 0..=5));
            if ui.input().pointer.any_released() {
                change_color_walls(
                    scene,
                    palette.get_first(settings.scheme_id, settings.color_off),
                    palette.get_second(settings.scheme_id, settings.color_off),
                );
            }
        });

        ui.horizontal(|ui| {
            ui.label("blend mode");
            ui.add(egui::Slider::new(&mut settings.blend_id, 0..=3));
        });

        ui.horizontal(|ui| {
            ui.label("color offset");
            ui.add(egui::Slider::new(&mut settings.color_off, 0..=4));
            if ui.input().pointer.any_released() {
                change_color_walls(
                    scene,
                    palette.get_first(settings.scheme_id, settings.color_off),
                    palette.get_second(settings.scheme_id, settings.color_off),
                );
            }
        });

        ui.horizontal(|ui| {
            ui.label("light color %");
            ui.add(egui::Slider::new(&mut settings.light_color_pct, 0.0..=1.0));
            if ui.input().pointer.any_released() {
                change_color_walls(
                    scene,
                    palette.get_first(settings.scheme_id, settings.color_off),
                    palette.get_second(settings.scheme_id, settings.color_off),
                );
            }
        });

        ui.horizontal(|ui| {
            ui.label("palette alpha %");
            ui.add(egui::Slider::new(&mut settings.palette_alpha, 0.0..=1.0));
            if ui.input().pointer.any_released() {
                change_color_walls(
                    scene,
                    palette.get_first(settings.scheme_id, settings.color_off),
                    palette.get_second(settings.scheme_id, settings.color_off),
                );
            }
        });

        ui.heading("Polygon Style");
        ui.horizontal(|ui| {
            ui.label("draw polygon");
            ui.checkbox(&mut settings.draw_polygon, "");
        });
        ui.horizontal(|ui| {
            ui.label("polygon contour weight");
            ui.add(egui::Slider::new(
                &mut settings.polygon_contour_weight,
                0.5..=30.0,
            ));
        });
        ui.horizontal(|ui| {
            ui.label("draw polygon mode");
            ui.add(egui::Slider::new(&mut settings.draw_polygon_mode, 0..=2));
        });

        ui.heading("Background Style");
        ui.horizontal(|ui| {
            ui.label("clean bg");
            ui.checkbox(&mut settings.clean_bg, "");
        });
        ui.horizontal(|ui| {
            ui.label("transparent bg");
            ui.checkbox(&mut settings.transparent_bg, "");
        });

        //redraw if any of those was touched
        if ui.input().pointer.any_released() {
            redraw = true;
        }
    });

    if model.settings.animation | redraw {
        // Use the frame number to animate, ensuring we get a constant update time.
        let elapsed_frames = app.main_window().elapsed_frames();
        let time = elapsed_frames as f32 / 60.0;
        // let time = app.time;

        let rot = model.settings.rotation;
        let anim = model.settings.animation;
        let anim_speed = model.settings.animation_speed;
        let scene = &model.scene;
        let canvas_rect = model.canvas_rect;
        let animation_mode = model.settings.animation_mode;

        if model.settings.animation {
            // Animate raycaster
            model
                .rays
                .par_iter_mut()
                .for_each(|r| r.animate(&canvas_rect, anim_speed, animation_mode, time))
        }

        model
            .rays
            .par_iter_mut()
            .for_each(|ray| ray.collide(rot, anim, anim_speed, time, scene, canvas_rect));

        // Because we draw in the texture, all the code that usually goes in the view method has to be moved into the update
        // function.

        // VIEW
        // First, reset the `draw` state.
        let d = &model.capturer.draw;
        d.reset();
        let blends = [BLEND_NORMAL, BLEND_ADD, BLEND_SUBTRACT, BLEND_LIGHTEST];
        let draw = d.color_blend(blends[model.settings.blend_id].clone());

        if model.settings.transparent_bg {
            let mut color = model
                .palette
                .get_fifth(model.settings.scheme_id, model.settings.color_off);
            color.alpha = 0.0;
            draw.background().color(color);
        }

        if model.settings.clean_bg && !model.settings.transparent_bg {
            let mut color = model
                .palette
                .get_fifth(model.settings.scheme_id, model.settings.color_off);
            color.alpha = 1.0;
            draw.background().color(color);
        }

        if model.settings.show_walls {
            for element in model.scene.iter() {
                element.draw(&draw, &model.settings.wall_width);
                // Debug bounding volume
                // if let Some(c) = element.bounding_volume() {
                //     match c {
                //         BoundingVolume::Circle { position, radius } => {
                //             draw.ellipse()
                //                 .no_fill()
                //                 .x_y(position.x, position.y)
                //                 .w_h(radius * 2.0, radius * 2.0)
                //                 .color(element.material().coloration)
                //                 .stroke_weight(model.wall_width);
                //         }
                //         _ => {}
                //     }
                // }
            }
        }

        for r in &model.rays {
            if model.settings.draw_polygon {
                r.draw_polygon(
                    &draw,
                    model.settings.polygon_contour_weight,
                    model.settings.ray_width,
                    model.settings.draw_not_colliding_rays,
                    model.settings.draw_polygon_mode,
                );
            }

            if model.settings.draw_arrows {
                r.draw_arrows(&draw, model.settings.ray_width);
            }

            if model.settings.draw_rays {
                r.draw_rays(
                    &draw,
                    model.settings.ray_width,
                    model.settings.draw_not_colliding_rays,
                );
            }
        }

        // Render our drawing to the texture.
        let window = app.main_window();
        let device = window.device();
        model.capturer.update(&window, &device, elapsed_frames);
        redraw = false;
    }
}

fn view(_app: &App, model: &Model, frame: Frame) {
    model.capturer.view(frame);
}

fn key_pressed(app: &App, model: &mut Model, key: Key) {
    match key {
        Key::S => {
            // let window = app.main_window();
            // let device = window.device();
            // model
            //     .capturer
            //     .update(&window, &device, app.main_window().elapsed_frames());
            model.capturer.take_screenshot();
        }
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

fn ui_view(_app: &App, model: &Model, frame: Frame) {
    model.egui.draw_to_frame(&frame).unwrap();
}

// The directory where we'll save the frames.
fn capture_directory(app: &App) -> std::path::PathBuf {
//         env::current_dir()
//    // app.project_path()
//         .expect("could not locate project_path")
//         .join(app.exe_name().unwrap())

       Path::new("/home/dapx/").to_path_buf()
}
// Wait for capture to finish.
fn exit(app: &App, model: Model) {
    println!("Waiting for PNG writing to complete...");
    let window = app.main_window();
    let device = window.device();
    model.capturer.exit(&device);
    println!("Done!");
}

fn regenerate_scene_and_rays(
    rays: &mut Vec<Wraycaster>,
    scene: &mut Vec<Element>,
    settings: &mut Settings,
    canvas_rect: geom::Rect,
    material: &Material,
    palette: &Palette,
) {
    make_walls(
        scene,
        &canvas_rect,
        settings.tile_count_w,
        settings.wall_split,
        settings.wall_padding,
        settings.hole_pct,
        settings.hole_n,
        palette.get_first(settings.scheme_id, settings.color_off),
        palette.get_second(settings.scheme_id, settings.color_off),
        &material,
    );

    make_raycasters(
        rays,
        &canvas_rect,
        settings.tile_count_w,
        settings.n_caster,
        settings.max_bounces,
        settings.raycaster_density,
        scene,
        settings.rays_position_mode,
        settings.rays_prob,
    )
}

fn raw_window_event(_app: &App, model: &mut Model, event: &nannou::winit::event::WindowEvent) {
    // Let egui handle things like keyboard and mouse input.
    model.egui.handle_raw_event(event);
}
