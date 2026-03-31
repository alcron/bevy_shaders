#import bevy_pbr::{
    mesh_functions,
    mesh_view_bindings::{globals, view},
    forward_io::{Vertex, VertexOutput, FragmentOutput},
}

@group(#{MATERIAL_BIND_GROUP}) @binding(0) var image_texture: texture_2d<f32>;
@group(#{MATERIAL_BIND_GROUP}) @binding(1) var image_sampler: sampler;

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

// TODO: Complex logic. Look through again to undestand better.
@fragment
fn fragment(
    mesh: VertexOutput,
) -> FragmentOutput {
    let color = textureSample(image_texture, image_sampler, mesh.uv);
    let color_inverse = vec3f(1.0, 1.0, 1.0) - color.rgb;
    let color_grayscale = vec3f(color.r);
    let screen_center = view.viewport.zw / 2.0;
    let pixel_size = 4.0;
    let pixelated_pos = floor(mesh.position.xy / pixel_size) * pixel_size + pixel_size / 2.0;
    let dist = distance(pixelated_pos, screen_center) / length(screen_center);
    let mask = 1.0 - step(0.2, dist);

    let final_color = mix(color.rgb, color_grayscale, mask);

    return FragmentOutput(vec4f(final_color, color.a));
}