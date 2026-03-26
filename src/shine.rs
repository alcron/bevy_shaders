use bevy::{
    prelude::*, render::render_resource::AsBindGroup, shader::ShaderRef, ui_widgets::ValueChange,
};

use crate::dev_ui::{DevUISliderProps, Panel, labeled_slider};

pub struct ShinePlugin;

impl Plugin for ShinePlugin {
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
    texture: Option<Handle<Image>>,
    #[uniform(2)]
    angle: f32,
    #[uniform(3)]
    thickness: f32,
    #[uniform(4)]
    offset: Vec2,

    alpha_mode: AlphaMode,
}

impl Material for CustomMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/10_shine.wgsl".into()
    }

    fn alpha_mode(&self) -> AlphaMode {
        self.alpha_mode
    }
}

#[derive(Debug)]
enum Offset {
    X(f32),
    Y(f32),
}

#[derive(Message, Debug)]
enum DevUIControlMessage {
    AngleChanged(f32),
    ThicknessChanged(f32),
    OffsetChanged(Offset),
}

fn spawn_dev_sliders(mut commands: Commands, q_panel: Query<Entity, With<Panel>>) {
    let Ok(panel) = q_panel.single() else {
        return;
    };
    commands.entity(panel).with_children(|parent| {
        parent.spawn(labeled_slider(
            "Angle",
            DevUISliderProps {
                value: 30.0,
                min: 0.0,
                max: 360.0,
                step: 1.0,
            },
            |event: On<ValueChange<f32>>,
             mut message_writer: MessageWriter<DevUIControlMessage>| {
                message_writer.write(DevUIControlMessage::AngleChanged(event.value));
            },
        ));
        parent.spawn(labeled_slider(
            "Thickness",
            DevUISliderProps {
                value: 0.1,
                min: 0.0,
                max: 1.0,
                step: 0.01,
            },
            |event: On<ValueChange<f32>>,
             mut message_writer: MessageWriter<DevUIControlMessage>| {
                message_writer.write(DevUIControlMessage::ThicknessChanged(event.value));
            },
        ));
        parent.spawn(labeled_slider(
            "Offset X",
            DevUISliderProps {
                value: 0.0,
                min: -1.0,
                max: 1.0,
                step: 0.01,
            },
            |event: On<ValueChange<f32>>,
             mut message_writer: MessageWriter<DevUIControlMessage>| {
                message_writer.write(DevUIControlMessage::OffsetChanged(Offset::X(event.value)));
            },
        ));
        parent.spawn(labeled_slider(
            "Offset Y",
            DevUISliderProps {
                value: 0.0,
                min: -1.0,
                max: 1.0,
                step: 0.01,
            },
            |event: On<ValueChange<f32>>,
             mut message_writer: MessageWriter<DevUIControlMessage>| {
                message_writer.write(DevUIControlMessage::OffsetChanged(Offset::Y(event.value)));
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
        Mesh3d(meshes.add(Plane3d::default().mesh().size(1.0, 1.0))),
        MeshMaterial3d(materials.add(CustomMaterial {
            texture: Some(asset_server.load("godot_icon.png")),
            angle: 30.0_f32.to_radians(),
            thickness: 0.1,
            offset: Vec2::ZERO,
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
            DevUIControlMessage::AngleChanged(angle) => {
                if let Some(material) = materials.get_mut(&material.0) {
                    material.angle = angle.to_radians();
                }
            }
            DevUIControlMessage::ThicknessChanged(thickness) => {
                if let Some(material) = materials.get_mut(&material.0) {
                    material.thickness = *thickness;
                }
            }
            DevUIControlMessage::OffsetChanged(offset) => {
                if let Some(material) = materials.get_mut(&material.0) {
                    match offset {
                        Offset::X(x) => {
                            material.offset.x = *x;
                        }
                        Offset::Y(y) => {
                            material.offset.y = *y;
                        }
                    }
                }
            }
        }
    }
}
