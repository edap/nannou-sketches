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


fn rand(st:vec2<f32>) -> f32 {
    return fract(sin(dot(st.xy,
                         vec2<f32>(12.9898,78.233)))*
        43758.5453123);
}
// 2D Noise based on Morgan McGuire @morgan3d
// https://www.shadertoy.com/view/4dS3Wd
fn noise(st:vec2<f32>) -> f32{
    let i = floor(st);
    let f = fract(st);

    let a = rand(i);
    let b = rand(i + vec2<f32>(1.0, 0.0));
    let c = rand(i + vec2<f32>(0.0, 1.0));
    let d = rand(i + vec2<f32>(1.0, 1.0));

    let u = f*f*(3.0-(2.0*f));
    return mix(a, b, u.x) +
            (c - a)* u.y * (1.0 - u.x) +
            (d - b) * u.x * u.y;
}

fn vectorField(uv: vec2<f32>) -> vec2<f32>{
  var res:vec2<f32> = uv;
  let n = noise(res*vec2<f32>(3.0));
  res.y = res.y - uniforms.time * 0.05;
  res = res + sin(res.yx * 40.) * 0.02;
  res = res + vec2<f32>(n);
  return res;
}

fn plot(val:f32, c:f32, t:f32) -> f32{
  let l = smoothStep(c,c-t,val);
  let r = smoothStep(c,c-t/5.,val);
  return r-l;
}


[[stage(fragment)]]

fn main(
    [[location(0)]] tex_coords: vec2<f32>,
) -> FragmentOutput {
    // Get the integer tex coordinates.
    let tex_size: vec2<i32> = textureDimensions(tex);
    let waved_x = tex_coords.x + sin((uniforms.time + tex_coords.y) * 19.0) * 0.02;

    //let tex_x: i32 = i32(f32(tex_size.x) * tex_coords.x);
    let tex_x: i32 = i32(f32(tex_size.x) * waved_x);
    let tex_y: i32 = i32(f32(tex_size.y) * tex_coords.y);
    let itex_coords: vec2<i32> = vec2<i32>(tex_x, tex_y);

    // Average the pixels with its neighbours
    var color: vec4<f32> = vec4<f32>(0.0, 0.0, 0.0, 0.0);
    color = color + textureLoad(tex, itex_coords, 0);
    color = color + textureLoad(tex, itex_coords, 1);
    color = color + textureLoad(tex, itex_coords, 2);
    color = color + textureLoad(tex, itex_coords, 3);
    color = color * 0.25;

    // This is wrong, to apply a pixel effect on a pixel already averaged
    // leads to aliasing. Anyway ...

    let thick = 0.3;
    var st:vec2<f32> = tex_coords;
    st = vec2<f32>(st.y, st.y * (st.x * 0.4));
    st = vectorField(st);

    let cell = 0.3;
    let modSt = st % vec2<f32>(cell);

    let x = plot(modSt.x, cell, thick);
    let y = plot(modSt.y, cell, thick);

    color.r = color.r * y;
    color.g = (1.0 - color.g) * smoothStep(1.3, .01,x+y);
    color.b = (1.0 - color.b) * smoothStep(1.9, .01,x+y);

    return FragmentOutput(color);
}