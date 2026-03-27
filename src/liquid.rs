use bevy::{
    prelude::*, render::render_resource::AsBindGroup, shader::ShaderRef, ui_widgets::ValueChange,
};

use crate::dev_ui::{DevUISliderProps, Panel, labeled_slider};

pub struct LiquidPlugin;

impl Plugin for LiquidPlugin {
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
    wave_height: f32,
    #[uniform(1)]
    amount: f32,
    #[uniform(2)]
    wave_speed: f32,
    #[uniform(3)]
    background_color: LinearRgba,
    #[uniform(4)]
    front_wave_color: LinearRgba,
    #[uniform(5)]
    back_wave_color: LinearRgba,

    alpha_mode: AlphaMode,
}

impl Material for CustomMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/13_liquid.wgsl".into()
    }

    fn alpha_mode(&self) -> AlphaMode {
        self.alpha_mode
    }
}

#[derive(Message, Debug)]
enum DevUIControlMessage {
    WaveHeightChanged(f32),
    AmountChanged(f32),
    WaveSpeedChanged(f32),
}

fn spawn_dev_sliders(mut commands: Commands, q_panel: Query<Entity, With<Panel>>) {
    let Ok(panel) = q_panel.single() else {
        return;
    };
    commands.entity(panel).with_children(|parent| {
        parent.spawn(labeled_slider(
            "Wave height",
            DevUISliderProps {
                value: 0.08,
                min: 0.0,
                max: 1.0,
                step: 0.01,
            },
            |event: On<ValueChange<f32>>,
             mut message_writer: MessageWriter<DevUIControlMessage>| {
                message_writer.write(DevUIControlMessage::WaveHeightChanged(event.value));
            },
        ));
        parent.spawn(labeled_slider(
            "Amount",
            DevUISliderProps {
                value: 0.5,
                min: 0.0,
                max: 1.0,
                step: 0.01,
            },
            |event: On<ValueChange<f32>>,
             mut message_writer: MessageWriter<DevUIControlMessage>| {
                message_writer.write(DevUIControlMessage::AmountChanged(event.value));
            },
        ));
        parent.spawn(labeled_slider(
            "Wave speed",
            DevUISliderProps {
                value: 0.4,
                min: 0.0,
                max: 1.0,
                step: 0.01,
            },
            |event: On<ValueChange<f32>>,
             mut message_writer: MessageWriter<DevUIControlMessage>| {
                message_writer.write(DevUIControlMessage::WaveSpeedChanged(event.value));
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
            wave_height: 0.08,
            amount: 0.5,
            wave_speed: 0.4,
            background_color: Srgba::hex("171210").unwrap().into(),
            front_wave_color: Srgba::hex("e32748").unwrap().into(),
            back_wave_color: Srgba::hex("771210").unwrap().into(),
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
            DevUIControlMessage::WaveHeightChanged(wave_height) => {
                if let Some(material) = materials.get_mut(&material.0) {
                    material.wave_height = *wave_height;
                }
            }
            DevUIControlMessage::AmountChanged(amount) => {
                if let Some(material) = materials.get_mut(&material.0) {
                    material.amount = *amount;
                }
            }
            DevUIControlMessage::WaveSpeedChanged(wave_speed) => {
                if let Some(material) = materials.get_mut(&material.0) {
                    material.wave_speed = *wave_speed;
                }
            }
        }
    }
}
