#import bevy_pbr::{
    mesh_functions,
    mesh_view_bindings::globals,
    forward_io::{Vertex, VertexOutput},
}

@group(#{MATERIAL_BIND_GROUP}) @binding(0) var<uniform> color: f32;
@group(#{MATERIAL_BIND_GROUP}) @binding(1) var<uniform> x: f32;
@group(#{MATERIAL_BIND_GROUP}) @binding(2) var material_texture: texture_2d<f32>;
@group(#{MATERIAL_BIND_GROUP}) @binding(3) var material_texture_sampler: sampler;

@fragment
fn fragment(
    mesh: VertexOutput,
) -> @location(0) vec4<f32> {
    //----- scale & position

    // let scaled_uv = mesh.uv * 2.0 - vec2(x, 0.1);
    // let scaled_texture = textureSample(material_texture, material_texture_sampler, scaled_uv);
    // let mask = step(vec2(0), scaled_uv) * step(scaled_uv, vec2(1));

    // return vec4(scaled_texture.rgb, scaled_texture.a * mask.x * mask.y);


    //----- centering

    // let scaled_uv = mesh.uv * 2.0 - 1.0;

    // return vec4(scaled_uv, 0.0, step(distance(abs(scaled_uv.x), 0.5), 0.1));


    //----- pixelate
    // let grid = vec2(24.0, 24.0);
    // let  pixel_uv = floor(mesh.uv * grid) / grid;
    // let piexled_texture = textureSample(material_texture, material_texture_sampler, pixel_uv);

    // return vec4(piexled_texture);


    //----- checkerboard
    // let grid = floor(mesh.uv * 4.0);
    // let checker = (grid.x + grid.y) % 2.0;

    // return vec4(grid, 0.0, checker);


    //----- pixelate upgrade
    // *** my solution
    // let grid = vec2(24.0, 24.0);
    // let pixel_uv_min = floor(mesh.uv * grid) / grid;
    // let pixel_uv_max = ceil(mesh.uv * grid) / grid;
    // let piexled_texture = textureSample(material_texture, material_texture_sampler, pixel_uv_min) * textureSample(material_texture, material_texture_sampler, pixel_uv_max) / 2.0;
    // return vec4(piexled_texture);
    // *** 

    // *** course colution
    let pixel_size: u32 = 32;
    let pixel_uv = floor((mesh.uv * f32(pixel_size)) + 0.5) / f32(pixel_size);
    let piexled_texture = textureSample(material_texture, material_texture_sampler, pixel_uv);

    return vec4(piexled_texture);
    // *** 
}
