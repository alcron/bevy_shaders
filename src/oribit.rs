use bevy::{
    prelude::*, render::render_resource::AsBindGroup, shader::ShaderRef, ui_widgets::ValueChange,
};

use crate::dev_ui::{DevUISliderProps, Panel, labeled_slider};

pub struct OrbitPlugin;

impl Plugin for OrbitPlugin {
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
    earth_radius: f32,
    #[uniform(1)]
    earth_color: LinearRgba,
    #[uniform(2)]
    moon_offset: f32,
    #[uniform(3)]
    moon_radius: f32,
    #[uniform(4)]
    moon_color: LinearRgba,
    #[uniform(5)]
    satellite_offset: f32,
    #[uniform(6)]
    satellite_radius: f32,
    #[uniform(7)]
    satellite_color: LinearRgba,

    alpha_mode: AlphaMode,
}

impl Material for CustomMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/09_orbit.wgsl".into()
    }

    fn alpha_mode(&self) -> AlphaMode {
        self.alpha_mode
    }
}

#[derive(Message, Debug)]
enum DevUIControlMessage {
    EarthRadiusChanged(f32),
    MoonOffsetChanged(f32),
    MoonRadiusChanged(f32),
    SatelliteOffsetChanged(f32),
    SatelliteRadiusChanged(f32),
}

fn spawn_dev_sliders(mut commands: Commands, q_panel: Query<Entity, With<Panel>>) {
    let Ok(panel) = q_panel.single() else {
        return;
    };
    commands.entity(panel).with_children(|parent| {
        parent.spawn(labeled_slider(
            "Earth radius",
            DevUISliderProps {
                value: 0.1,
                min: 0.0,
                max: 0.2,
                step: 0.01,
            },
            |event: On<ValueChange<f32>>,
             mut message_writer: MessageWriter<DevUIControlMessage>| {
                message_writer.write(DevUIControlMessage::EarthRadiusChanged(event.value));
            },
        ));
        parent.spawn(labeled_slider(
            "Moon offset",
            DevUISliderProps {
                value: 0.4,
                min: 0.0,
                max: 0.5,
                step: 0.005,
            },
            |event: On<ValueChange<f32>>,
             mut message_writer: MessageWriter<DevUIControlMessage>| {
                message_writer.write(DevUIControlMessage::MoonOffsetChanged(event.value));
            },
        ));
        parent.spawn(labeled_slider(
            "Moon radius",
            DevUISliderProps {
                value: 0.05,
                min: 0.0,
                max: 0.5,
                step: 0.005,
            },
            |event: On<ValueChange<f32>>,
             mut message_writer: MessageWriter<DevUIControlMessage>| {
                message_writer.write(DevUIControlMessage::MoonRadiusChanged(event.value));
            },
        ));
        parent.spawn(labeled_slider(
            "Satellite radius",
            DevUISliderProps {
                value: 0.03,
                min: 0.0,
                max: 0.5,
                step: 0.005,
            },
            |event: On<ValueChange<f32>>,
             mut message_writer: MessageWriter<DevUIControlMessage>| {
                message_writer.write(DevUIControlMessage::SatelliteRadiusChanged(event.value));
            },
        ));
        parent.spawn(labeled_slider(
            "Satellite offset",
            DevUISliderProps {
                value: 0.2,
                min: 0.0,
                max: 0.5,
                step: 0.005,
            },
            |event: On<ValueChange<f32>>,
             mut message_writer: MessageWriter<DevUIControlMessage>| {
                message_writer.write(DevUIControlMessage::SatelliteOffsetChanged(event.value));
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
            earth_radius: 0.1,
            earth_color: LinearRgba::new(0.0, 1.0, 0.0, 1.0),
            moon_offset: 0.4,
            moon_radius: 0.05,
            moon_color: LinearRgba::new(0.0, 0.0, 1.0, 1.0),
            satellite_offset: 0.2,
            satellite_radius: 0.03,
            satellite_color: LinearRgba::new(1.0, 0.0, 0.0, 1.0),
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
            DevUIControlMessage::EarthRadiusChanged(value) => {
                if let Some(mat) = materials.get_mut(&material.0) {
                    mat.earth_radius = *value;
                }
            }
            DevUIControlMessage::MoonOffsetChanged(value) => {
                if let Some(mat) = materials.get_mut(&material.0) {
                    mat.moon_offset = *value;
                }
            }
            DevUIControlMessage::MoonRadiusChanged(value) => {
                if let Some(mat) = materials.get_mut(&material.0) {
                    mat.moon_radius = *value;
                }
            }
            DevUIControlMessage::SatelliteOffsetChanged(value) => {
                if let Some(mat) = materials.get_mut(&material.0) {
                    mat.satellite_offset = *value;
                }
            }
            DevUIControlMessage::SatelliteRadiusChanged(value) => {
                if let Some(mat) = materials.get_mut(&material.0) {
                    mat.satellite_radius = *value;
                }
            }
        }
    }
}
