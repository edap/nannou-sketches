// A demonstration of drawing to a very large texture, capturing the texture in its original size
// as a PNG and displaying a down-scaled version of the image within the window each frame.

use nannou::prelude::*;
pub mod post_processing_effect;
pub use crate::post_processing_effect::PostProcessingEffect;

fn main() {
    nannou::app(model).update(update).run();
}

struct Model {
    // The off-screen effect.
    effect: PostProcessingEffect,
}

fn model(app: &App) -> Model {
    let texture_size = [800, 800];

    // Create the window.
    let [win_w, win_h] = [texture_size[0], texture_size[1]];
    let w_id = app
        .new_window()
        .size(win_w, win_h)
        .title("nannou")
        .view(view)
        .build()
        .unwrap();

    let sample_count = app.window(w_id).unwrap().msaa_samples();

    // load the sahders
    let vs_desc = wgpu::include_wgsl!("shaders/vs.wgsl");
    // at the moment PostProcessingEffect works only with the default sample_count value, that is 4.
    let fs_desc = wgpu::include_wgsl!("shaders/fs_msaa4_noise.wgsl");

    let effect = PostProcessingEffect::new(
        texture_size,
        sample_count,
        app.window(w_id).unwrap().device(),
        vs_desc,
        fs_desc,
    );

    Model { effect }
}

fn update(app: &App, model: &mut Model, _update: Update) {
    // Because we draw in the texture, all the code that usually goes in the view method
    // has to be moved into the update function.

    // First, reset the `draw` state.
    let draw = &model.effect.draw;
    draw.reset();

    // Draw like we normally would in the `view`.
    draw.background().color(BLACK);
    let time = app.time;
    draw.ellipse()
        .x_y(time.sin() * 200.0, time.cos() * 200.0)
        .w_h(400.0, 400.0)
        .color(RED);

    // Render our drawing to the texture.
    let window = app.main_window();
    let device = window.device();
    model.effect.update_buffer(&window, app.time);
    model.effect.update(&window, &device);
}

// Draw your texture into the given `Frame` here.
fn view(_app: &App, model: &Model, frame: Frame) {
    // Sample the texture and write it to the frame.
    frame.texture_view();
    model.effect.view(frame);
}
