#import bevy_pbr::{
    mesh_functions,
    mesh_view_bindings::{globals, view, view_transmission_texture, view_transmission_sampler},
    forward_io::{Vertex, VertexOutput, FragmentOutput},
}

@group(#{MATERIAL_BIND_GROUP}) @binding(0) var<uniform> magnifier_offset: vec2f;
@group(#{MATERIAL_BIND_GROUP}) @binding(1) var<uniform> zoom: f32;
@group(#{MATERIAL_BIND_GROUP}) @binding(2) var<uniform> blur_intensity: f32;

const PI: f32 = acos(-1.0);

fn gaussian_blur(uv: vec2f, intensity: f32) -> vec4f {
    let pixel_size = 1.0 / vec2f(view.viewport.zw);
    let sigma = max(intensity * 10.0, 0.001);
    let spread = intensity * 5.0;

    var color = vec4f(0.0);
    var total_weight = 0.0;

    for (var x = -4; x <= 4; x++) {
        for (var y = -4; y <= 4; y++) {
            let offset = vec2f(f32(x), f32(y)) * pixel_size * spread;
            let d = f32(x * x + y * y);
            let weight = exp(-d / (2.0 * sigma * sigma));
            color += textureSampleLevel(
                view_transmission_texture,
                view_transmission_sampler,
                uv + offset,
                0.0
            ) * weight;
            total_weight += weight;
        }
    }

    return color / total_weight;
}

@vertex
fn vertex(
    vertex: Vertex,
) -> VertexOutput {
    var out: VertexOutput;

    var position = vertex.position + vec3f(magnifier_offset.x, 0.0, -magnifier_offset.y);

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
    let screen_uv = mesh.position.xy / vec2f(view.viewport.zw);

    // Project the magnifier's world center to screen UV space
    // Model has rotation_x(PI/2): local (x, 0, z) -> world (x, -z, 0)
    // Vertex shader offsets local pos by (magnifier_offset.x, 0, -magnifier_offset.y)
    // So world center = (magnifier_offset.x, magnifier_offset.y, 0)
    let center_clip = view.clip_from_world * vec4f(magnifier_offset.x, magnifier_offset.y, 0.0, 1.0);
    let center = center_clip.xy / center_clip.w * vec2f(0.5, -0.5) + vec2f(0.5);
    let zoomed_uv = center + (screen_uv - center) * zoom;

    let color = gaussian_blur(zoomed_uv, blur_intensity);

    return FragmentOutput(color);
}