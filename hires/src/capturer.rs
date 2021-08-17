use std::fmt::Binary;

use nannou::prelude::*;
use nannou::wgpu::Device;

pub struct Capturer {
    // The texture that we will draw to.
    pub texture: wgpu::Texture,
    // Create a `Draw` instance for drawing to our texture.
    pub draw: nannou::Draw,
    // The type used to render the `Draw` vertices to our texture.
    pub renderer: nannou::draw::Renderer,
    // The type used to capture the texture.
    pub texture_capturer: wgpu::TextureCapturer,
    // The type used to resize our texture to the window texture.
    pub texture_reshaper: wgpu::TextureReshaper,

    // where the the image are saved
    pub path: std::path::PathBuf,

    // save every frame
    pub is_recording: bool,
}

impl Capturer {
    pub fn new(texture_size: [u32; 2], window: &Window, device: &Device, path: std::path::PathBuf) -> Self{
    // Create our custom texture.
    let sample_count = window.msaa_samples();
    let texture = wgpu::TextureBuilder::new()
        .size(texture_size)
        // Our texture will be used as the RENDER_ATTACHMENT for our `Draw` render pass.
        // It will also be SAMPLED by the `TextureCapturer` and `TextureResizer`.
        .usage(wgpu::TextureUsage::RENDER_ATTACHMENT | wgpu::TextureUsage::SAMPLED)
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

        // Create the texture capturer.
        let texture_capturer = wgpu::TextureCapturer::default();

        // Create the texture reshaper.
        let texture_view = texture.view().build();
        let texture_sample_type = texture.sample_type();
        let dst_format = Frame::TEXTURE_FORMAT;
        let texture_reshaper = wgpu::TextureReshaper::new(
            device,
            &texture_view,
            sample_count,
            texture_sample_type,
            sample_count,
            dst_format,
        );

        let is_recording = true;

        // Make sure the directory where we will save images to exists.
        std::fs::create_dir_all(&path).unwrap();

        Capturer {
            texture,
            draw,
            renderer,
            texture_capturer,
            texture_reshaper,
            path,
            is_recording,
        }
    }

    pub fn update(&mut self, window: &Window, device: &Device, elapsed_frames: u64){
        let ce_desc = wgpu::CommandEncoderDescriptor {
            label: Some("texture renderer"),
        };
        let mut encoder = device.create_command_encoder(&ce_desc);
        self
        .renderer
        .render_to_texture(device, &mut encoder, &self.draw, &self.texture);

        // Take a snapshot of the texture. The capturer will do the following:
        //
        // 1. Resolve the texture to a non-multisampled texture if necessary.
        // 2. Convert the format to non-linear 8-bit sRGBA ready for image storage.
        // 3. Copy the result to a buffer ready to be mapped for reading.
        let snapshot = &self.texture_capturer.capture(device, &mut encoder, &self.texture);

        // Submit the commands for our drawing and texture capture to the GPU.
        window.swap_chain_queue().submit(Some(encoder.finish()));


        // Submit a function for writing our snapshot to a PNG.
        
        // NOTE: It is essential that the commands for capturing the snapshot are `submit`ted before we
        // attempt to read the snapshot - otherwise we will read a blank texture!
        let path = &self.path
            .join(elapsed_frames.to_string())
            .with_extension("png");
        // snapshot
        //     .read(move |result| {
        //         let image = result.expect("failed to map texture memory").to_owned();
        //         image
        //             .save(&path)
        //             .expect("failed to save texture to png image");
        //     })
        //     .unwrap();
    }

    pub fn startRecording(&mut self){
        //&self.is_recording = true;
    }

    pub fn view(){

    }

    fn record(){

    }

    pub fn take_screenshot(){

    }

    pub fn exit(){

    }
}