#import bevy_pbr::{
    mesh_functions,
    mesh_view_bindings::globals,
    forward_io::{Vertex, VertexOutput},
}

@group(#{MATERIAL_BIND_GROUP}) @binding(0) var texture: texture_2d<f32>;
@group(#{MATERIAL_BIND_GROUP}) @binding(1) var texture_sampler: sampler;
@group(#{MATERIAL_BIND_GROUP}) @binding(2) var<uniform> reload: f32;
@group(#{MATERIAL_BIND_GROUP}) @binding(3) var<uniform> charge: f32;
@group(#{MATERIAL_BIND_GROUP}) @binding(4) var<uniform> alpha: f32;

const PI: f32 = acos(-1.0);

// TODO: Add animation controls using timers and restart it on space press like in the lesson
@fragment
fn fragment(
    mesh: VertexOutput,
) -> @location(0) vec4<f32> {
    // TODO: pass through uniforms instead of hardcoding them in the shader
    let dial_color = vec3(1.0, 0.0, 0.0);
    let texture = textureSample(texture, texture_sampler, mesh.uv);
    let uv = mesh.uv * 2.0 - vec2(1.0);
    // var dial = (PI + atan2(uv.y, uv.x)) + PI / 2.0;
    // var dial = (PI + atan2(uv.y, uv.x)) + PI;
    var dial = (PI + atan2(uv.y, uv.x)) - PI / 2.0;
    dial = (dial + 2.0 * PI) % (2.0 * PI);
    let dial_mask = step(reload * 2.0 * PI, dial);

    let circle_mask = step(length(uv), charge * 1.5 /* multiply by constante to cover whole area */);
    let main_mask = dial_mask * circle_mask;
    let color = mix(texture.rgb, dial_color, main_mask * alpha);

    // return vec4f(vec3f(dial_mask), texture.a);
    return vec4f(color, texture.a);
}