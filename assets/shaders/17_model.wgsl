#import bevy_pbr::{
    mesh_functions,
    mesh_view_bindings::globals,
    forward_io::{Vertex, VertexOutput, FragmentOutput},
}

@group(#{MATERIAL_BIND_GROUP}) @binding(0) var image_texture: texture_2d<f32>;
@group(#{MATERIAL_BIND_GROUP}) @binding(1) var image_sampler: sampler;
@group(#{MATERIAL_BIND_GROUP}) @binding(2) var background_texture: texture_2d<f32>;
@group(#{MATERIAL_BIND_GROUP}) @binding(3) var background_sampler: sampler;
@group(#{MATERIAL_BIND_GROUP}) @binding(4) var<uniform> amount: f32;

const PI: f32 = acos(-1.0);

@vertex
fn vertex(
    vertex: Vertex,
) -> VertexOutput {
    var out: VertexOutput;

    var position = vertex.position;

    var model = mesh_functions::get_world_from_local(vertex.instance_index);
    out.world_position = mesh_functions::mesh_position_local_to_world(model, vec4<f32>(position, 1.0));
    out.position = mesh_functions::mesh_position_local_to_clip(model, vec4<f32>(position, 1.0));
    // out.world_normal = mesh_functions::mesh_normal_local_to_world(vertex.normal, vertex.instance_index);
    out.uv = vertex.uv;

    return out;
}

@fragment
fn fragment(
    mesh: VertexOutput,
) -> FragmentOutput {
    let circle_center = vec2f(0.0, 0.0);
    let circle_radius = 0.1;
    let distance_from_center = distance(mesh.world_position.xy, circle_center);
    let alpha = step(circle_radius, distance_from_center);
    //  TODO: Offset values for uv was found by trial and error. Later better undestand how it works.
    let background = textureSample(background_texture, background_sampler, mesh.world_position.xy * 0.1 + vec2f(0.5, 0.5));
    let texture = textureSample(image_texture, image_sampler, mesh.uv);
    let color = mix(texture, background, amount);

    return FragmentOutput(vec4f(color.rgb, texture.a));
}