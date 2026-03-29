use bevy::{
    prelude::*, render::render_resource::AsBindGroup, shader::ShaderRef, ui_widgets::ValueChange,
};

use crate::dev_ui::{DevUISliderProps, Panel, labeled_slider};

pub struct MatrixPlugin;

impl Plugin for MatrixPlugin {
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
    offset_x: f32,
    #[uniform(3)]
    offset_y: f32,
    #[uniform(4)]
    scale: f32,
    #[uniform(5)]
    angle: f32,

    alpha_mode: AlphaMode,
}

impl Material for CustomMaterial {
    fn vertex_shader() -> ShaderRef {
        "shaders/15_matrix.wgsl".into()
    }

    fn fragment_shader() -> ShaderRef {
        "shaders/15_matrix.wgsl".into()
    }

    fn alpha_mode(&self) -> AlphaMode {
        self.alpha_mode
    }
}

#[derive(Message, Debug)]
enum DevUIControlMessage {
    OffsetXChanged(f32),
    OffsetYChanged(f32),
    ScaleChanged(f32),
    AngleChanged(f32),
}

fn spawn_dev_sliders(mut commands: Commands, panel: Single<Entity, With<Panel>>) {
    commands.entity(*panel).with_children(|parent| {
        parent.spawn(labeled_slider(
            "Offset X",
            DevUISliderProps {
                value: 0.0,
                min: -10.0,
                max: 10.0,
                step: 0.1,
            },
            |event: On<ValueChange<f32>>,
             mut message_writer: MessageWriter<DevUIControlMessage>| {
                message_writer.write(DevUIControlMessage::OffsetXChanged(event.value));
            },
        ));
        parent.spawn(labeled_slider(
            "Offset Y",
            DevUISliderProps {
                value: 0.0,
                min: -10.0,
                max: 10.0,
                step: 0.1,
            },
            |event: On<ValueChange<f32>>,
             mut message_writer: MessageWriter<DevUIControlMessage>| {
                message_writer.write(DevUIControlMessage::OffsetYChanged(event.value));
            },
        ));
        parent.spawn(labeled_slider(
            "Scale",
            DevUISliderProps {
                value: 1.0,
                min: -10.0,
                max: 10.0,
                step: 1.0,
            },
            |event: On<ValueChange<f32>>,
             mut message_writer: MessageWriter<DevUIControlMessage>| {
                message_writer.write(DevUIControlMessage::ScaleChanged(event.value));
            },
        ));
        parent.spawn(labeled_slider(
            "Angle",
            DevUISliderProps {
                value: 0.0,
                min: -1.0,
                max: 1.0,
                step: 0.1,
            },
            |event: On<ValueChange<f32>>,
             mut message_writer: MessageWriter<DevUIControlMessage>| {
                message_writer.write(DevUIControlMessage::AngleChanged(event.value));
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
            offset_x: 0.0,
            offset_y: 0.0,
            scale: 1.0,
            angle: 0.0,
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
            DevUIControlMessage::OffsetXChanged(offset_x) => {
                if let Some(material) = materials.get_mut(&material.0) {
                    material.offset_x = *offset_x;
                }
            }
            DevUIControlMessage::OffsetYChanged(offset_y) => {
                if let Some(material) = materials.get_mut(&material.0) {
                    material.offset_y = *offset_y;
                }
            }
            DevUIControlMessage::ScaleChanged(scale) => {
                if let Some(material) = materials.get_mut(&material.0) {
                    material.scale = *scale;
                }
            }
            DevUIControlMessage::AngleChanged(angle) => {
                if let Some(material) = materials.get_mut(&material.0) {
                    material.angle = *angle;
                }
            }
        }
    }
}
