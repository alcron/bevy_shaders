use bevy::{
    prelude::*, render::render_resource::AsBindGroup, shader::ShaderRef, ui_widgets::ValueChange,
};

use crate::dev_ui::{DevUISliderProps, Panel, labeled_slider};

pub struct MagnifyPlugin;

impl Plugin for MagnifyPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(MaterialPlugin::<CustomMaterial>::default())
            .add_message::<DevUIControlMessage>()
            .add_systems(Startup, (setup, spawn_dev_sliders).chain())
            .add_systems(Update, on_dev_control_change);
    }
}

#[derive(Asset, TypePath, AsBindGroup, Clone)]
struct CustomMaterial {
    #[uniform(0)]
    magnifier_offset: Vec2,
    #[uniform(1)]
    zoom: f32,
    #[uniform(2)]
    blur_intensity: f32,

    alpha_mode: AlphaMode,
}

impl Material for CustomMaterial {
    fn vertex_shader() -> ShaderRef {
        "shaders/21_magnify.wgsl".into()
    }

    fn fragment_shader() -> ShaderRef {
        "shaders/21_magnify.wgsl".into()
    }

    fn alpha_mode(&self) -> AlphaMode {
        self.alpha_mode
    }

    fn reads_view_transmission_texture(&self) -> bool {
        true
    }
}

#[derive(Message, Debug)]
enum DevUIControlMessage {
    MagnifierOffsetXChanged(f32),
    MagnifierOffsetYChanged(f32),
    ZoomChanged(f32),
    BlurIntensityChanged(f32),
}

fn spawn_dev_sliders(mut commands: Commands, panel: Single<Entity, With<Panel>>) {
    commands.entity(*panel).with_children(|parent| {
        parent.spawn(labeled_slider(
            "Magnifier offset X",
            DevUISliderProps {
                value: 0.0,
                min: -1.0,
                max: 1.0,
                step: 0.01,
            },
            |event: On<ValueChange<f32>>,
             mut message_writer: MessageWriter<DevUIControlMessage>| {
                message_writer.write(DevUIControlMessage::MagnifierOffsetXChanged(event.value));
            },
        ));
        parent.spawn(labeled_slider(
            "Magnifier offset Y",
            DevUISliderProps {
                value: 0.0,
                min: -1.0,
                max: 1.0,
                step: 0.01,
            },
            |event: On<ValueChange<f32>>,
             mut message_writer: MessageWriter<DevUIControlMessage>| {
                message_writer.write(DevUIControlMessage::MagnifierOffsetYChanged(event.value));
            },
        ));
        parent.spawn(labeled_slider(
            "Zoom",
            DevUISliderProps {
                value: 0.2,
                min: 0.0,
                max: 1.0,
                step: 0.01,
            },
            |event: On<ValueChange<f32>>,
             mut message_writer: MessageWriter<DevUIControlMessage>| {
                message_writer.write(DevUIControlMessage::ZoomChanged(event.value));
            },
        ));
        parent.spawn(labeled_slider(
            "Blur intensity",
            DevUISliderProps {
                value: 0.2,
                min: 0.0,
                max: 1.0,
                step: 0.01,
            },
            |event: On<ValueChange<f32>>,
             mut message_writer: MessageWriter<DevUIControlMessage>| {
                message_writer.write(DevUIControlMessage::BlurIntensityChanged(event.value));
            },
        ));
    });
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut custom_materials: ResMut<Assets<CustomMaterial>>,
    mut standard_materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
) {
    let map_scale_multiplier = 0.7;
    commands.spawn((
        Mesh3d(
            meshes.add(
                Plane3d::default()
                    .mesh()
                    .size(
                        16.0 * 2.0 / 9.0 * map_scale_multiplier,
                        2.0 * map_scale_multiplier,
                    )
                    .subdivisions(8),
            ),
        ),
        MeshMaterial3d(standard_materials.add(StandardMaterial {
            base_color_texture: Some(asset_server.load("map.png")),
            ..default()
        })),
        Transform::from_xyz(0.0, 0.0, -0.001)
            .with_rotation(Quat::from_rotation_x(std::f32::consts::FRAC_PI_2)),
    ));
    commands.spawn((
        Mesh3d(meshes.add(Plane3d::default().mesh().size(0.5, 0.5).subdivisions(8))),
        MeshMaterial3d(custom_materials.add(CustomMaterial {
            magnifier_offset: Vec2::new(0.0, 0.0),
            zoom: 0.2,
            blur_intensity: 0.2,
            alpha_mode: AlphaMode::Opaque,
        })),
        Transform::from_xyz(0.0, 0.0, 0.0)
            .with_rotation(Quat::from_rotation_x(std::f32::consts::FRAC_PI_2)),
    ));
}

fn on_dev_control_change(
    mut messages: MessageReader<DevUIControlMessage>,
    mut custom_materials: ResMut<Assets<CustomMaterial>>,
    material: Single<&MeshMaterial3d<CustomMaterial>>,
) {
    for message in messages.read() {
        if let Some(material) = custom_materials.get_mut(&material.0) {
            match message {
                DevUIControlMessage::MagnifierOffsetXChanged(x) => {
                    material.magnifier_offset.x = *x;
                }
                DevUIControlMessage::MagnifierOffsetYChanged(y) => {
                    material.magnifier_offset.y = *y;
                }
                DevUIControlMessage::ZoomChanged(zoom) => {
                    material.zoom = *zoom;
                }
                DevUIControlMessage::BlurIntensityChanged(blur_intensity) => {
                    material.blur_intensity = *blur_intensity;
                }
            }
        }
    }
}
