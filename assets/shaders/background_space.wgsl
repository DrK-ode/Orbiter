#import bevy_pbr::{
    forward_io::{VertexOutput, FragmentOutput},
}
#import bevy_shader_utils::perlin_noise_2d::perlin_noise_2d

struct Material {
    min_z: f32,
    max_z: f32,
    zoom_factor: f32,
};

const noise_scale: f32 = 0.01;
const miasma_speed: f32 = 0.25;

@group(#{MATERIAL_BIND_GROUP}) @binding(101) var<uniform> material: Material;

@fragment
fn fragment(
    mesh: VertexOutput
) -> @location(0) vec4<f32> {
    let zoom = (1. + material.zoom_factor * (mesh.world_position.z - material.min_z) / (material.max_z - material.min_z));
    let world_position = vec2(mesh.world_position.x, -mesh.world_position.y);
    let clip_position = mesh.position.xy * (1. - 1. / (2. * mesh.uv));
    var scale = noise_scale * zoom;
    var noise_value = 0.0;
    var amplitude_sum = 0.0;
    var amplitude = 1.0;
    let attenuation = 0.5;
    for (var i: i32 = 0; i < 4; i++){
        noise_value += amplitude * perlin_noise_2d((scale * clip_position) + miasma_speed * world_position);
        scale *= 2.;
        amplitude_sum += amplitude;
        amplitude *= attenuation;
    }
    noise_value = (noise_value + amplitude_sum) / (2. * amplitude_sum);

    let color_a = vec3(0.0, 0.0, 0.0);
    let color_b = vec3(1.0, 0.0, 1.0);
    let mixed = mix(color_a, color_b, noise_value);
    return vec4(mixed, 1.0);
}