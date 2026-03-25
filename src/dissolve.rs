use bevy::{
    image::{ImageAddressMode, ImageLoaderSettings, ImageSampler, ImageSamplerDescriptor},
    prelude::*,
    render::render_resource::AsBindGroup,
    shader::ShaderRef,
    ui_widgets::ValueChange,
};

use crate::dev_ui::{DevUISliderProps, Panel, labeled_slider};

pub struct DissolvePlugin;

impl Plugin for DissolvePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((MaterialPlugin::<CustomMaterial>::default()))
            .add_systems(Startup, (setup, spawn_dev_sliders).chain());
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
            |event: On<ValueChange<f32>>,
             mut materials: ResMut<Assets<CustomMaterial>>,
             material: Single<&MeshMaterial3d<CustomMaterial>>| {
                if let Some(mat) = materials.get_mut(&material.0) {
                    mat.progress = event.value;
                }
            },
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
            |event: On<ValueChange<f32>>,
             mut materials: ResMut<Assets<CustomMaterial>>,
             material: Single<&MeshMaterial3d<CustomMaterial>>| {
                if let Some(mat) = materials.get_mut(&material.0) {
                    mat.gradient_progress = event.value;
                }
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


