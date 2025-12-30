#import bevy_pbr::{
    forward_io::{VertexOutput, FragmentOutput},
    mesh_view_bindings::{globals, view},
}

@group(#{MATERIAL_BIND_GROUP}) @binding(100) var<storage, read> noise_data: array<f32>;
@group(#{MATERIAL_BIND_GROUP}) @binding(101) var<uniform> noise_width: u32;
@group(#{MATERIAL_BIND_GROUP}) @binding(102) var<uniform> noise_height: u32;
@group(#{MATERIAL_BIND_GROUP}) @binding(103) var<uniform> star_color: vec4<f32>;

@fragment
fn fragment(
    mesh: VertexOutput,
) -> @location(0) vec4<f32> {
    let w = view.viewport[2];
    let h = view.viewport[3];
    let x : u32 = u32(mesh.uv.x * w + globals.time) % noise_width;
    let y : u32 = u32(mesh.uv.y * h - globals.time) % noise_height;
    var noise = noise_data[y* noise_width + x];
    // if noise * noise < 0.99 {
    //  noise = 0.;
    // }
    var color = vec4(noise, noise, noise, 1.);
    color *= star_color;
    return color;
}