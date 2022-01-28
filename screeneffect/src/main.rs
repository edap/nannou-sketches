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
        .view(view)
        .build()
        .unwrap();
    let window = app.window(w_id).unwrap();


    let sample_count = app.window(w_id).unwrap().msaa_samples();

    // load the sahders
    let vs_desc = wgpu::include_wgsl!("shaders/vs.wgsl");
    // at the moment only sample_count 4 works.
    let fs_desc = match sample_count {
        1 => wgpu::include_wgsl!("shaders/fs.wgsl"),
        4 => wgpu::include_wgsl!("shaders/fs_msaa4.wgsl"),
        _ => wgpu::include_wgsl!("shaders/fs_msaa.wgsl"),
    };
    let effect = PostProcessingEffect::new(
        texture_size,
        sample_count,
        app.window(w_id).unwrap().device(),
        vs_desc,
        fs_desc,
    );

    Model {
        effect,
    }
}

fn update(app: &App, model: &mut Model, _update: Update) {
    // Because we draw in the texture, all the code that usually goes in the view method 
    // has to be moved into the update function.

    // VIEW
    // First, reset the `draw` state.
    let draw = &model.effect.draw;
    draw.reset();

    // Draw like we normally would in the `view`.
    draw.background().color(BLACK);
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
    model.effect.update(&window, &device);

}

// Draw the state of your `Model` into the given `Frame` here.
fn view(_app: &App, model: &Model, frame: Frame) {
    // Sample the texture and write it to the frame.
    model.effect.view(frame);
}
