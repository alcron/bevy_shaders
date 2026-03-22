#import bevy_pbr::{
    mesh_functions,
    mesh_view_bindings::globals,
    forward_io::{Vertex, VertexOutput},
}

@group(#{MATERIAL_BIND_GROUP}) @binding(0) var<uniform> material_color: vec4<f32>;
@group(#{MATERIAL_BIND_GROUP}) @binding(1) var material_noise_texture: texture_2d<f32>;
@group(#{MATERIAL_BIND_GROUP}) @binding(2) var material_noise_sampler: sampler;
@group(#{MATERIAL_BIND_GROUP}) @binding(3) var<uniform> shield_radius: f32;
@group(#{MATERIAL_BIND_GROUP}) @binding(4) var<uniform> shield_color_intensity: f32;

@vertex
fn vertex(vertex: Vertex) -> VertexOutput {
    var out: VertexOutput;

    // TODO: Make param
    var amplitude = 0.015;

    var position = vertex.position;
    let displacement = vec3(
        1.0 + sin(globals.time * 2.0),
        0.0,
        1.0 + sin(globals.time * 2.0 + 1.57 /* creates some offset*/));
    position += normalize(position) * displacement * amplitude;

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
) -> @location(0) vec4<f32> {
    let circle_gradient = distance(mesh.uv, vec2(0.5));

    let circle_main = step(circle_gradient, shield_radius);
    let circle_inner = smoothstep(shield_color_intensity, 1, circle_gradient);
    let main_shape = circle_main * circle_inner;

    // TODO: Make param
    let highlight_center = vec2(0.0, 0.5);
    let highlight_color = vec4(1.0, 0.5, 0.0, 1.0);
    let highlight_intinsity = 2.5;
    // 0 - 1
    let glow = 0.5 * 5.0;

    let highlight_gradient = 1.0 - distance(mesh.uv, highlight_center);
    let highlight_shape = pow(highlight_gradient, 3.0) * highlight_intinsity * main_shape;

    let noise_alpha = textureSample(material_noise_texture, material_noise_sampler, mesh.uv + vec2(globals.time * 0.1, 0.0)).r;

    let color = material_color * main_shape + highlight_color * highlight_shape * glow;

    return vec4(color.rgb, (main_shape + highlight_shape) * noise_alpha);
}