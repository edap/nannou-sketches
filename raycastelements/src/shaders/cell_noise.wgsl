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

///// Iinigo quilez gradient noise
fn random2(st:vec2<f32>) -> f32{
    let res = vec2<f32>( dot(st.xy,vec2<f32>(127.1,311.7)),
              dot(st.xy,vec2<f32>(269.5,183.3)) );
    
    return -1.0 + 2.0 * fract( sin( dot( res.xy, vec2<f32>(12.9898,78.233) ) ) * 43758.5453123);
}

// 2d randome the book of shaders
fn random2d(p:vec2<f32>) -> vec2<f32>{
    return fract(sin(vec2<f32>(dot(p.xy,vec2<f32>(127.1,311.7)),dot(p.xy,vec2<f32>(269.5,183.3))))*43758.5453);
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

// Gradient Noise by Inigo Quilez - iq/2013
// https://www.shadertoy.com/view/lsf3WH
fn gnoise(st:vec2<f32>) -> f32{
    let i = floor(st.xy);
    let f = fract(st.xy);
	
	let u = f*f*(3.0-(2.0*f));

    return mix( mix( random2( i + vec2<f32>(0.0,0.0) ), 
                     random2( i + vec2<f32>(1.0,0.0) ), u.x),
                mix( random2( i + vec2<f32>(0.0,1.0) ), 
                     random2( i + vec2<f32>(1.0,1.0) ), u.x), u.y);
}



[[stage(fragment)]]

fn main(
    [[location(0)]] tex_coords: vec2<f32>,
) -> FragmentOutput {
    // Get the integer tex coordinates.
    let tex_size: vec2<i32> = textureDimensions(tex);

    //let tex_x: i32 = i32(f32(tex_size.x) * tex_coords.x);


    // 1) let's tile the space
    // Scale
    let scaled_tex_coords = tex_coords * 3.0;


    // Tile the space
    let i_st = floor(scaled_tex_coords);
    let f_st = fract(scaled_tex_coords);

    // 2) We will use the tile coordinates (stored in the integer coordinate, i_st)
    // to construct a random position of a point.
    // remember to copy the random.glsl file from our repo
    var m_dist:f32 = 1.;  // minimum distance
    for (var y :i32 = -1; y <= 1; y= y+1) {
        for (var x:i32 = -1; x <= 1; x = x+1) {
            let neighbor = vec2<f32>(f32(x),f32(y));

            // Random position from current + neighbor place in the grid
            let point = random2d((i_st + neighbor));

			// 5) Animate the point
            //point = 0.5 + 0.5*sin(uniform.time + 6.2831*point);

			// Vector between the pixel and the point
            let diff = neighbor + point - f_st;

            // Distance to the point
            let dist = length(diff);

            // Keep the closer distance
            m_dist = min(m_dist, dist);
        }
    }

    let c = (m_dist * 2.0) - 1.0;
    let animated_c = c * sin(gnoise(vec2<f32>(uniforms.time*0.2))) * 0.14;

    //let tex = texture2D(u_texture_1,(gl_FragCoord.xy/ u_resolution.xy)+animated_c).xyz;





    let tex_x: i32 = i32(f32(tex_size.x) * (tex_coords.x + animated_c));
    let tex_y: i32 = i32(f32(tex_size.y) * (tex_coords.y + animated_c));



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

    //color = vec4<f32>(m_dist, 0.0, 1.0, 1.0);
    //color = vec4<f32>(vec3<f32>(tex_coords.x+c, tex_coords.y+c, 1.0), 1.0);

    //color = textureSample(tex, tex_sampler, tex_coords);

    return FragmentOutput(color);
}