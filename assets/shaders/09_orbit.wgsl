#import bevy_pbr::{
    mesh_functions,
    mesh_view_bindings::globals,
    forward_io::{Vertex, VertexOutput},
}
@group(#{MATERIAL_BIND_GROUP}) @binding(0) var<uniform> earth_radius: f32;
@group(#{MATERIAL_BIND_GROUP}) @binding(1) var<uniform> earth_color: vec4<f32>;
@group(#{MATERIAL_BIND_GROUP}) @binding(2) var<uniform> moon_offset: f32;
@group(#{MATERIAL_BIND_GROUP}) @binding(3) var<uniform> moon_radius: f32;
@group(#{MATERIAL_BIND_GROUP}) @binding(4) var<uniform> moon_color: vec4<f32>;
@group(#{MATERIAL_BIND_GROUP}) @binding(5) var<uniform> satellite_offset: f32;
@group(#{MATERIAL_BIND_GROUP}) @binding(6) var<uniform> satellite_radius: f32;
@group(#{MATERIAL_BIND_GROUP}) @binding(7) var<uniform> satellite_color: vec4<f32>;

const PI: f32 = acos(-1.0);

@fragment
fn fragment(
    mesh: VertexOutput,
) -> @location(0) vec4<f32> {
    let uv = 2.0 * mesh.uv - vec2f(1.0);
    let earth_mask = step(length(uv), earth_radius);

    let moon_center = vec2f(cos(globals.time) * moon_offset, sin(globals.time) * moon_offset);
    let moon_mask = step(distance(uv, moon_center), moon_radius);
    let earth_moon = mix(earth_color.rgb, moon_color.rgb, moon_mask);

    let satellite_center = vec2f(cos(-globals.time * 2.0) * satellite_offset, sin(-globals.time * 2.0) * satellite_offset) + moon_center;
    let satellite_mask = step(distance(uv, satellite_center), satellite_radius);
    let earth_moon_satellite = mix(earth_moon, satellite_color.rgb, satellite_mask);

    // return vec4f(earth_moon, 1.0);
    return vec4f(earth_moon_satellite, earth_mask + moon_mask + satellite_mask);
}