pub const WIN_W: u32 = 600;
pub const WIN_H: u32 = 900;

use edapx_colors::Palette;
use nannou::prelude::*;
use nannou_egui::{self, egui, Egui};
use rayon::prelude::*;
use scene::Material;
use wall_helper::change_surface_walls;

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
// Add a bounding box for the curves.

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
    let texture_size = [2160, 2700];
    //let texture_size = [3_840, 2_160];
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
        .raw_event(raw_window_event)
        .key_pressed(key_pressed)
        .build()
        .unwrap();

    // set up the capturer
    let sample_count = app.window(main_window_id).unwrap().msaa_samples();
    let path = capture_directory(app);
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
    let mut settings = Settings {
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

    let light_color_pct: f32 = 0.5;
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

    let canvas_rect =model.canvas_rect;
    let material = &model.material;
    let palette = &model.palette;
    let mut rays = &mut model.rays;
    let mut scene = &mut model.scene;

    egui.set_elapsed_time(update.since_start);
    let ctx = egui.begin_frame();


    egui::SidePanel::left("Scene").show(&ctx, |ui| {
        ui.heading("My scene");
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

        // if ui.button("Click each year").clicked() {
        //     self.age += 1;
        // }
        //ui.label(format!("Hello '{}', age {}", self.name, self.age));
    });

    egui::SidePanel::left("Raycaster").show(&ctx, |ui| {
        ui.heading("Rays");
        ui.horizontal(|ui| {
            ui.label("N. raycasters:");
            ui.add(egui::Slider::new(&mut settings.n_caster, 1..=50));
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
            ui.add(egui::Slider::new(&mut settings.collision_radius, 0.0..=185.0));
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

        if ui.add(egui::Button::new("Regenerate Walls")).clicked() {
            regenerate_scene_and_rays(&mut rays, &mut scene, settings, canvas_rect, material,palette);
        }


        //ui.add(toggle.(settings.draw_rayss))
        //ui.add(egui::Button: &mut settings.draw_rays, "test");

        // ui.horizontal(|ui| {
        //     ui.label("Draw rays:");
        //     ui.add(egui::Ra::new(&mut settings.draw_rays));
        // });


        //ui.add(egui::Slider::new(&mut settings.n_caster, 0..=120).text("n raycaster"));


        // if ui.button("Click each year").clicked() {
        //     self.age += 1;
        // }
        //ui.label(format!("Hello '{}', age {}", self.name, self.age));







    });

    egui::SidePanel::left("Style").show(&ctx, |ui| {
        ui.heading("My style");
        ui.horizontal(|ui| {
           ui.label("Draw rays");
           ui.checkbox(&mut settings.draw_rays, "");
        });
        ui.horizontal(|ui| {
            ui.label("Draw not collyding rays");
            ui.checkbox(&mut settings.draw_not_colliding_rays, "");
        });
        ui.horizontal(|ui| {
            ui.label("animation");
            ui.checkbox(&mut settings.animation, "");
        });
        ui.horizontal(|ui| {
            ui.label("animation mode");
            ui.add(egui::Slider::new(&mut settings.animation_mode, 0..=1));
        });
        //ui.interact(rect, id, sense)

        // Check for drags:
 

        // TODO. come triggare l'interaction con la gui
        // egui/src/containers/collapsing_header.rs



//         for value in gui::slider(model.animation_speed as f32, 80.0, 0.01)
//             .label("animation speed")
//             .set(model.ids.animation_speed, ui)
//         {
//             model.animation_speed = value;
//         }
        // if let Some(interaction) = ui.input().pointer.interact_pos() {
        //     println!("pos:{:?}", interaction);
        // }
        //redraw if any of those was touched
        if ui.input().pointer.any_released(){
            redraw = true;
            //println!("f:{:?}",app.main_window().elapsed_frames());
        }
    });

    egui::CentralPanel::default().show(&ctx, |ui| {
        ui.heading("My egui Application");
        // ui.horizontal(|ui| {
        //     ui.label("Your name: ");
        //     //ui.text_edit_singleline(&mut self.name);
        // });
        ui.label("Rotation:");
        ui.add(egui::Slider::new(&mut settings.rotation, 0.0..=360.0));
        // if ui.button("Click each year").clicked() {
        //     self.age += 1;
        // }
        //ui.label(format!("Hello '{}', age {}", self.name, self.age));
    });


    // egui::Window::new("Settings").show(&ctx, |ui| {
    //     // Resolution slider
    //     ui.label("Resolution:");
    //     ui.add(egui::Slider::new(&mut settings.n_caster, 1..=6));

    //     // Scale slider
    //     ui.label("Scale:");
    //     ui.add(egui::Slider::new(&mut settings.tile_count_w, 1..=20));

    //     // Rotation slider
    //     ui.label("Rotation:");
    //     ui.add(egui::Slider::new(&mut settings.rotation, 0.0..=360.0));

    //     // Random color button
    //     let clicked = ui.button("Random color").clicked();

    //     // if clicked {
    //     //     settings.color = rgb(random(), random(), random());
    //     // }
    // });



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
            let mut color = model.palette.get_fifth(model.settings.scheme_id, model.settings.color_off);
            color.alpha = 0.0;
            draw.background().color(color);
        }

        if model.settings.clean_bg && !model.settings.transparent_bg {
            let mut color = model.palette.get_fifth(model.settings.scheme_id, model.settings.color_off);
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
                r.draw_rays(&draw, model.settings.ray_width, model.settings.draw_not_colliding_rays);
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

// fn ui_event(_app: &App, model: &mut Model, _event: WindowEvent) {
//     let ui = &mut model.ui.set_widgets();
//     {
//         // WALLS
//         for value in gui::slider(model.wall_width as f32, 1.0, 15.0)
//             .top_left_with_margin(20.0)
//             .label("wall width")
//             .set(model.ids.wall_width, ui)
//         {
//             model.wall_width = value;
//         }





//         for v in gui::toggle(model.show_walls as bool)
//             .label("Show wall")
//             .set(model.ids.show_walls, ui)
//         {
//             model.show_walls = v;
//         }
//         for _click in gui::button()
//             .label("Walls Refl Refr")
//             .set(model.ids.button_refl_refr, ui)
//         {
//             let surface = SurfaceType::ReflectiveAndRefractive {
//                 reflectivity: 1.0,
//                 ior: 1.4,
//             };
//             model.material.surface = surface;
//             change_surface_walls(&mut model.scene, &surface)
//         }
//         for _click in gui::button()
//             .label("Walls Refl")
//             .set(model.ids.button_refl, ui)
//         {
//             let surface = SurfaceType::Reflective { reflectivity: 1.0 };
//             model.material.surface = surface;
//             change_surface_walls(&mut model.scene, &surface)
//         }
//         for _click in gui::button()
//             .label("Walls Diffuse")
//             .set(model.ids.button_diffuse, ui)
//         {
//             let surface = SurfaceType::Diffuse;
//             model.material.surface = surface;
//             change_surface_walls(&mut model.scene, &surface)
//         }

//         for _click in gui::button()
//             .label("Regenerate Walls")
//             .set(model.ids.button_regenerate, ui)
//         {
//             let canvas_rect = model.canvas_rect;
//             make_walls(
//                 &mut model.scene,
//                 &canvas_rect,
//                 model.tile_count_w,
//                 model.wall_split,
//                 model.wall_padding,
//                 model.hole_pct,
//                 model.hole_n,
//                 model.palette.get_first(model.scheme_id, model.color_off),
//                 model.palette.get_second(model.scheme_id, model.color_off),
//                 &model.material,
//             );

//             make_raycasters(
//                 &mut model.rays,
//                 &canvas_rect,
//                 model.tile_count_w,
//                 model.n_caster,
//                 model.max_bounces,
//                 model.raycaster_density,
//                 &model.scene,
//                 model.rays_position_mode,
//                 model.rays_prob,
//             )
//         }

//         // RAYCASTER
//         for value in gui::slider(model.n_caster as f32, 1.0, 50.0)
//             //.left(gui::PAD + gui::COL_W)
//             //.top_left_with_margin_on(model.ids.wall_width, 30.0)
//             .top_left_with_margins_on(model.ids.wall_width, 0.0, gui::PAD + gui::COL_W)
//             //.top_right_with_margins_on(model.ids.wall_width, gui::PAD, 15.0)
//             //.top_right_of(model.ids.wall_width)
//             .label("n_caster ")
//             .set(model.ids.n_caster, ui)
//         {
//             model.n_caster = value as u32;
//         }

//         for value in gui::slider(model.raycaster_density as f32, 1.0, 36.0)
//             .label("raycaster_density ")
//             .set(model.ids.raycaster_density, ui)
//         {
//             model.raycaster_density = value as usize;
//         }

//         for value in gui::slider(model.rays_position_mode as f32, 0.0, 1.0)
//             .label("raycaster pos mode")
//             .set(model.ids.rays_position_mode, ui)
//         {
//             model.rays_position_mode = value as usize;
//         }

//         for value in gui::slider(model.collision_radius as f32, 0.0, 185.0)
//             .label("collision radius")
//             .set(model.ids.collision_radius, ui)
//         {
//             model.collision_radius = value;
//         }

//         for value in gui::slider(model.ray_width, 0.5, 10.0)
//             .label("ray width")
//             .set(model.ids.ray_width, ui)
//         {
//             model.ray_width = value;
//         }
//         for value in gui::slider(model.rays_prob as f32, 0.0, 1.0)
//             .label("rays prob.")
//             .set(model.ids.rays_prob, ui)
//         {
//             model.rays_prob = value;
//         }
//         for value in gui::slider(model.max_bounces as f32, 1.0, 6.0)
//             .label("max_bounces")
//             .set(model.ids.max_bounces, ui)
//         {
//             model.max_bounces = value as usize;
//         }

//         for value in gui::slider(model.clear_interval as f32, 5.0, 20.0)
//             .label("clear_interval")
//             .set(model.ids.clear_interval, ui)
//         {
//             model.clear_interval = value as usize;
//         }

//         for val in gui::slider(model.rotation, -PI, PI)
//             .label("Rotation")
//             .set(model.ids.rotation, ui)
//         {
//             model.rotation = val;
//         }

//         for v in gui::toggle(model.draw_rays as bool)
//             .label("Draw rays")
//             .set(model.ids.draw_rays, ui)
//         {
//             model.draw_rays = v;
//         }

//         for v in gui::toggle(model.draw_not_colliding_rays as bool)
//             .label("Draw Not Colliding rays")
//             .set(model.ids.draw_not_colliding_rays, ui)
//         {
//             model.draw_not_colliding_rays = v;
//         }

//         // COLORS
//         for value in gui::slider(model.scheme_id as f32, 0.0, 5.0)
//             .top_left_with_margins_on(model.ids.n_caster, 0.0, gui::PAD + gui::COL_W)
//             .label("scheme_id")
//             .set(model.ids.scheme_id, ui)
//         {
//             model.scheme_id = value as usize;
//             change_color_walls(
//                 &mut model.scene,
//                 model.palette.get_first(model.scheme_id, model.color_off),
//                 model.palette.get_second(model.scheme_id, model.color_off),
//             );
//         }

//         for value in gui::slider(model.blend_id as f32, 0.0, 3.0)
//             .label("blend_id")
//             .set(model.ids.blend_id, ui)
//         {
//             model.blend_id = value as usize;
//         }

//         for value in gui::slider(model.color_off as f32, 0.0, 4.0)
//             .label("color_off")
//             .set(model.ids.color_off, ui)
//         {
//             model.color_off = value as usize;
//             change_color_walls(
//                 &mut model.scene,
//                 model.palette.get_first(model.scheme_id, model.color_off),
//                 model.palette.get_second(model.scheme_id, model.color_off),
//             );
//         }
//         for value in gui::slider(model.light_color_pct as f32, 0.0, 1.0)
//             .label("light color %")
//             .set(model.ids.light_color_pct, ui)
//         {
//             model.light_color_pct = value;
//         }

//         for value in gui::slider(model.palette_alpha as f32, 0.0, 1.0)
//             .label("palette_alpha")
//             .set(model.ids.palette_alpha, ui)
//         {
//             model.palette_alpha = value;
//             model.palette.set_alpha(value);
//             change_color_walls(
//                 &mut model.scene,
//                 model.palette.get_first(model.scheme_id, model.color_off),
//                 model.palette.get_second(model.scheme_id, model.color_off),
//             );
//         }

//         for value in gui::slider(model.polygon_contour_weight, 0.5, 30.0)
//             .label("polygon cont weight")
//             .set(model.ids.polygon_contour_weight, ui)
//         {
//             model.polygon_contour_weight = value;
//         }

//         for v in gui::toggle(model.clean_bg as bool)
//             .label("Draw Bg")
//             .set(model.ids.clean_bg, ui)
//         {
//             model.clean_bg = v;
//         }
//         for v in gui::toggle(model.transparent_bg as bool)
//             .label("Transparent Bg")
//             .set(model.ids.transparent_bg, ui)
//         {
//             model.transparent_bg = v;
//         }

//         for v in gui::toggle(model.draw_polygon as bool)
//             .label("Draw poly")
//             .set(model.ids.draw_polygon, ui)
//         {
//             model.draw_polygon = v;
//         }

//         for value in gui::slider(model.draw_polygon_mode as f32, 0.0, 2.0)
//             .label("Draw poly mode")
//             .set(model.ids.draw_polygon_mode, ui)
//         {
//             model.draw_polygon_mode = value as usize;
//         }

//         for v in gui::toggle(model.draw_arrows as bool)
//             .label("Draw Arrows")
//             .set(model.ids.draw_arrows, ui)
//         {
//             model.draw_arrows = v;
//         }


//     }
// }

fn ui_view(app: &App, model: &Model, frame: Frame) {
    model.egui.draw_to_frame(&frame).unwrap();
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
    let device = window.device();
    model.capturer.exit(&device);
    println!("Done!");
}

fn regenerate_scene_and_rays(rays: &mut Vec<Wraycaster>, scene: &mut Vec<Element>, settings: &mut Settings, canvas_rect: geom::Rect,material: &Material,palette: &Palette){
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
