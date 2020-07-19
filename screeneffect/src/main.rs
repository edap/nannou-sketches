// A demonstration of drawing to a very large texture, capturing the texture in its original size
// as a PNG and displaying a down-scaled version of the image within the window each frame.

use nannou::prelude::*;

fn main() {
    nannou::app(model).update(update).run();
}

struct Model {
    // The texture that we will draw to.
    texture: wgpu::Texture,
    // Create a `Draw` instance for drawing to our texture.
    draw: nannou::Draw,
    // The type used to render the `Draw` vertices to our texture.
    renderer: nannou::draw::Renderer,
    // The type used to resize our texture to the window texture.
    texture_reshaper: wgpu::TextureReshaper,
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

    // Retrieve the wgpu device.
    let device = window.swap_chain_device();

    // Create our custom texture.
    let sample_count = window.msaa_samples();
    let texture = wgpu::TextureBuilder::new()
        .size(texture_size)
        // Our texture will be used as the OUTPUT_ATTACHMENT for our `Draw` render pass.
        // It will also be SAMPLED by the `TextureResizer`.
        .usage(wgpu::TextureUsage::OUTPUT_ATTACHMENT | wgpu::TextureUsage::SAMPLED)
        // Use nannou's default multisampling sample count.
        .sample_count(sample_count)
        // Use a spacious 16-bit linear sRGBA format suitable for high quality drawing.
        .format(wgpu::TextureFormat::Rgba16Float)
        // Build it!
        .build(device);

    // Create our `Draw` instance and a renderer for it.
    let draw = nannou::Draw::new();
    let descriptor = texture.descriptor();
    let renderer =
        nannou::draw::RendererBuilder::new().build_from_texture_descriptor(device, descriptor);

    // Create the texture reshaper.
    let texture_view = texture.create_default_view();
    let texture_component_type = texture.component_type();
    let dst_format = Frame::TEXTURE_FORMAT;
    let texture_reshaper = wgpu::TextureReshaper::new(
        device,
        &texture_view,
        sample_count,
        texture_component_type,
        sample_count,
        dst_format,
    );

    Model {
        texture,
        draw,
        renderer,
        texture_reshaper,
    }
}

fn update(app: &App, model: &mut Model, _update: Update) {
    // First, reset the `draw` state.
    let draw = &model.draw;
    draw.reset();

    // Draw like we normally would in the `view`.
    draw.background().color(BLACK);
    draw.ellipse()
        .x_y(app.mouse.x, app.mouse.y)
        .w_h(200.0, 200.0)
        .color(RED);

    // Render our drawing to the texture.
    let window = app.main_window();
    let device = window.swap_chain_device();
    let ce_desc = wgpu::CommandEncoderDescriptor {
        label: Some("texture renderer"),
    };
    let mut encoder = device.create_command_encoder(&ce_desc);
    model
        .renderer
        .render_to_texture(device, &mut encoder, draw, &model.texture);
    // Submit the commands for our drawing and texture capture to the GPU.
    window.swap_chain_queue().submit(&[encoder.finish()]);
}

// Draw the state of your `Model` into the given `Frame` here.
fn view(_app: &App, model: &Model, frame: Frame) {
    // Sample the texture and write it to the frame.
    let mut encoder = frame.command_encoder();
    model
        .texture_reshaper
        .encode_render_pass(frame.texture_view(), &mut *encoder);
}
