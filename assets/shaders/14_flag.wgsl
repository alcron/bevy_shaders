#import bevy_pbr::{
    mesh_functions,
    mesh_view_bindings::globals,
    forward_io::{Vertex, VertexOutput, FragmentOutput},
}

@group(#{MATERIAL_BIND_GROUP}) @binding(0) var<uniform> speed: f32;
@group(#{MATERIAL_BIND_GROUP}) @binding(1) var<uniform> frequency_x: f32;
@group(#{MATERIAL_BIND_GROUP}) @binding(2) var<uniform> frequency_y: f32;
@group(#{MATERIAL_BIND_GROUP}) @binding(3) var<uniform> amplitude_x: f32;
@group(#{MATERIAL_BIND_GROUP}) @binding(4) var<uniform> amplitude_y: f32;
@group(#{MATERIAL_BIND_GROUP}) @binding(5) var<uniform> incline: f32;
@group(#{MATERIAL_BIND_GROUP}) @binding(6) var<uniform> max_incline: f32;

@vertex
fn vertex(
    vertex: Vertex,
) -> VertexOutput {
    var out: VertexOutput;

    var position = vertex.position;
    // position.z += sin(vertex.uv.x * globals.time * 5.0) * vertex.uv.x / 10.0;
    position.z += sin((vertex.uv.x + globals.time * speed) * frequency_y) * amplitude_y / 20.0 * vertex.uv.x;
    position.x += cos((vertex.uv.y - globals.time * speed) * frequency_x) * amplitude_x / 20.0 * vertex.uv.x;
    position.x -= incline * vertex.uv.y;

    var model = mesh_functions::get_world_from_local(vertex.instance_index);
    out.world_position = mesh_functions::mesh_position_local_to_world(model, vec4<f32>(position, 1.0));
    out.position = mesh_functions::mesh_position_local_to_clip(model, vec4<f32>(position, 1.0));
    out.world_normal = mesh_functions::mesh_normal_local_to_world(vertex.normal, vertex.instance_index);
    out.uv = vertex.uv;

    return out;
}

@fragment
fn fragment(
    mesh: VertexOutput,
) -> FragmentOutput {
    let circle_gradient = distance(mesh.uv, vec2(0.5));
    let center_mask = step(circle_gradient, 0.2);
    let color = mix(vec3f(1.0), vec3f(1.0, 0.0, 0.0), center_mask);

    return FragmentOutput(vec4f(color, 1.0));
}