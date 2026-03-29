#import bevy_pbr::{
    mesh_functions,
    mesh_view_bindings::globals,
    forward_io::{Vertex, VertexOutput, FragmentOutput},
}

@group(#{MATERIAL_BIND_GROUP}) @binding(0) var image_texture: texture_2d<f32>;
@group(#{MATERIAL_BIND_GROUP}) @binding(1) var image_sampler: sampler;
@group(#{MATERIAL_BIND_GROUP}) @binding(2) var<uniform> offset_x: f32;
@group(#{MATERIAL_BIND_GROUP}) @binding(3) var<uniform> offset_y: f32;
@group(#{MATERIAL_BIND_GROUP}) @binding(4) var<uniform> scale: f32;
@group(#{MATERIAL_BIND_GROUP}) @binding(5) var<uniform> unified_rotation: f32;

const PI: f32 = acos(-1.0);

// TODO: Better orient plane for easier matrix transformation. Currently it's oriented in xy plane, so we need to apply offset to z axis and rotate around y axis to make it look like it's in xz plane
@vertex
fn vertex(
    vertex: Vertex,
) -> VertexOutput {
    var out: VertexOutput;

    var position = vertex.position;

    // in 3d space of bevy offset_y should be applied to z axis
    let translation_matrix = mat4x4<f32>(
        vec4<f32>(1.0, 0.0, 0.0, 0.0),
        vec4<f32>(0.0, 1.0, 0.0, 0.0),
        vec4<f32>(0.0, 0.0, 1.0, 0.0),
        vec4<f32>(offset_x, 0.0, offset_y, 1.0),
    );
    position = (translation_matrix * vec4<f32>(position, 1.0)).xyz;

    let scale_matrix = mat3x3<f32>(
        vec3<f32>(scale, 0.0, 0.0),
        vec3<f32>(0.0, scale, 0.0),
        vec3<f32>(0.0, 0.0, scale),
    );
    position = (scale_matrix * position);

    let angle = unified_rotation * PI ;

    let rotation_matrix = mat3x3<f32>(
        vec3<f32>(cos(angle), 0.0, -sin(angle)),
        vec3<f32>(0.0, 1.0, 0.0),
        vec3<f32>(sin(angle), 0.0, cos(angle)),
    );
    position = (rotation_matrix * position);

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