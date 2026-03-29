use bevy::{
    prelude::*, render::render_resource::AsBindGroup, shader::ShaderRef, ui_widgets::ValueChange,
};

use crate::dev_ui::{DevUISliderProps, Panel, labeled_slider};

pub struct Fake3DPlugin;

impl Plugin for Fake3DPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(MaterialPlugin::<CustomMaterial>::default())
            .add_message::<DevUIControlMessage>()
            .add_systems(Startup, (setup, spawn_dev_sliders).chain())
            .add_systems(Update, on_dev_control_change);
    }
}

#[derive(Asset, TypePath, AsBindGroup, Clone)]
struct CustomMaterial {
    #[texture(0)]
    #[sampler(1)]
    image_texture: Option<Handle<Image>>,
    #[uniform(2)]
    rotation_x: f32,
    #[uniform(3)]
    rotation_y: f32,
    #[uniform(4)]
    rotation_z: f32,
    #[uniform(5)]
    pivot_x: f32,
    #[uniform(6)]
    pivot_y: f32,

    alpha_mode: AlphaMode,
}

impl Material for CustomMaterial {
    fn vertex_shader() -> ShaderRef {
        "shaders/16_fake_3d.wgsl".into()
    }

    fn fragment_shader() -> ShaderRef {
        "shaders/16_fake_3d.wgsl".into()
    }

    fn alpha_mode(&self) -> AlphaMode {
        self.alpha_mode
    }
}

#[derive(Message, Debug)]
enum DevUIControlMessage {
    RotationXChanged(f32),
    RotationYChanged(f32),
    RotationZChanged(f32),
    PivotXChanged(f32),
    PivotYChanged(f32),
}

fn spawn_dev_sliders(mut commands: Commands, panel: Single<Entity, With<Panel>>) {
    commands.entity(*panel).with_children(|parent| {
        parent.spawn(labeled_slider(
            "Rotation X",
            DevUISliderProps {
                value: 0.0,
                min: -1.0,
                max: 1.0,
                step: 0.1,
            },
            |event: On<ValueChange<f32>>,
             mut message_writer: MessageWriter<DevUIControlMessage>| {
                message_writer.write(DevUIControlMessage::RotationXChanged(event.value));
            },
        ));
        parent.spawn(labeled_slider(
            "Rotation Y",
            DevUISliderProps {
                value: 0.0,
                min: -1.0,
                max: 1.0,
                step: 0.1,
            },
            |event: On<ValueChange<f32>>,
             mut message_writer: MessageWriter<DevUIControlMessage>| {
                message_writer.write(DevUIControlMessage::RotationYChanged(event.value));
            },
        ));
        parent.spawn(labeled_slider(
            "Rotation Z",
            DevUISliderProps {
                value: 0.0,
                min: -1.0,
                max: 1.0,
                step: 0.1,
            },
            |event: On<ValueChange<f32>>,
             mut message_writer: MessageWriter<DevUIControlMessage>| {
                message_writer.write(DevUIControlMessage::RotationZChanged(event.value));
            },
        ));
        parent.spawn(labeled_slider(
            "Pivot X",
            DevUISliderProps {
                value: 0.0,
                min: -10.0,
                max: 10.0,
                step: 0.1,
            },
            |event: On<ValueChange<f32>>,
             mut message_writer: MessageWriter<DevUIControlMessage>| {
                message_writer.write(DevUIControlMessage::PivotXChanged(event.value));
            },
        ));
        parent.spawn(labeled_slider(
            "Pivot Y",
            DevUISliderProps {
                value: 0.0,
                min: -10.0,
                max: 10.0,
                step: 0.1,
            },
            |event: On<ValueChange<f32>>,
             mut message_writer: MessageWriter<DevUIControlMessage>| {
                message_writer.write(DevUIControlMessage::PivotYChanged(event.value));
            },
        ));
    });
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<CustomMaterial>>,
    asset_server: Res<AssetServer>,
) {
    commands.spawn((
        Mesh3d(meshes.add(Plane3d::default().mesh().size(1.0, 1.0).subdivisions(8))),
        MeshMaterial3d(materials.add(CustomMaterial {
            image_texture: Some(asset_server.load("godot_icon.png")),
            rotation_x: 0.0,
            rotation_y: 0.0,
            rotation_z: 0.0,
            pivot_x: 0.0,
            pivot_y: 0.0,
            alpha_mode: AlphaMode::Blend,
        })),
        Transform::from_xyz(0.0, 0.0, 0.0)
            .with_rotation(Quat::from_rotation_x(std::f32::consts::FRAC_PI_2)),
    ));
}

fn on_dev_control_change(
    mut messages: MessageReader<DevUIControlMessage>,
    mut materials: ResMut<Assets<CustomMaterial>>,
    material: Single<&MeshMaterial3d<CustomMaterial>>,
) {
    for message in messages.read() {
        match message {
            DevUIControlMessage::RotationXChanged(rotation_x) => {
                if let Some(material) = materials.get_mut(&material.0) {
                    material.rotation_x = *rotation_x;
                }
            }
            DevUIControlMessage::RotationYChanged(rotation_y) => {
                if let Some(material) = materials.get_mut(&material.0) {
                    material.rotation_y = *rotation_y;
                }
            }
            DevUIControlMessage::RotationZChanged(rotation_z) => {
                if let Some(material) = materials.get_mut(&material.0) {
                    material.rotation_z = *rotation_z;
                }
            }
            DevUIControlMessage::PivotXChanged(pivot_x) => {
                if let Some(material) = materials.get_mut(&material.0) {
                    material.pivot_x = *pivot_x;
                }
            }
            DevUIControlMessage::PivotYChanged(pivot_y) => {
                if let Some(material) = materials.get_mut(&material.0) {
                    material.pivot_y = *pivot_y;
                }
            }
        }
    }
}
