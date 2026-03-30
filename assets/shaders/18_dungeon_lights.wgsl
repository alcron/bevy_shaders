#import bevy_pbr::{
    mesh_functions,
    mesh_view_bindings::globals,
    forward_io::{Vertex, VertexOutput, FragmentOutput},
}

@group(#{MATERIAL_BIND_GROUP}) @binding(0) var<uniform> character_position_x: f32;
@group(#{MATERIAL_BIND_GROUP}) @binding(1) var<uniform> character_position_y: f32;
@group(#{MATERIAL_BIND_GROUP}) @binding(2) var<uniform> light_source_positions: array<vec4<f32>, 2>;
@group(#{MATERIAL_BIND_GROUP}) @binding(3) var background_texture: texture_2d<f32>;
@group(#{MATERIAL_BIND_GROUP}) @binding(4) var background_sampler: sampler;

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
    // ----- fake light
    // let gray_background_color = vec3f(0.1, 0.1, 0.1);
    // let light_source_radius = 0.1;
    // let light_source_color = vec3f(1.0, 0.8, 0.6);
    // let dist = distance(vec2f(character_position_x, character_position_y), mesh.world_position.xy);
    // let intencity = smoothstep(light_source_radius, 0.0, dist);
    // let color = mix(vec3f(0.0), light_source_color, step(dist, light_source_radius)) * vec3f(intencity);

    // return FragmentOutput(vec4f(color, 1.0));
    // -----

    let full_brightness_radius = 0.12;
    let faded_brightness_radius = 0.2;
    let full_brightness_intencity = 0.5;
    let faded_brightness_intencity = 0.1;
    let pixel_size = 0.03;

    var brightness: f32 = 0.0;

    let pixel_position = (floor(mesh.world_position.xy / vec2f(pixel_size)) + vec2f(0.5)) * pixel_size;

    for (var i = 0u; i < 2u; i = i + 1u) {
        let light_pos = light_source_positions[i].xy;
        let dist = distance(pixel_position, light_pos);
        let full_brightness_mask = max(brightness, 1.0 - step(full_brightness_radius, dist));
        let faded_brightness_mask = max(brightness, 1.0 - step(faded_brightness_radius, dist));

        brightness = max(full_brightness_mask * full_brightness_intencity, faded_brightness_mask * faded_brightness_intencity);
    }

    let background_color = textureSample(background_texture, background_sampler, mesh.uv);

    // TODO: Check why top right circle light is brighter when itersecting with character light than the bottom left one.
    let world_distance = distance(pixel_position, vec2f(character_position_x, character_position_y));
    let full_brightness_mask = max(brightness, 1.0 - step(full_brightness_radius, world_distance));
    let faded_brightness_mask = max(brightness, 1.0 - step(faded_brightness_radius, world_distance));

    brightness = max(brightness, max(full_brightness_mask * full_brightness_intencity, faded_brightness_mask * faded_brightness_intencity));

    let color = mix(vec3f(0.0), background_color.rgb, brightness);

    return FragmentOutput(vec4f(color, background_color.a));
}