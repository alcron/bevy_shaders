#import bevy_pbr::{
    mesh_functions,
    mesh_view_bindings::globals,
    forward_io::{Vertex, VertexOutput},
}

@group(#{MATERIAL_BIND_GROUP}) @binding(0) var<uniform> width: f32;
@group(#{MATERIAL_BIND_GROUP}) @binding(1) var<uniform> size: f32;
@group(#{MATERIAL_BIND_GROUP}) @binding(2) var<uniform> blur: f32;
@group(#{MATERIAL_BIND_GROUP}) @binding(3) var noise_texture: texture_2d<f32>;
@group(#{MATERIAL_BIND_GROUP}) @binding(4) var noise_sampler: sampler;
@group(#{MATERIAL_BIND_GROUP}) @binding(5) var<uniform> speed: f32;
@group(#{MATERIAL_BIND_GROUP}) @binding(6) var<uniform> outer_color: vec4f;
@group(#{MATERIAL_BIND_GROUP}) @binding(7) var<uniform> mid_color: vec4f;
@group(#{MATERIAL_BIND_GROUP}) @binding(8) var<uniform> inner_color: vec4f;

// TODO: Add seamless texture sampling to avoid visible tiling when the fire is large and the noise texture is repeated
// TODO: Add more apropriate noise texture
@fragment
fn fragment(
    mesh: VertexOutput,
) -> @location(0) vec4<f32> {
    // make the uv range from -1 to 1 instead of 0 to 1, with (0, 0) in the center of the texture
    let uv = 2.0 * mesh.uv - 1.0;
    // TODO: Too comples calculation, simplify
    let ellipse = 1.0 - length(vec2(uv.x * (2.0 - width), uv.y));
    let falloff_offset = vec2(-0.6, 0.5);
    let falloff = smoothstep(falloff_offset.x, falloff_offset.y, uv.y);
    // let shape = (1.0 - smoothstep(size, size - blur, ellipse)) * falloff;
    let shape = smoothstep(size - blur, size, ellipse) * falloff;
    let moving_uv = vec2(uv.x, uv.y + speed * globals.time);
    let noise = textureSample(noise_texture, noise_sampler, moving_uv).r;
    let outer_shape_alpha = step(0.3, (shape + noise) * 0.3);
    let mid_shape_alpha = step(0.4, shape * 0.3 + noise * 0.4);
    let inner_shape_alpha = step(0.5, shape * 0.3 + noise * 0.5);

    let outer_mask = outer_shape_alpha - mid_shape_alpha;
    let mid_mask = mid_shape_alpha - inner_shape_alpha;
    let color = outer_mask * outer_color + mid_mask * mid_color + inner_shape_alpha * inner_color;

    return vec4(color.rgb, outer_shape_alpha);
}