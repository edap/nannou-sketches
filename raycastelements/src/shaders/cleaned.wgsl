struct FragmentOutput {
    [[location(0)]] out_color: vec4<f32>;
};

[[block]]
struct Data {
    time: f32;
};

[[group(0), binding(0)]]
var tex: texture_multisampled_2d<f32>;
[[group(0), binding(1)]]
var tex_sampler: sampler;
[[group(0), binding(2)]]
var<uniform> uniforms: Data;

[[stage(fragment)]]

fn main(
    [[location(0)]] tex_coords: vec2<f32>,
) -> FragmentOutput {
    
    let tex_size: vec2<i32> = textureDimensions(tex);

    // oscillate the x coord
    let waved_x = tex_coords.x + sin((uniforms.time + tex_coords.y) * 6.0) * 0.02;
    let tex_x: i32 = i32(f32(tex_size.x) * waved_x);
    let tex_y: i32 = i32(f32(tex_size.y) * tex_coords.y);
    // Get the integer tex coordinates.
    let itex_coords: vec2<i32> = vec2<i32>(tex_x, tex_y);

    // Average the pixels with its neighbours
    var color: vec4<f32> = vec4<f32>(0.0, 0.0, 0.0, 0.0);
    color = color + textureLoad(tex, itex_coords, 0);
    color = color + textureLoad(tex, itex_coords, 1);
    color = color + textureLoad(tex, itex_coords, 2);
    color = color + textureLoad(tex, itex_coords, 3);
    color = color * 0.25;
    if (itex_coords.x >= tex_size.x || itex_coords.x <= 0 || itex_coords.y >= tex_size.y || itex_coords.y <= 0) {
        color = textureLoad(tex, vec2<i32>(1,1), 0);
    }

    return FragmentOutput(color);
}