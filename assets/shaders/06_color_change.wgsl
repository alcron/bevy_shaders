#import bevy_pbr::{
    mesh_functions,
    mesh_view_bindings::globals,
    forward_io::{Vertex, VertexOutput},
}
@group(#{MATERIAL_BIND_GROUP}) @binding(0) var image_texture: texture_2d<f32>;
@group(#{MATERIAL_BIND_GROUP}) @binding(1) var image_sampler: sampler;
@group(#{MATERIAL_BIND_GROUP}) @binding(2) var<uniform> colors: array<vec4f, 4>;

fn get_index(color_value: f32) -> u32 {
    return u32(round(color_value * 20.0));
}

// WARNING: Doesn't work as intended in the lesson. Just for practice
@fragment
fn fragment(
    mesh: VertexOutput,
) -> @location(0) vec4<f32> {
    let gray_weights = vec3f(0.299, 0.587, 0.114);
    let vibrance = 0.5;
    let texture_color = textureSample(image_texture, image_sampler, mesh.uv);

    let color = colors[get_index(texture_color.r)] * texture_color.a;
    let luminance = dot(color.rgb, gray_weights);
    let modified_color = mix(vec3f(luminance), color.rgb, vibrance);

    return vec4f(modified_color, color.a);
}