#import bevy_pbr::{
    mesh_functions,
    mesh_view_bindings::globals,
    forward_io::{Vertex, VertexOutput, FragmentOutput},
}

@group(#{MATERIAL_BIND_GROUP}) @binding(0) var image_texture: texture_2d<f32>;
@group(#{MATERIAL_BIND_GROUP}) @binding(1) var image_sampler: sampler;
@group(#{MATERIAL_BIND_GROUP}) @binding(2) var<uniform> rotation_x: f32;
@group(#{MATERIAL_BIND_GROUP}) @binding(3) var<uniform> rotation_y: f32;
@group(#{MATERIAL_BIND_GROUP}) @binding(4) var<uniform> rotation_z: f32;
@group(#{MATERIAL_BIND_GROUP}) @binding(5) var<uniform> pivot_x: f32;
@group(#{MATERIAL_BIND_GROUP}) @binding(6) var<uniform> pivot_y: f32;

const PI: f32 = acos(-1.0);

@vertex
fn vertex(
    vertex: Vertex,
) -> VertexOutput {
    var out: VertexOutput;

    var position = vertex.position;

    let c = cos(rotation_x / 2.0 * PI);
    let s = sin(rotation_x / 2.0 * PI);
    let rot_x = mat3x3f(
        vec3<f32>(1.0, 0.0, 0.0),
        vec3<f32>(0.0,   c,   s),
        vec3<f32>(0.0,  -s,   c),
    );

    let cy = cos(rotation_y / 2.0 * PI);
    let sy = sin(rotation_y / 2.0 * PI);
    let rot_y = mat3x3f(
        vec3<f32>( cy,  sy, 0.0),
        vec3<f32>(-sy,  cy, 0.0),
        vec3<f32>(0.0, 0.0, 1.0),
    );

    let cz = cos(rotation_z / 2.0 * PI);
    let sz = sin(rotation_z / 2.0 * PI);
    let rot_z = mat3x3f(
        vec3<f32>( cz, 0.0, -sz),
        vec3<f32>(0.0, 1.0, 0.0),
        vec3<f32>( sz, 0.0,  cz),
    );
    let conversion_matrix = rot_x * rot_y * rot_z;

    // TODO: Weird pivot point calculation, look through it again and undestand it better
    // let pivot = vec3f(pivot_y, 0.0, pivot_x);
    let pivot = vec3f(pivot_x, 0.0, pivot_y);
    let converted_pivot = conversion_matrix * pivot;
    let offset_by_pivot_position = position - pivot;

    position = conversion_matrix * offset_by_pivot_position + pivot;

    var model = mesh_functions::get_world_from_local(vertex.instance_index);
    // out.world_position = mesh_functions::mesh_position_local_to_world(model, vec4<f32>(position, 1.0));
    out.position = mesh_functions::mesh_position_local_to_clip(model, vec4<f32>(position, 1.0));
    // out.world_normal = mesh_functions::mesh_normal_local_to_world(vertex.normal, vertex.instance_index);
    out.uv = vertex.uv;

    return out;
}

@fragment
fn fragment(
    mesh: VertexOutput,
) -> FragmentOutput {
    let texture = textureSample(image_texture, image_sampler, mesh.uv);

    return FragmentOutput(texture);
}