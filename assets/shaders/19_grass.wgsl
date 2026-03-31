#import bevy_pbr::{
    mesh_functions,
    mesh_view_bindings::globals,
    forward_io::{Vertex, VertexOutput, FragmentOutput},
}

@group(#{MATERIAL_BIND_GROUP}) @binding(0) var<uniform> character_position: vec2f;
@group(#{MATERIAL_BIND_GROUP}) @binding(2) var<uniform> grass_positions: array<vec4<f32>, 4>;
@group(#{MATERIAL_BIND_GROUP}) @binding(3) var grass_texture: texture_2d<f32>;
@group(#{MATERIAL_BIND_GROUP}) @binding(4) var grass_sampler: sampler;

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
    let max_distance = 0.08;
    let min_distance = 0.02;
    let max_blend = 0.6;
    let grass_size = 1.0 / 8.0; // each grass is 1/8th of plane
    let half_size = grass_size / 2.0;

    var color = vec4<f32>(0.0, 0.0, 0.0, 0.0);

    for (var i = 0u; i < 4u; i = i + 1u) {
        // Convert grass position from [-0.5, 0.5] to UV [0, 1]
        let center = grass_positions[i].xy + 0.5;
        // Compute distance from grass center (not per-fragment) so the whole quad rotates rigidly
        let grass_world_pos = vec2f(grass_positions[i].x, -grass_positions[i].y);
        let dist = distance(grass_world_pos, character_position);
        var falloff = clamp((max_distance - dist) / (max_distance - min_distance), 0.0, 1.0);
        falloff = smoothstep(0.0, 1.0, falloff * 0.5); // smoother easing
        let h_side = normalize(character_position - grass_world_pos).x;
        let angle = falloff * max_blend * h_side;

        let min_uv = center - half_size;
        let max_uv = center + half_size;

        // Rotate around bottom middle using rotation matrix
        let pivot_uv = vec2f(center.x, max_uv.y);
        let rot = mat2x2f(
            cos(angle), sin(angle),
            -sin(angle), cos(angle)
        );
        let rotated = rot * (mesh.uv - pivot_uv) + pivot_uv;

        // Remap rotated UV to [0, 1] within this grass quad
        let local_uv = (rotated - min_uv) / grass_size;
        let texture = textureSample(grass_texture, grass_sampler, local_uv);
        // Blend on top (last one wins if overlapping)
        color = select(
            color,
            mix(color, texture, texture.a),
            local_uv.x >= 0.0 && local_uv.x <= 1.0 &&
            local_uv.y >= 0.0 && local_uv.y <= 1.0
        );
    }

    let character_circle = 1.0 - step(0.03, distance(mesh.world_position.xy, character_position));
    color = mix(color, vec4f(1.0, 1.0, 1.0, 1.0), character_circle);

    return FragmentOutput(color);
}