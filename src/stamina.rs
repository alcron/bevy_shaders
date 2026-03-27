use std::time::Duration;

use bevy::{
    prelude::*, render::render_resource::AsBindGroup, shader::ShaderRef, ui_widgets::ValueChange,
};
use bevy_tweening::*;

use crate::dev_ui::{DevUISliderProps, Panel, labeled_slider};

pub struct StaminaPlugin;

impl Plugin for StaminaPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(MaterialPlugin::<CustomMaterial>::default())
            .add_message::<DevUIControlMessage>()
            .add_systems(Startup, (setup, spawn_dev_sliders).chain())
            .add_systems(Update, (on_dev_control_change, handle_keyboard_input));
    }
}

// TODO: Add color parameters and change change_progress color for negative and positive stamina changes
#[derive(Asset, TypePath, AsBindGroup, Clone)]
struct CustomMaterial {
    #[uniform(0)]
    diameter: f32,
    #[uniform(1)]
    width: f32,
    #[uniform(2)]
    progress: f32,
    #[uniform(3)]
    change_progress: f32,

    alpha_mode: AlphaMode,
}

#[derive(Component)]
struct ProgressTween;

struct ChangeProgressLens {
    start: f32,
    end: f32,
}

impl Lens<CustomMaterial> for ChangeProgressLens {
    fn lerp(&mut self, mut target: Mut<CustomMaterial>, ratio: f32) {
        target.change_progress = self.start + (self.end - self.start) * ratio;
    }
}

struct ProgressLens {
    start: f32,
    end: f32,
}

impl Lens<CustomMaterial> for ProgressLens {
    fn lerp(&mut self, mut target: Mut<CustomMaterial>, ratio: f32) {
        target.progress = self.start + (self.end - self.start) * ratio;
    }
}

impl Material for CustomMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/12_stamina.wgsl".into()
    }

    fn alpha_mode(&self) -> AlphaMode {
        self.alpha_mode
    }
}

#[derive(Message, Debug)]
enum DevUIControlMessage {
    DiameterChanged(f32),
    WidthChanged(f32),
    ProgressChanged(f32),
    ChangeProgressChanged(f32),
}

fn spawn_dev_sliders(mut commands: Commands, q_panel: Query<Entity, With<Panel>>) {
    let Ok(panel) = q_panel.single() else {
        return;
    };
    commands.entity(panel).with_children(|parent| {
        parent.spawn(labeled_slider(
            "Diameter",
            DevUISliderProps {
                value: 1.0,
                min: 0.0,
                max: 1.0,
                step: 0.01,
            },
            |event: On<ValueChange<f32>>,
             mut message_writer: MessageWriter<DevUIControlMessage>| {
                message_writer.write(DevUIControlMessage::DiameterChanged(event.value));
            },
        ));
        parent.spawn(labeled_slider(
            "Width",
            DevUISliderProps {
                value: 0.5,
                min: 0.0,
                max: 1.0,
                step: 0.01,
            },
            |event: On<ValueChange<f32>>,
             mut message_writer: MessageWriter<DevUIControlMessage>| {
                message_writer.write(DevUIControlMessage::WidthChanged(event.value));
            },
        ));
        parent.spawn(labeled_slider(
            "Progress",
            DevUISliderProps {
                value: 0.8,
                min: 0.0,
                max: 1.0,
                step: 0.01,
            },
            |event: On<ValueChange<f32>>,
             mut message_writer: MessageWriter<DevUIControlMessage>| {
                message_writer.write(DevUIControlMessage::ProgressChanged(event.value));
            },
        ));
        parent.spawn(labeled_slider(
            "Change Progress",
            DevUISliderProps {
                value: 0.8,
                min: 0.0,
                max: 1.0,
                step: 0.01,
            },
            |event: On<ValueChange<f32>>,
             mut message_writer: MessageWriter<DevUIControlMessage>| {
                message_writer.write(DevUIControlMessage::ChangeProgressChanged(event.value));
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
        Mesh3d(meshes.add(Plane3d::default().mesh().size(1.0, 1.0))),
        MeshMaterial3d(materials.add(CustomMaterial {
            diameter: 1.0,
            width: 0.5,
            progress: 0.8,
            change_progress: 0.8,
            alpha_mode: AlphaMode::Blend,
        })),
        Transform::from_xyz(0.0, 0.0, 0.0)
            .with_rotation(Quat::from_rotation_x(std::f32::consts::FRAC_PI_2)),
    ));
}

fn handle_keyboard_input(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut commands: Commands,
    mut materials: ResMut<Assets<CustomMaterial>>,
    material: Single<&MeshMaterial3d<CustomMaterial>>,
    existing_tweens: Query<Entity, With<ProgressTween>>,
) {
    let mut stamina_change = 0.0;

    if keyboard.just_pressed(KeyCode::ArrowDown) {
        stamina_change -= 0.2;
    } else if keyboard.just_pressed(KeyCode::ArrowUp) {
        stamina_change += 0.2;
    }

    if stamina_change != 0.0 {
        for entity in existing_tweens.iter() {
            commands.entity(entity).despawn();
        }
        if let Some(mat) = materials.get_mut(&material.0) {
            let current = mat.change_progress;
            let target = (current + stamina_change).clamp(0.0, 1.0);

            if current == target {
                return;
            }

            if stamina_change < 0.0 {
                mat.progress = target;

                let tween = Tween::new(
                    EaseFunction::QuadraticInOut,
                    Duration::from_millis(300),
                    ChangeProgressLens {
                        start: current,
                        end: target,
                    },
                );
                commands.spawn((
                    ProgressTween,
                    TweenAnim::new(tween),
                    AnimTarget::asset::<CustomMaterial>(material.0.id()),
                ));
            } else {
                mat.change_progress = target;

                let tween = Tween::new(
                    EaseFunction::QuadraticInOut,
                    Duration::from_millis(300),
                    ProgressLens {
                        start: current,
                        end: target,
                    },
                );
                commands.spawn((
                    ProgressTween,
                    TweenAnim::new(tween),
                    AnimTarget::asset::<CustomMaterial>(material.0.id()),
                ));
            }
        }
    }
}

fn on_dev_control_change(
    mut messages: MessageReader<DevUIControlMessage>,
    mut materials: ResMut<Assets<CustomMaterial>>,
    material: Single<&MeshMaterial3d<CustomMaterial>>,
) {
    for message in messages.read() {
        match message {
            DevUIControlMessage::DiameterChanged(diameter) => {
                if let Some(material) = materials.get_mut(&material.0) {
                    material.diameter = *diameter;
                }
            }
            DevUIControlMessage::WidthChanged(width) => {
                if let Some(material) = materials.get_mut(&material.0) {
                    material.width = *width;
                }
            }
            DevUIControlMessage::ProgressChanged(progress) => {
                if let Some(material) = materials.get_mut(&material.0) {
                    material.progress = *progress;
                }
            }
            DevUIControlMessage::ChangeProgressChanged(change_progress) => {
                if let Some(material) = materials.get_mut(&material.0) {
                    material.change_progress = *change_progress;
                }
            }
        }
    }
}
