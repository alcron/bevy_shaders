#import bevy_pbr::{
    mesh_functions,
    mesh_view_bindings::globals,
    forward_io::{Vertex, VertexOutput},
}

@group(#{MATERIAL_BIND_GROUP}) @binding(0) var<uniform> diameter: f32;
@group(#{MATERIAL_BIND_GROUP}) @binding(1) var<uniform> width: f32;
@group(#{MATERIAL_BIND_GROUP}) @binding(2) var<uniform> progress: f32;
@group(#{MATERIAL_BIND_GROUP}) @binding(3) var<uniform> change_progress: f32;

const PI: f32 = acos(-1.0);

// TODO: Add animation controls using timers and restart it on space press like in the lesson
@fragment
fn fragment(
    mesh: VertexOutput,
) -> @location(0) vec4<f32> {
    let background_color = vec3<f32>(0.0, 0.0, 0.0);
    let main_color = vec3<f32>(0.0, 1.0, 0.0);
    let change_color = vec3<f32>(1.0);

    let uv = mesh.uv * 2.0 - vec2<f32>(1.0);
    let outer_circle_mask = step(length(uv), diameter);
    let inner_circle_mask = step(length(uv), diameter - width);
    let donut_mask = outer_circle_mask - inner_circle_mask;

    let dial_mask = step(1.0 - progress, (PI + atan2(uv.x, uv.y)) / (2.0 * PI));
    let dial_change_mask = step(1.0 - change_progress, (PI + atan2(uv.x, uv.y)) / (2.0 * PI));
    var color = mix(background_color, change_color, dial_change_mask);
    color = mix(color, main_color, dial_mask);

    return vec4f(color, donut_mask);
}