use bevy::{
    image::{ImageAddressMode, ImageLoaderSettings, ImageSampler, ImageSamplerDescriptor},
    prelude::*,
    render::render_resource::AsBindGroup,
    shader::ShaderRef,
};

use crate::dev_ui::{DevUIMessage, DevUISliderProps, Panel, labeled_slider};

pub struct DissolvePlugin;

impl Plugin for DissolvePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((MaterialPlugin::<CustomMaterial>::default()))
            .add_systems(Startup, (setup, spawn_dev_sliders).chain())
            .add_systems(Update, on_slider_change);
    }
}

#[derive(Asset, TypePath, AsBindGroup, Clone)]
struct CustomMaterial {
    #[texture(0)]
    #[sampler(1)]
    image_texture: Option<Handle<Image>>,
    #[texture(2)]
    #[sampler(3)]
    noise_texture: Option<Handle<Image>>,
    #[uniform(4)]
    progress: f32,
    #[uniform(5)]
    gradient_progress: f32,
    #[uniform(6)]
    dissolve_color: Vec3,
    alpha_mode: AlphaMode,
}

impl Material for CustomMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/04_dissolve.wgsl".into()
    }

    fn alpha_mode(&self) -> AlphaMode {
        self.alpha_mode
    }
}

#[derive(Component, PartialEq)]
enum DevControls {
    Progress,
    GradientProgress,
    DissolveColor,
}

fn spawn_dev_sliders(mut commands: Commands, q_panel: Query<Entity, With<Panel>>) {
    let Ok(panel) = q_panel.single() else {
        return;
    };
    commands.entity(panel).with_children(|parent| {
        parent.spawn(labeled_slider(
            "Progress",
            DevUISliderProps {
                value: 0.5,
                min: 0.0,
                max: 1.0,
                step: 0.01,
            },
            DevControls::Progress,
        ));
        parent.spawn(labeled_slider(
            "Gradient Progress",
            DevUISliderProps {
                value: 0.25,
                min: 0.0,
                max: 1.0,
                step: 0.01,
                ..default()
            },
            DevControls::GradientProgress,
        ));
    });
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<CustomMaterial>>,
    asset_server: Res<AssetServer>,
) {
    // plane with custom circle shader
    commands.spawn((
        Mesh3d(meshes.add(Plane3d::default().mesh().size(1.0, 1.0))),
        MeshMaterial3d(materials.add(CustomMaterial {
            image_texture: Some(asset_server.load("godot_icon.png")),
            noise_texture: Some(asset_server.load("noise.png")),
            // TODO: Connect with corresponding slider value in the dev UI
            progress: 0.5,
            gradient_progress: 0.25,
            dissolve_color: Vec3::new(0.486, 0.565, 1.0),
            alpha_mode: AlphaMode::Blend,
        })),
        Transform::from_xyz(0.0, 0.0, 0.0)
            .with_rotation(Quat::from_rotation_x(std::f32::consts::FRAC_PI_2)),
    ));
}

// TODO: Think how to decouple this from dev ui and handle it in a more generic way, e.g. by sending messages with marker and applying them to all materials that have this uniform
fn on_slider_change(
    mut message_reader: MessageReader<DevUIMessage>,
    q_controls: Query<&DevControls>,
    mut materials: ResMut<Assets<CustomMaterial>>,
    material: Single<&MeshMaterial3d<CustomMaterial>>,
) {
    for message in message_reader.read() {
        let Ok(control) = q_controls.get(message.entity) else {
            continue;
        };
        if let Some(mat) = materials.get_mut(&material.0) {
            match control {
                DevControls::Progress => mat.progress = message.value,
                DevControls::GradientProgress => mat.gradient_progress = message.value,
                // TODO: Make color picker in dev UI
                // DevControls::DissolveColor => {
                //     mat.dissolve_color = Vec3::new(message.value, message.value, message.value)
                // }
                _ => {}
            }
        }
    }
}
