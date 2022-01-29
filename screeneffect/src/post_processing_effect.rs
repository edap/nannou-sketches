// copied from https://github.com/nannou-org/nannou/blob/master/examples/draw/draw_capture_hi_res.rs

use nannou::prelude::*;
use nannou::wgpu::util::DeviceExt;
use nannou::wgpu::Device;

#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq, PartialOrd)]
struct Vertex {
    pub position: [f32; 2],
}

#[repr(C)]
#[derive(Copy, Clone)]
struct Uniforms {
    time: f32,
}

// TODO: those vertices can be emitted in the vertex shader.
const VERTICES: [Vertex; 4] = [
    Vertex {
        position: [-1.0, 1.0],
    },
    Vertex {
        position: [-1.0, -1.0],
    },
    Vertex {
        position: [1.0, 1.0],
    },
    Vertex {
        position: [1.0, -1.0],
    },
];

pub struct PostProcessingEffect {
    // The texture that we will draw to.
    pub texture: wgpu::Texture,
    // Create a `Draw` instance for drawing to our texture.
    pub draw: nannou::Draw,
    // The type used to render the `Draw` vertices to our texture.
    pub renderer: nannou::draw::Renderer,
    bind_group: wgpu::BindGroup,
    render_pipeline: wgpu::RenderPipeline,
    uniform_buffer: wgpu::Buffer,
    vertex_buffer: wgpu::Buffer,
}

// TODO, vertex shader as argument for a screenspace effect is unecessary.
impl PostProcessingEffect {
    pub fn new(
        texture_size: [u32; 2],
        src_sample_count: u32,
        device: &Device,
        vs_desc: wgpu::ShaderModuleDescriptor,
        fs_desc: wgpu::ShaderModuleDescriptor,
    ) -> Self {
        // Create our custom texture.
        let texture = wgpu::TextureBuilder::new()
            .size(texture_size)
            // Our texture will be used as the RENDER_ATTACHMENT for our `Draw` render pass.
            .usage(wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::TEXTURE_BINDING)
            // Use nannou's default multisampling sample count.
            .sample_count(src_sample_count)
            // Use a spacious 16-bit linear sRGBA format suitable for high quality drawing.
            .format(wgpu::TextureFormat::Rgba16Float)
            // Build it!
            .build(device);

        // Create our `Draw` instance and a renderer for it.
        let draw = nannou::Draw::new();
        let descriptor = texture.descriptor();
        let renderer =
            nannou::draw::RendererBuilder::new().build_from_texture_descriptor(device, descriptor);

        // Create the texture view
        let src_texture = texture.view().build();
        let src_sample_type = texture.sample_type();
        let dst_format = Frame::TEXTURE_FORMAT;

        // Verify if it has to be like this. But it sounds that the source texture (where I draw)
        // and the texture where nannoud draw the frame at the end, need to have the same sample_count
        // https://github.com/bevyengine/bevy/issues/3254
        let dst_sample_count = src_sample_count;

        let vs_mod = device.create_shader_module(&vs_desc);
        let fs_mod = device.create_shader_module(&fs_desc);

        // Create the sampler for sampling the source texture.
        let sampler_desc = wgpu::SamplerBuilder::new().into_descriptor();
        let sampler_filtering = wgpu::sampler_filtering(&sampler_desc);
        let sampler = device.create_sampler(&sampler_desc);

        // Create the render pipeline.
        let bind_group_layout =
            bind_group_layout(device, src_sample_count, src_sample_type, sampler_filtering);
        let pipeline_layout = pipeline_layout(device, &bind_group_layout);
        let render_pipeline = render_pipeline(
            device,
            &pipeline_layout,
            &vs_mod,
            &fs_mod,
            dst_sample_count,
            dst_format,
        );

        // create the buffer containing the uniforms (just time ATM)
        let uniforms = Uniforms { time: 0.0 };
        let uniforms_bytes = uniforms_as_bytes(&uniforms);
        let uniform_buffer = device.create_buffer_init(&BufferInitDescriptor {
            label: None,
            contents: &uniforms_bytes,
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });

        // Create the bind group.
        let bind_group = bind_group(
            device,
            &bind_group_layout,
            &src_texture,
            &sampler,
            &uniform_buffer,
        );

        // Create the vertex buffer. TODO, remove this and emit vertices in the vertex shader
        let vertices_bytes = vertices_as_bytes(&VERTICES[..]);
        let vertex_usage = wgpu::BufferUsages::VERTEX;
        let vertex_buffer = device.create_buffer_init(&BufferInitDescriptor {
            label: None,
            contents: vertices_bytes,
            usage: vertex_usage,
        });

        PostProcessingEffect {
            texture,
            draw,
            renderer,
            bind_group,
            render_pipeline,
            uniform_buffer,
            vertex_buffer,
        }
    }

    // receive an encoder and submit a render pass command for writing the src texture to the dst texture.
    pub fn encode_render_pass(
        &self,
        dst_texture: &wgpu::TextureViewHandle,
        encoder: &mut wgpu::CommandEncoder,
    ) {
        let mut render_pass = wgpu::RenderPassBuilder::new()
            .color_attachment(dst_texture, |color| color)
            .begin(encoder);
        render_pass.set_pipeline(&self.render_pipeline);
        render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
        render_pass.set_bind_group(0, &self.bind_group, &[]);
        let vertex_range = 0..VERTICES.len() as u32;
        let instance_range = 0..1;
        render_pass.draw(vertex_range, instance_range);
    }

    pub fn update(&mut self, window: &Window, device: &Device) {
        let ce_desc = wgpu::CommandEncoderDescriptor {
            label: Some("texture renderer"),
        };
        let mut encoder = device.create_command_encoder(&ce_desc);
        self.renderer
            .render_to_texture(device, &mut encoder, &self.draw, &self.texture);

        // Submit the commands for our drawing .
        window.queue().submit(Some(encoder.finish()));
    }

    pub fn update_buffer(&mut self, window: &Window, val: f32) {
        let u = Uniforms { time: val };
        window
            .queue()
            .write_buffer(&self.uniform_buffer, 0, uniforms_as_bytes(&u));
    }

    // Draw into the given `Frame`.
    pub fn view(&self, frame: Frame) {
        // Sample the texture and write it to the frame.
        let mut encoder = frame.command_encoder();
        self.encode_render_pass(frame.texture_view(), &mut *encoder);
    }
}

fn pipeline_layout(
    device: &wgpu::Device,
    bind_group_layout: &wgpu::BindGroupLayout,
) -> wgpu::PipelineLayout {
    let desc = wgpu::PipelineLayoutDescriptor {
        label: Some("nannou_Effect"),
        bind_group_layouts: &[&bind_group_layout],
        push_constant_ranges: &[],
    };
    device.create_pipeline_layout(&desc)
}

fn render_pipeline(
    device: &wgpu::Device,
    layout: &wgpu::PipelineLayout,
    vs_mod: &wgpu::ShaderModule,
    fs_mod: &wgpu::ShaderModule,
    dst_sample_count: u32,
    dst_format: wgpu::TextureFormat,
) -> wgpu::RenderPipeline {
    wgpu::RenderPipelineBuilder::from_layout(layout, vs_mod)
        .fragment_shader(fs_mod)
        .color_format(dst_format)
        .color_blend(wgpu::BlendComponent::REPLACE)
        .alpha_blend(wgpu::BlendComponent::REPLACE)
        .add_vertex_buffer::<Vertex>(&wgpu::vertex_attr_array![0 => Float32x2])
        .primitive_topology(wgpu::PrimitiveTopology::TriangleStrip)
        .sample_count(dst_sample_count)
        .build(device)
}

fn uniforms_as_bytes(uniforms: &Uniforms) -> &[u8] {
    unsafe { wgpu::bytes::from(uniforms) }
}

fn vertices_as_bytes(data: &[Vertex]) -> &[u8] {
    unsafe { wgpu::bytes::from_slice(data) }
}

fn bind_group_layout(
    device: &wgpu::Device,
    src_sample_count: u32,
    src_sample_type: wgpu::TextureSampleType,
    sampler_filtering: bool,
) -> wgpu::BindGroupLayout {
    let mut builder = wgpu::BindGroupLayoutBuilder::new()
        .texture(
            wgpu::ShaderStages::FRAGMENT,
            src_sample_count > 1,
            wgpu::TextureViewDimension::D2,
            src_sample_type,
        )
        .sampler(wgpu::ShaderStages::FRAGMENT, sampler_filtering);

    builder = builder.uniform_buffer(wgpu::ShaderStages::FRAGMENT, false);
    builder.build(device)
}
fn bind_group(
    device: &wgpu::Device,
    layout: &wgpu::BindGroupLayout,
    texture: &wgpu::TextureViewHandle,
    sampler: &wgpu::Sampler,
    uniform_buffer: &wgpu::Buffer,
) -> wgpu::BindGroup {
    let mut builder = wgpu::BindGroupBuilder::new()
        .texture_view(texture)
        .sampler(sampler);

    builder = builder.buffer::<Uniforms>(uniform_buffer, 0..1);
    builder.build(device, layout)
}
