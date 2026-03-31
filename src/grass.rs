use bevy::{
    prelude::*, render::render_resource::AsBindGroup, shader::ShaderRef, ui_widgets::ValueChange,
};

use crate::dev_ui::{DevUISliderProps, Panel, labeled_slider};

pub struct GrassPlugin;

impl Plugin for GrassPlugin {
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
    character_position: Vec2,
    #[uniform(2)]
    grass_positions: [Vec4; 4],
    #[texture(3)]
    #[sampler(4)]
    grass_texture: Option<Handle<Image>>,

    alpha_mode: AlphaMode,
}

impl Material for CustomMaterial {
    fn vertex_shader() -> ShaderRef {
        "shaders/19_grass.wgsl".into()
    }

    fn fragment_shader() -> ShaderRef {
        "shaders/19_grass.wgsl".into()
    }

    fn alpha_mode(&self) -> AlphaMode {
        self.alpha_mode
    }
}

#[derive(Message, Debug)]
enum DevUIControlMessage {
    CharacterPositionXChanged(f32),
    CharacterPositionYChanged(f32),
}

fn spawn_dev_sliders(mut commands: Commands, panel: Single<Entity, With<Panel>>) {
    commands.entity(*panel).with_children(|parent| {
        parent.spawn(labeled_slider(
            "Character position X",
            DevUISliderProps {
                value: 0.0,
                min: -1.0,
                max: 1.0,
                step: 0.1,
            },
            |event: On<ValueChange<f32>>,
             mut message_writer: MessageWriter<DevUIControlMessage>| {
                message_writer.write(DevUIControlMessage::CharacterPositionXChanged(event.value));
            },
        ));
        parent.spawn(labeled_slider(
            "Character position Y",
            DevUISliderProps {
                value: 0.0,
                min: -1.0,
                max: 1.0,
                step: 0.1,
            },
            |event: On<ValueChange<f32>>,
             mut message_writer: MessageWriter<DevUIControlMessage>| {
                message_writer.write(DevUIControlMessage::CharacterPositionYChanged(event.value));
            },
        ));
    });
}

#[derive(Component)]
struct ShaderPlane;

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<CustomMaterial>>,
    asset_server: Res<AssetServer>,
) {
    commands.spawn((
        Mesh3d(meshes.add(Plane3d::default().mesh().size(1.0, 1.0).subdivisions(8))),
        MeshMaterial3d(materials.add(CustomMaterial {
            character_position: Vec2::ZERO,
            grass_positions: [
                Vec4::new(0.25, 0.2, 0.0, 0.0),
                Vec4::new(-0.3, -0.25, 0.0, 0.0),
                Vec4::new(0.1, -0.1, 0.0, 0.0),
                Vec4::new(-0.2, 0.3, 0.0, 0.0),
            ],
            grass_texture: Some(asset_server.load("grass1.png")),
            alpha_mode: AlphaMode::Blend,
        })),
        Transform::from_xyz(0.0, 0.0, 0.0)
            .with_rotation(Quat::from_rotation_x(std::f32::consts::FRAC_PI_2)),
        ShaderPlane,
    ));
}

fn on_dev_control_change(
    mut messages: MessageReader<DevUIControlMessage>,
    mut materials: ResMut<Assets<CustomMaterial>>,
    material: Single<&MeshMaterial3d<CustomMaterial>>,
) {
    for message in messages.read() {
        match message {
            DevUIControlMessage::CharacterPositionXChanged(character_position_x) => {
                if let Some(material) = materials.get_mut(&material.0) {
                    material.character_position.x = *character_position_x;
                }
            }
            DevUIControlMessage::CharacterPositionYChanged(character_position_y) => {
                if let Some(material) = materials.get_mut(&material.0) {
                    material.character_position.y = *character_position_y;
                }
            }
        }
    }
}
