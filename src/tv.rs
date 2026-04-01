use bevy::{
    prelude::*, render::render_resource::AsBindGroup, shader::ShaderRef, ui_widgets::ValueChange,
};

use crate::dev_ui::{DevUISliderProps, Panel, labeled_slider};

pub struct TVPlugin;

impl Plugin for TVPlugin {
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
    warp_amount: f32,
    #[uniform(3)]
    lines_count: u32,
    #[uniform(4)]
    lines_opacity: f32,
    #[uniform(5)]
    lines_speed: f32,
    #[uniform(6)]
    vignette_strength: f32,

    alpha_mode: AlphaMode,
}

impl Material for CustomMaterial {
    fn vertex_shader() -> ShaderRef {
        "shaders/22_tv.wgsl".into()
    }

    fn fragment_shader() -> ShaderRef {
        "shaders/22_tv.wgsl".into()
    }

    fn alpha_mode(&self) -> AlphaMode {
        self.alpha_mode
    }

    fn reads_view_transmission_texture(&self) -> bool {
        true
    }
}

#[derive(Message, Debug)]
enum DevUIControlMessage {
    WarpAmountChanged(f32),
    LinesCountChanged(u32),
    LinesOpacityChanged(f32),
    LinesSpeedChanged(f32),
    VignetteStrengthChanged(f32),
}

fn spawn_dev_sliders(mut commands: Commands, panel: Single<Entity, With<Panel>>) {
    commands.entity(*panel).with_children(|parent| {
        parent.spawn(labeled_slider(
            "Warp amount",
            DevUISliderProps {
                value: 0.4,
                min: 0.0,
                max: 1.0,
                step: 0.01,
            },
            |event: On<ValueChange<f32>>,
             mut message_writer: MessageWriter<DevUIControlMessage>| {
                message_writer.write(DevUIControlMessage::WarpAmountChanged(event.value));
            },
        ));
        parent.spawn(labeled_slider(
            "Lines count",
            DevUISliderProps {
                value: 100.0,
                min: 0.0,
                max: 1000.0,
                step: 1.0,
            },
            |event: On<ValueChange<f32>>,
             mut message_writer: MessageWriter<DevUIControlMessage>| {
                message_writer.write(DevUIControlMessage::LinesCountChanged(event.value as u32));
            },
        ));
        parent.spawn(labeled_slider(
            "Lines opacity",
            DevUISliderProps {
                value: 0.4,
                min: 0.0,
                max: 1.0,
                step: 0.01,
            },
            |event: On<ValueChange<f32>>,
             mut message_writer: MessageWriter<DevUIControlMessage>| {
                message_writer.write(DevUIControlMessage::LinesOpacityChanged(event.value));
            },
        ));
        parent.spawn(labeled_slider(
            "Lines speed",
            DevUISliderProps {
                value: 5.0,
                min: 0.0,
                max: 20.0,
                step: 0.1,
            },
            |event: On<ValueChange<f32>>,
             mut message_writer: MessageWriter<DevUIControlMessage>| {
                message_writer.write(DevUIControlMessage::LinesSpeedChanged(event.value));
            },
        ));
        parent.spawn(labeled_slider(
            "Vignette strength",
            DevUISliderProps {
                value: 0.2,
                min: 0.0,
                max: 1.0,
                step: 0.01,
            },
            |event: On<ValueChange<f32>>,
             mut message_writer: MessageWriter<DevUIControlMessage>| {
                message_writer.write(DevUIControlMessage::VignetteStrengthChanged(event.value));
            },
        ));
    });
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut custom_materials: ResMut<Assets<CustomMaterial>>,
    asset_server: Res<AssetServer>,
) {
    commands.spawn((
        Mesh3d(meshes.add(Plane3d::default().mesh().size(1.0, 1.0).subdivisions(8))),
        MeshMaterial3d(custom_materials.add(CustomMaterial {
            texture: Some(asset_server.load("godot_icon.png")),
            warp_amount: 0.4,
            lines_count: 100,
            lines_opacity: 0.4,
            lines_speed: 5.0,
            vignette_strength: 0.0,
            alpha_mode: AlphaMode::Blend,
        })),
        Transform::from_xyz(0.0, 0.0, 0.0)
            .with_rotation(Quat::from_rotation_x(std::f32::consts::FRAC_PI_2)),
    ));
}

fn on_dev_control_change(
    mut messages: MessageReader<DevUIControlMessage>,
    mut custom_materials: ResMut<Assets<CustomMaterial>>,
    material: Single<&MeshMaterial3d<CustomMaterial>>,
) {
    for message in messages.read() {
        if let Some(material) = custom_materials.get_mut(&material.0) {
            match message {
                DevUIControlMessage::LinesCountChanged(count) => {
                    material.lines_count = *count;
                }
                DevUIControlMessage::LinesOpacityChanged(opacity) => {
                    material.lines_opacity = *opacity;
                }
                DevUIControlMessage::LinesSpeedChanged(speed) => {
                    material.lines_speed = *speed;
                }
                DevUIControlMessage::VignetteStrengthChanged(strength) => {
                    material.vignette_strength = *strength;
                }
                DevUIControlMessage::WarpAmountChanged(warp_amount) => {
                    material.warp_amount = *warp_amount;
                }
            }
        }
    }
}
