#import bevy_pbr::{
    mesh_functions,
    mesh_view_bindings::globals,
    forward_io::{Vertex, VertexOutput},
}
@group(#{MATERIAL_BIND_GROUP}) @binding(0) var image_texture: texture_2d<f32>;
@group(#{MATERIAL_BIND_GROUP}) @binding(1) var image_sampler: sampler;
@group(#{MATERIAL_BIND_GROUP}) @binding(2) var<uniform> noise_color_intensity: f32;

fn random(uv: vec2f) -> f32 {
    return fract(sin(dot(uv, vec2f(12.9898, 78.233))) * 43758.5453);
}

@fragment
fn fragment(
    mesh: VertexOutput,
) -> @location(0) vec4<f32> {
    // ------- Old TV noise effect
    // let texture_color = textureSample(image_texture, image_sampler, mesh.uv);
    // let noise = random(mesh.uv * vec2f(globals.time * 0.001));
    // let color = mix(texture_color.rgb, vec3f(noise), noise_color_intensity);

    // return vec4f(color, texture_color.a);
    // -------



    // ------- Random color per grid cell
    let grid_size = 10.0;

    let grid_uv = floor(mesh.uv * grid_size);
    let r = random(grid_uv);
    let g = random(grid_uv + vec2f(1.0, 0.0));
    let b = random(grid_uv + vec2f(0.0, 1.0));

    return vec4f(vec3f(r, g, b), 1.0);
    // -------
}