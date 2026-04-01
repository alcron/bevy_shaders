#import bevy_pbr::{
    mesh_functions,
    mesh_view_bindings::{globals, view, view_transmission_texture, view_transmission_sampler},
    forward_io::{Vertex, VertexOutput, FragmentOutput},
}

@group(#{MATERIAL_BIND_GROUP}) @binding(0) var image_texture: texture_2d<f32>;
@group(#{MATERIAL_BIND_GROUP}) @binding(1) var image_sampler: sampler;
@group(#{MATERIAL_BIND_GROUP}) @binding(2) var<uniform> warp_amount: f32;
@group(#{MATERIAL_BIND_GROUP}) @binding(3) var<uniform> lines_count: u32;
@group(#{MATERIAL_BIND_GROUP}) @binding(4) var<uniform> lines_opacity: f32;
@group(#{MATERIAL_BIND_GROUP}) @binding(5) var<uniform> lines_speed: f32;
@group(#{MATERIAL_BIND_GROUP}) @binding(6) var<uniform> vignette_strength: f32;

const PI: f32 = acos(-1.0);

fn warp(uv: vec2f) -> vec2f {
    let center_uv = 2.0 * uv - vec2f(1.0);
    let squared_distance = dot(center_uv, center_uv);
    let quad_distance = pow(squared_distance, 2.0);
    return uv + center_uv * quad_distance * warp_amount;
}

fn scanlines(uv: vec2f) -> vec3f {
    var s = sin(uv.y * f32(lines_count) + globals.time * lines_speed);
    // Remap from [-1, 1] to [0, 1]
    s = (s + 1.0) / 2.0;

    return vec3f(pow(s, lines_opacity));
}

fn vignette(uv: vec2f) -> f32 {
    let center_uv = 2.0 * uv - vec2f(1.0);
    let squared_distance = dot(center_uv, center_uv);
    return 1.0 - squared_distance * vignette_strength;
}

@vertex
fn vertex(
    vertex: Vertex,
) -> VertexOutput {
    var out: VertexOutput;

    var position = vertex.position;

    var model = mesh_functions::get_world_from_local(vertex.instance_index);
    out.world_position = mesh_functions::mesh_position_local_to_world(model, vec4<f32>(position, 1.0));
    out.position = mesh_functions::mesh_position_local_to_clip(model, vec4<f32>(position, 1.0));
    out.uv = vertex.uv;

    return out;
}

@fragment
fn fragment(
    mesh: VertexOutput,
) -> FragmentOutput {
    let warped_uv = warp(mesh.uv);

    let texture = textureSample(image_texture, image_sampler, warped_uv);
    let alpha = select(texture.a, 0.0, warped_uv.x < 0.0 || warped_uv.x > 1.0 || warped_uv.y < 0.0 || warped_uv.y > 1.0);
    let color = texture.rgb * scanlines(warped_uv) * vignette(warped_uv);

    return FragmentOutput(vec4f(color, alpha));
}