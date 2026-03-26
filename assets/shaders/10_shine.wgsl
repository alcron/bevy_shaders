#import bevy_pbr::{
    mesh_functions,
    mesh_view_bindings::globals,
    forward_io::{Vertex, VertexOutput},
}

@group(#{MATERIAL_BIND_GROUP}) @binding(0) var texture: texture_2d<f32>;
@group(#{MATERIAL_BIND_GROUP}) @binding(1) var texture_sampler: sampler;
@group(#{MATERIAL_BIND_GROUP}) @binding(2) var<uniform> angle: f32;
@group(#{MATERIAL_BIND_GROUP}) @binding(3) var<uniform> thickness: f32;
@group(#{MATERIAL_BIND_GROUP}) @binding(4) var<uniform> offset: vec2<f32>;

const PI: f32 = acos(-1.0);

@fragment
fn fragment(
    mesh: VertexOutput,
) -> @location(0) vec4<f32> {
    let texture = textureSample(texture, texture_sampler, mesh.uv);
    let uv = mesh.uv * 2.0 - vec2(1.0) + offset;
    let slope = tan(angle / 2.0);
    let line = uv.y - slope * uv.x;
    let distance_to_line = abs(line) / length(vec2(1.0, slope));
    let shine_mask = step(distance_to_line, thickness);
    let color = mix(texture.rgb, vec3f(1.0), shine_mask);

    return vec4f(color, texture.a);
}