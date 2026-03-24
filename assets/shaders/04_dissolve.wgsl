#import bevy_pbr::{
    mesh_functions,
    mesh_view_bindings::globals,
    forward_io::{Vertex, VertexOutput},
}

// TODO: Remove if unused
const PI: f32 = acos(-1.0);

@group(#{MATERIAL_BIND_GROUP}) @binding(0) var material_image_texture: texture_2d<f32>;
@group(#{MATERIAL_BIND_GROUP}) @binding(1) var material_image_sampler: sampler;
@group(#{MATERIAL_BIND_GROUP}) @binding(2) var material_noise_texture: texture_2d<f32>;
@group(#{MATERIAL_BIND_GROUP}) @binding(3) var material_noise_sampler: sampler;
@group(#{MATERIAL_BIND_GROUP}) @binding(4) var<uniform> progress: f32;
@group(#{MATERIAL_BIND_GROUP}) @binding(5) var<uniform> gradient_progress: f32;
@group(#{MATERIAL_BIND_GROUP}) @binding(6) var<uniform> dissolve_color: vec3f;

// progress 0 - 1 in 0.8s
// gradient_progress 0 - 1 in 1.1s

// TODO: Try to make offset value between progress and gradient_progress and replace current implementation with separate parameters

fn zero_one_bounce(duration: f32) -> f32 {
    return fract(globals.time / duration * 0.4);
}

// @TODO With this noise dissolve gradient_progress hardly visible. Load another noise texture
@fragment
fn fragment(
    mesh: VertexOutput,
) -> @location(0) vec4<f32> {
    // let progress = zero_one_bounce(0.8);
    // let gradient_progress = zero_one_bounce(1.1);
    let texture = textureSample(material_image_texture, material_image_sampler, mesh.uv);

    let noise_alpha = textureSample(material_noise_texture, material_noise_sampler, mesh.uv).r;
    let alpha = step(progress, noise_alpha)
        + smoothstep(noise_alpha, 0.0, gradient_progress);

    // operator order: select(false_value, true_value, condition)
    let color = select(texture.rgb, dissolve_color, step(progress, noise_alpha) < 1.0);
    // makes the dissolve effect visible only on opaque pixels
    let alpha_final = min(alpha, texture.a);

    return vec4f(color, alpha_final);
    // return vec4f(vec3f(fract(globals.time)), 1.0);
    // return vec4f(vec3f(0.0), 1.0);
}