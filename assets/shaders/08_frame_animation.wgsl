#import bevy_pbr::{
    mesh_functions,
    mesh_view_bindings::globals,
    forward_io::{Vertex, VertexOutput},
}
@group(#{MATERIAL_BIND_GROUP}) @binding(0) var sprite_texture: texture_2d<f32>;
@group(#{MATERIAL_BIND_GROUP}) @binding(1) var sprite_texture_sampler: sampler;
@group(#{MATERIAL_BIND_GROUP}) @binding(2) var<uniform> speed: f32;

@fragment
fn fragment(
    mesh: VertexOutput,
) -> @location(0) vec4<f32> {
    let frames_in_sprite_count: u32 = 4;
    let frame_uv_ratio_x = 1.0 / f32(frames_in_sprite_count);
    let current_frame_index = floor((globals.time * speed) % f32(frames_in_sprite_count));
    let current_frame_uv_x = (mesh.uv.x + current_frame_index) * frame_uv_ratio_x;
    let texture = textureSample(sprite_texture, sprite_texture_sampler, vec2f(current_frame_uv_x, mesh.uv.y));

    return texture;
}