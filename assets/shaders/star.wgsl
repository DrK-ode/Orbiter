#import bevy_pbr::{
    forward_io::{VertexOutput, FragmentOutput},
    mesh_view_bindings::globals
}

//#import bevy_sprite::mesh2d_vertex_output::VertexOutput
//#import bevy_sprite::mesh2d_view_bindings::globals

// @group(#{MATERIAL_BIND_GROUP}) @binding(100) var material_color_texture: texture_2d<f32>;
// @group(#{MATERIAL_BIND_GROUP}) @binding(101) var material_color_sampler: sampler;

const iterations = 17;
const formuparam = 0.53;

const volsteps = 20;
const stepsize = 0.1;

const zoom  = 0.800;
const tile  = 0.850;
const speed = 0.0001;

const brightness = 0.0015;
const darkmatter = 0.300;
const distfading = 0.730;
const saturation = 0.850;

@fragment
fn fragment(
    mesh: VertexOutput,
) -> @location(0) vec4<f32> {
    let time = globals.time * speed + 25.;
    let dir = vec3(mesh.uv * zoom - 220.5, 10.);
    let view_from = 100. * vec3(sin(time), cos(time), 0.);
    var s = 0.1;
    var fade = 1.;
    var v = vec3(0.);
    for (var r = 0; r < volsteps; r++) {
        var p = view_from + s * dir * 0.5;
        p = abs(vec3(tile) - p % vec3(tile*2.)); // tiling fold
		var pa = 0.;
		var a = 0.;
		for (var i: i32 = 0; i < iterations; i++) { 
			p = abs(p) / dot(p,p) - formuparam; // the magic formula
			a += abs(length(p) - pa); // absolute sum of average change
			pa = length(p);
		}
		let dm = max(0., darkmatter - a * a * .001); //dark matter
		a *= a * a; // add contrast
		if (r > 6) {
		    fade *= 1. - dm; // dark matter, don't render near
		}
		//v+=vec3(dm,dm*.5,0.);
		v += fade;
		v += vec3(s, s * s, s * s * s * s) * a * brightness * fade; // coloring based on distance
		fade *= distfading; // distance fading
		s += stepsize;
	}
	v = mix(vec3(length(v)), v, saturation); //color adjust
	return vec4(v * .01, 1.);
}