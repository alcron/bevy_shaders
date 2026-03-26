use bevy::{
    prelude::*, render::render_resource::AsBindGroup, shader::ShaderRef, ui_widgets::ValueChange,
};

use crate::dev_ui::{DevUISliderProps, Panel, labeled_slider};

pub struct ReloadPlugin;

impl Plugin for ReloadPlugin {
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
    reload: f32,
    #[uniform(3)]
    charge: f32,
    #[uniform(4)]
    alpha: f32,

    alpha_mode: AlphaMode,
}

impl Material for CustomMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/11_reload.wgsl".into()
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
    ReloadChanged(f32),
    ChargeChanged(f32),
    AlphaChanged(f32),
}

fn spawn_dev_sliders(mut commands: Commands, q_panel: Query<Entity, With<Panel>>) {
    let Ok(panel) = q_panel.single() else {
        return;
    };
    commands.entity(panel).with_children(|parent| {
        parent.spawn(labeled_slider(
            "Reload",
            DevUISliderProps {
                value: 0.0,
                min: 0.0,
                max: 1.0,
                step: 0.01,
            },
            |event: On<ValueChange<f32>>,
             mut message_writer: MessageWriter<DevUIControlMessage>| {
                message_writer.write(DevUIControlMessage::ReloadChanged(event.value));
            },
        ));
        parent.spawn(labeled_slider(
            "Charge",
            DevUISliderProps {
                value: 0.5,
                min: 0.0,
                max: 1.0,
                step: 0.01,
            },
            |event: On<ValueChange<f32>>,
             mut message_writer: MessageWriter<DevUIControlMessage>| {
                message_writer.write(DevUIControlMessage::ChargeChanged(event.value));
            },
        ));
        parent.spawn(labeled_slider(
            "Alpha",
            DevUISliderProps {
                value: 0.8,
                min: 0.0,
                max: 1.0,
                step: 0.01,
            },
            |event: On<ValueChange<f32>>,
             mut message_writer: MessageWriter<DevUIControlMessage>| {
                message_writer.write(DevUIControlMessage::AlphaChanged(event.value));
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
            reload: 0.0,
            charge: 0.5,
            alpha: 0.8,
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
            DevUIControlMessage::ReloadChanged(reload) => {
                if let Some(material) = materials.get_mut(&material.0) {
                    material.reload = *reload;
                }
            }
            DevUIControlMessage::ChargeChanged(charge) => {
                if let Some(material) = materials.get_mut(&material.0) {
                    material.charge = *charge;
                }
            }
            DevUIControlMessage::AlphaChanged(alpha) => {
                if let Some(material) = materials.get_mut(&material.0) {
                    material.alpha = *alpha;
                }
            }
        }
    }
}
