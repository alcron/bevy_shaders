#import bevy_pbr::{
    mesh_functions,
    mesh_view_bindings::globals,
    forward_io::{Vertex, VertexOutput},
}

@group(#{MATERIAL_BIND_GROUP}) @binding(0) var<uniform> wave_height: f32;
@group(#{MATERIAL_BIND_GROUP}) @binding(1) var<uniform> amount: f32;
@group(#{MATERIAL_BIND_GROUP}) @binding(2) var<uniform> wave_speed: f32;
@group(#{MATERIAL_BIND_GROUP}) @binding(3) var<uniform> background_color: vec4<f32>;
@group(#{MATERIAL_BIND_GROUP}) @binding(4) var<uniform> front_wave_color: vec4<f32>;
@group(#{MATERIAL_BIND_GROUP}) @binding(5) var<uniform> back_wave_color: vec4<f32>;

@fragment
fn fragment(
    mesh: VertexOutput,
) -> @location(0) vec4<f32> {
    let circle_mask = step(distance(mesh.uv, vec2f(0.5)), 0.5);
    let front_wave = sin((mesh.uv.x + globals.time * 0.25) * wave_speed * 3.0) * wave_height * cos(globals.time) + amount;
    let front_wave_mask = step(front_wave, mesh.uv.y);
    let back_wave = cos((mesh.uv.x + globals.time * 0.1) * wave_speed * 2.6) * wave_height * cos(globals.time) + amount;
    let back_wave_mask = step(back_wave, mesh.uv.y);
    let waves_mask = max(front_wave_mask, back_wave_mask);
    let back_wave_delta = waves_mask - front_wave_mask;

    let front_wave_background_color = mix(background_color, front_wave_color, front_wave_mask);
    let color = mix(front_wave_background_color, back_wave_color, back_wave_delta);

    return vec4f(color.rgb, circle_mask);
}