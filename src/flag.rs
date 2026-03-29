use bevy::{
    prelude::*, render::render_resource::AsBindGroup, shader::ShaderRef, ui_widgets::ValueChange,
};

use crate::dev_ui::{DevUISliderProps, Panel, labeled_slider};

pub struct FlagPlugin;

impl Plugin for FlagPlugin {
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
    speed: f32,
    #[uniform(1)]
    frequency_x: f32,
    #[uniform(2)]
    frequency_y: f32,
    #[uniform(3)]
    amplitude_x: f32,
    #[uniform(4)]
    amplitude_y: f32,
    #[uniform(5)]
    incline: f32,
    #[uniform(6)]
    max_incline: f32,

    alpha_mode: AlphaMode,
}

impl Material for CustomMaterial {
    fn vertex_shader() -> ShaderRef {
        "shaders/14_flag.wgsl".into()
    }

    fn fragment_shader() -> ShaderRef {
        "shaders/14_flag.wgsl".into()
    }

    fn alpha_mode(&self) -> AlphaMode {
        self.alpha_mode
    }
}

#[derive(Message, Debug)]
enum DevUIControlMessage {
    SpeedChanged(f32),
    FrequencyXChanged(f32),
    FrequencyYChanged(f32),
    AmplitudeXChanged(f32),
    AmplitudeYChanged(f32),
    InclineChanged(f32),
    MaxInclineChanged(f32),
}

fn spawn_dev_sliders(mut commands: Commands, q_panel: Query<Entity, With<Panel>>) {
    let Ok(panel) = q_panel.single() else {
        return;
    };
    commands.entity(panel).with_children(|parent| {
        parent.spawn(labeled_slider(
            "Speed",
            DevUISliderProps {
                value: 0.6,
                min: 0.0,
                max: 1.0,
                step: 0.01,
            },
            |event: On<ValueChange<f32>>,
             mut message_writer: MessageWriter<DevUIControlMessage>| {
                message_writer.write(DevUIControlMessage::SpeedChanged(event.value));
            },
        ));
        parent.spawn(labeled_slider(
            "Frequency X",
            DevUISliderProps {
                value: 1.0,
                min: 0.0,
                max: 10.0,
                step: 0.1,
            },
            |event: On<ValueChange<f32>>,
             mut message_writer: MessageWriter<DevUIControlMessage>| {
                message_writer.write(DevUIControlMessage::FrequencyXChanged(event.value));
            },
        ));
        parent.spawn(labeled_slider(
            "Frequency Y",
            DevUISliderProps {
                value: 1.0,
                min: 0.0,
                max: 10.0,
                step: 0.1,
            },
            |event: On<ValueChange<f32>>,
             mut message_writer: MessageWriter<DevUIControlMessage>| {
                message_writer.write(DevUIControlMessage::FrequencyYChanged(event.value));
            },
        ));
        parent.spawn(labeled_slider(
            "Amplitude X",
            DevUISliderProps {
                value: 1.0,
                min: 0.0,
                max: 10.0,
                step: 0.1,
            },
            |event: On<ValueChange<f32>>,
             mut message_writer: MessageWriter<DevUIControlMessage>| {
                message_writer.write(DevUIControlMessage::AmplitudeXChanged(event.value));
            },
        ));
        parent.spawn(labeled_slider(
            "Amplitude Y",
            DevUISliderProps {
                value: 1.0,
                min: 0.0,
                max: 10.0,
                step: 0.1,
            },
            |event: On<ValueChange<f32>>,
             mut message_writer: MessageWriter<DevUIControlMessage>| {
                message_writer.write(DevUIControlMessage::AmplitudeYChanged(event.value));
            },
        ));
        parent.spawn(labeled_slider(
            "Incline",
            DevUISliderProps {
                value: 0.3,
                min: 0.0,
                max: 1.0,
                step: 0.01,
            },
            |event: On<ValueChange<f32>>,
             mut message_writer: MessageWriter<DevUIControlMessage>| {
                message_writer.write(DevUIControlMessage::InclineChanged(event.value));
            },
        ));
        parent.spawn(labeled_slider(
            "Max incline",
            DevUISliderProps {
                value: 1.0,
                min: 0.0,
                max: 10.0,
                step: 0.1,
            },
            |event: On<ValueChange<f32>>,
             mut message_writer: MessageWriter<DevUIControlMessage>| {
                message_writer.write(DevUIControlMessage::MaxInclineChanged(event.value));
            },
        ));
    });
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<CustomMaterial>>,
) {
    commands.spawn((
        Mesh3d(meshes.add(Plane3d::default().mesh().size(1.0, 1.0).subdivisions(8))),
        MeshMaterial3d(materials.add(CustomMaterial {
            speed: 0.6,
            frequency_x: 1.0,
            frequency_y: 1.0,
            amplitude_x: 1.0,
            amplitude_y: 1.0,
            incline: 0.3,
            max_incline: 1.0,
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
            DevUIControlMessage::SpeedChanged(speed) => {
                if let Some(material) = materials.get_mut(&material.0) {
                    material.speed = *speed;
                }
            }
            DevUIControlMessage::FrequencyXChanged(frequency_x) => {
                if let Some(material) = materials.get_mut(&material.0) {
                    material.frequency_x = *frequency_x;
                }
            }
            DevUIControlMessage::FrequencyYChanged(frequency_y) => {
                if let Some(material) = materials.get_mut(&material.0) {
                    material.frequency_y = *frequency_y;
                }
            }
            DevUIControlMessage::AmplitudeXChanged(amplitude_x) => {
                if let Some(material) = materials.get_mut(&material.0) {
                    material.amplitude_x = *amplitude_x;
                }
            }
            DevUIControlMessage::AmplitudeYChanged(amplitude_y) => {
                if let Some(material) = materials.get_mut(&material.0) {
                    material.amplitude_y = *amplitude_y;
                }
            }
            DevUIControlMessage::InclineChanged(incline) => {
                if let Some(material) = materials.get_mut(&material.0) {
                    material.incline = *incline;
                }
            }
            DevUIControlMessage::MaxInclineChanged(max_incline) => {
                if let Some(material) = materials.get_mut(&material.0) {
                    material.max_incline = *max_incline;
                }
            }
        }
    }
}
