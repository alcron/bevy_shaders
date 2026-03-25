use bevy::{
    image::{ImageAddressMode, ImageLoaderSettings, ImageSampler, ImageSamplerDescriptor},
    prelude::*,
    render::render_resource::AsBindGroup,
    shader::ShaderRef,
    ui_widgets::ValueChange,
};

use crate::dev_ui::{DevUISliderProps, Panel, labeled_slider};

pub struct ShieldPlugin;

impl Plugin for ShieldPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((MaterialPlugin::<CustomMaterial>::default()))
            .add_systems(Startup, (setup, spawn_dev_sliders).chain());
    }
}

#[derive(Asset, TypePath, AsBindGroup, Clone)]
struct CustomMaterial {
    #[uniform(0)]
    color: LinearRgba,
    // Remove textureif unused
    #[texture(1)]
    #[sampler(2)]
    noise_texture: Option<Handle<Image>>,
    #[uniform(3)]
    shield_radius: f32,
    #[uniform(4)]
    shield_color_intensity: f32,
    alpha_mode: AlphaMode,
}

impl Material for CustomMaterial {
    fn vertex_shader() -> ShaderRef {
        "shaders/02_shield.wgsl".into()
    }

    fn fragment_shader() -> ShaderRef {
        "shaders/02_shield.wgsl".into()
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
            "Shield radius",
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
                    mat.shield_radius = event.value;
                }
            },
        ));
        parent.spawn(labeled_slider(
            "Color intensity",
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
                    mat.shield_color_intensity = event.value;
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
            color: LinearRgba::new(0.486, 0.565, 1.0, 1.0),
            noise_texture: Some(asset_server.load_with_settings(
                "noise.png",
                |settings: &mut ImageLoaderSettings| {
                    settings.sampler = ImageSampler::Descriptor(ImageSamplerDescriptor {
                        address_mode_u: ImageAddressMode::Repeat,
                        address_mode_v: ImageAddressMode::Repeat,
                        ..default()
                    });
                },
            )),
            // TODO: Connect with corresponding slider value in the dev UI
            shield_radius: 0.5,
            shield_color_intensity: 0.25,
            alpha_mode: AlphaMode::Blend,
        })),
        Transform::from_xyz(0.0, 0.0, 0.0)
            .with_rotation(Quat::from_rotation_x(std::f32::consts::FRAC_PI_2)),
    ));
}
