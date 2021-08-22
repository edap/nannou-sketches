// A demonstration of drawing to a very large texture, capturing the texture in its original size
// as a PNG and displaying a down-scaled version of the image within the window each frame.

use nannou::prelude::*;
pub mod capturer;
pub use crate::capturer::Capturer;

fn main() {
    nannou::app(model).update(update).exit(exit).run();
}

struct Model {
    main_window_id: WindowId,
    capturer: Capturer,
}

fn model(app: &App) -> Model {
    // Lets write to a 4K UHD texture.
    let texture_size = [3_840, 2_160];

    // Create the window.
    let [win_w, win_h] = [texture_size[0] / 4, texture_size[1] / 4];
    let main_window_id = app
        .new_window()
        .size(win_w, win_h)
        .title("nannou")
        .view(view)
        .key_pressed(key_pressed)
        .build()
        .unwrap();

    // Retrieve the number of samples.
    let sample_count = app.window(main_window_id).unwrap().msaa_samples();

    // path where images are saved
    let path = capture_directory(app);
    let capturer = Capturer::new(
        texture_size,
        sample_count,
        app.window(main_window_id).unwrap().swap_chain_device(),
        path,
        false,
    );

    Model {
        capturer,
        main_window_id,
    }
}

fn update(app: &App, model: &mut Model, _update: Update) {
    // First, reset the `draw` state.
    let draw = &model.capturer.draw;
    draw.reset();

    // Create a `Rect` for our texture to help with drawing.
    let [w, h] = model.capturer.texture.size();
    let r = geom::Rect::from_w_h(w as f32, h as f32);

    // Use the frame number to animate, ensuring we get a constant update time.
    let elapsed_frames = app.main_window().elapsed_frames();
    let t = elapsed_frames as f32 / 60.0;

    // Draw like we normally would in the `view`.
    draw.background().color(BLACK);
    let n_points = 10;
    let weight = 8.0;
    let hz = 6.0;
    let vertices = (0..n_points)
        .map(|i| {
            let x = map_range(i, 0, n_points - 1, r.left(), r.right());
            let fract = i as f32 / n_points as f32;
            let amp = (t + fract * hz * TAU).sin();
            let y = map_range(amp, -1.0, 1.0, r.bottom() * 0.75, r.top() * 0.75);
            pt2(x, y)
        })
        .enumerate()
        .map(|(i, p)| {
            let fract = i as f32 / n_points as f32;
            let r = (t + fract) % 1.0;
            let g = (t + 1.0 - fract) % 1.0;
            let b = (t + 0.5 + fract) % 1.0;
            let rgba = srgba(r, g, b, 1.0);
            (p, rgba)
        });
    draw.polyline()
        .weight(weight)
        .join_round()
        .points_colored(vertices);

    // Draw frame number and size in bottom left.
    let string = format!("Frame {} - {:?}", elapsed_frames, [w, h]);
    let text = text(&string)
        .font_size(48)
        .left_justify()
        .align_bottom()
        .build(r.pad(r.h() * 0.05));
    draw.path().fill().color(WHITE).events(text.path_events());

    // Render our drawing to the texture.
    let window = app.main_window();
    let device = window.swap_chain_device();
    model.capturer.update(&window, &device, elapsed_frames);
}

// Draw the state of your `Model` into the given `Frame` here.
fn view(_app: &App, model: &Model, frame: Frame) {
    model.capturer.view(frame);
}

// Wait for capture to finish.
fn exit(app: &App, model: Model) {
    println!("Waiting for PNG writing to complete...");
    let window = app.main_window();
    let device = window.swap_chain_device();
    model.capturer.exit(&device);
    println!("Done!");
}

fn key_pressed(app: &App, model: &mut Model, key: Key) {
    match key {
        Key::S => match app.window(model.main_window_id) {
            Some(_window) => {
                model.capturer.take_screenshot();
            }
            None => {}
        },
        Key::R => match app.window(model.main_window_id) {
            Some(_window) => {
                model.capturer.start_recording();
            }
            None => {}
        },
        Key::E => match app.window(model.main_window_id) {
            Some(_window) => {
                model.capturer.stop_recording();
            }
            None => {}
        },
        _other_key => {}
    }
}

// The directory where we'll save the frames.
fn capture_directory(app: &App) -> std::path::PathBuf {
    app.project_path()
        .expect("could not locate project_path")
        .join(app.exe_name().unwrap())
}
