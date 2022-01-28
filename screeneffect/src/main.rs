// A demonstration of drawing to a very large texture, capturing the texture in its original size
// as a PNG and displaying a down-scaled version of the image within the window each frame.

use nannou::prelude::*;
pub mod post_processing_effect;
pub use crate::post_processing_effect::PostProcessingEffect ;

fn main() {
    nannou::app(model).update(update).run();
}

struct Model {
    // The texture that we will draw to.
    effect: PostProcessingEffect
}

fn model(app: &App) -> Model {
    // Lets write to a 4K UHD texture.
    let texture_size = [800, 800];

    // Create the window.
    let [win_w, win_h] = [texture_size[0], texture_size[1]];
    let w_id = app
        .new_window()
        .size(win_w, win_h)
        .title("nannou")
        .key_pressed(key_pressed)
        .view(view)
        .build()
        .unwrap();
    let window = app.window(w_id).unwrap();

    // Retrieve the wgpu device.
    let device = window.device();

    // set up the capturer
    let sample_count = app.window(w_id).unwrap().msaa_samples();
    let path = capture_directory(app);
    let effect = PostProcessingEffect::new(
        texture_size,
        sample_count,
        app.window(w_id).unwrap().device(),
        path,
        false,
    );
    // end capturer



    Model {
        effect,
    }
}

fn update(app: &App, model: &mut Model, _update: Update) {
    // Because we draw in the texture, all the code that usually goes in the view method has to be moved into the update
    // function.

    // VIEW
    // First, reset the `draw` state.
    let draw = &model.effect.draw;
    draw.reset();


    // Draw like we normally would in the `view`.
    draw.background().color(BLACK);


    // we are not animating using time. But we should
    // Use the frame number to animate, ensuring we get a constant update time.
    let elapsed_frames = app.main_window().elapsed_frames();
    let time = elapsed_frames as f32 / 60.0;
    // let time = app.time;
    draw.ellipse()
        .x_y(time.sin() * 200.0, time.cos()* 200.0)
        .w_h(200.0, 200.0)
        .color(RED);

    // Render our drawing to the texture.
    let window = app.main_window();
    let device = window.device();
    model.effect.update(&window, &device, elapsed_frames);

}

// Draw the state of your `Model` into the given `Frame` here.
fn view(_app: &App, model: &Model, frame: Frame) {
    // Sample the texture and write it to the frame.
    model.effect.view(frame);
}

fn key_pressed(_app: &App, model: &mut Model, key: Key) {
    match key {
        Key::S => model.effect.take_screenshot(),
        Key::R => model.effect.start_recording(),
        Key::P => model.effect.stop_recording(),
        // Key::S => match app.window(model.main_window_id) {
        //     Some(window) => {
        //         window.capture_frame(app.time.to_string() + ".png");
        //     }
        //     None => {}
        // },
        _other_key => {}
    }
}

// The directory where we'll save the frames.
fn capture_directory(app: &App) -> std::path::PathBuf {
    app.project_path()
        .expect("could not locate project_path")
        .join(app.exe_name().unwrap())
}

fn exit(app: &App, model: Model) {
    println!("Waiting for PNG writing to complete...");
    let window = app.main_window();
    let device = window.device();
    model.effect.exit(&device);
    println!("Done!");
}
