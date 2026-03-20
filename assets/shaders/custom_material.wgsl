#import bevy_pbr::forward_io::VertexOutput

@group(#{MATERIAL_BIND_GROUP}) @binding(0) var<uniform> material_color: vec4<f32>;
@group(#{MATERIAL_BIND_GROUP}) @binding(1) var material_color_texture: texture_2d<f32>;
@group(#{MATERIAL_BIND_GROUP}) @binding(2) var material_color_sampler: sampler;
@group(#{MATERIAL_BIND_GROUP}) @binding(3) var<uniform> material_center_size: f32;
@group(#{MATERIAL_BIND_GROUP}) @binding(4) var<uniform> material_cutout_width: f32;

fn create_ring(uv: vec2<f32>, outer_radius: f32, width: f32) -> f32 {
    let circle_gradient = distance(uv, vec2(0.5));
    let outer_circle = step(circle_gradient, outer_radius);
    let inner_circle = step(circle_gradient, outer_radius - width);

    return outer_circle - inner_circle;
}

@fragment
fn fragment(
    mesh: VertexOutput,
) -> @location(0) vec4<f32> {
    let main_color = vec4(1.0);
    let cross_width = 0.2;

    let circle_gradient = distance(mesh.uv, vec2(0.5));

    let circle = step(circle_gradient, material_center_size);

    let horizontal_cutout = step(distance(mesh.uv.y, 0.5), material_cutout_width);
    let vertical_cutout = step(distance(mesh.uv.x, 0.5), material_cutout_width);
    let ring = create_ring(mesh.uv, 0.5, 0.1);
    let ring_cutout_form = max(horizontal_cutout, vertical_cutout);
    let cutout_ring = ring * (1.0 - ring_cutout_form);

    let horizontal_cross = step(distance(mesh.uv.y, 0.5), 0.01);
    let vertical_cross = step(distance(mesh.uv.x, 0.5), 0.01);
    let cross = max(horizontal_cross, vertical_cross);
    let cross_cutout_form = step(distance(mesh.uv.x, 0.5), cross_width) * step(distance(mesh.uv.y, 0.5), cross_width);
    let cutout_cross = cross * (1.0 - cross_cutout_form);


    return vec4(main_color.rgb, circle + cutout_ring + cutout_cross);
}
