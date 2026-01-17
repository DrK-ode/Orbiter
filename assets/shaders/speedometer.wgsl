#import bevy_ui::ui_vertex_output::UiVertexOutput
#import bevy_render::{
    color_operations::hsv_to_rgb,
    maths::{PI, PI_2, HALF_PI},
    }

const ri = 0.5;
const ro = 0.9;
const dial_angle_gap = radians(60.);
const dial_angle_end = PI_2 - dial_angle_gap;
const rot = -HALF_PI + 0.5 * dial_angle_gap;
const rotation = mat2x2<f32>(cos(rot), sin(rot), -sin(rot), cos(rot));
const r_edge: f32 = 0.05;
const bkg_color = vec4(0.);
const min_hue = radians(0.);
const max_hue = radians(240.);
const indicator_extra_r = (ro -ri) * 0.1;
const indicator_angle_width = radians(30.);
const indicator_extra_a = indicator_angle_width * 0.1;

@group(1) @binding(0) var<uniform> relative_speed: f32;

fn calc_hue(min_value: f32, max_value: f32, value: f32, min_hue: f32, max_hue: f32, coeff: f32) -> f32 {
    var hue = smoothstep(min_value, max_value, value);
    hue *= pow(coeff, hue - 1.);
    return mix(min_hue, max_hue, hue);
}

fn coordinate_transform(uv_in: vec2<f32>) -> vec2<f32> {
    var uv = 2. * (uv_in - vec2(0.5));
    uv.y = -uv.y;
    uv *= rotation;
    return uv;
}

fn calc_dial(r: f32, a: f32) -> vec4<f32> {
    let hue = calc_hue(0., dial_angle_end, a, min_hue, max_hue, 10.);
    let gradient_color = vec4(hsv_to_rgb(vec3(hue, 1., 1.)), 1.);

    let a_edge = r_edge / r;
    let radial_area = smoothstep(ri, ri + r_edge, r) - smoothstep(ro - r_edge, ro, r);
    let angle_area = smoothstep(0., a_edge, a) - smoothstep(dial_angle_end - a_edge, dial_angle_end, a);

    var color = mix(bkg_color, gradient_color, radial_area);
    color = mix(bkg_color, color, angle_area);

    return color;
}

fn calc_indicator(r: f32, a:f32, indicator_angle: f32, current_color: vec4<f32>) -> vec4<f32> {
    let ri = ri - indicator_extra_r;
    let ro = ro + indicator_extra_r;
    let half_indicator_angle_width = 0.5 * indicator_angle_width;
    let angle_end = indicator_angle + half_indicator_angle_width + indicator_extra_a;
    let angle_begin = indicator_angle - half_indicator_angle_width - indicator_extra_a;

    let a_edge = r_edge / r;
    let radial_area = smoothstep(ri , ri + r_edge, r) - smoothstep(ro - r_edge, ro, r);
    let angle_area = smoothstep(angle_begin, angle_begin + a_edge, a) - smoothstep(angle_end - a_edge, angle_end, a);

    let frosted_color = vec3(0.9, 0.9, 0.9);
    var glass_color = mix(current_color.rgb, frosted_color, 0.3);

    var color = mix(current_color, vec4(glass_color, 1.), radial_area);
    color = mix(current_color, color, angle_area);

    return color;
}

@fragment
fn fragment(in: UiVertexOutput) -> @location(0) vec4<f32> {
    let uv = coordinate_transform(in.uv);
    let r = length(uv);
    let a = (atan2(uv.y, uv.x) + PI_2) % PI_2;

    var color = calc_dial(r, a);
    let indicator_angle =  0.5 * indicator_angle_width + (1. - relative_speed) * (dial_angle_end - indicator_angle_width);
    color = calc_indicator(r, a, indicator_angle, color);

    return color;
}