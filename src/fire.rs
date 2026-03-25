use bevy::{
    image::{ImageAddressMode, ImageLoaderSettings, ImageSampler, ImageSamplerDescriptor},
    prelude::*,
    render::render_resource::AsBindGroup,
    shader::ShaderRef,
    ui_widgets::ValueChange,
};

use crate::dev_ui::{ColorChannel, DevUISliderProps, Panel, labeled_color_slider, labeled_slider};

pub struct FirePlugin;

impl Plugin for FirePlugin {
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
    width: f32,
    #[uniform(1)]
    size: f32,
    #[uniform(2)]
    blur: f32,
    #[texture(3)]
    #[sampler(4)]
    noise_texture: Option<Handle<Image>>,
    #[uniform(5)]
    speed: f32,
    #[uniform(6)]
    outer_color: LinearRgba,
    #[uniform(7)]
    mid_color: LinearRgba,
    #[uniform(8)]
    inner_color: LinearRgba,
    alpha_mode: AlphaMode,
}

impl Material for CustomMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/05_fire.wgsl".into()
    }

    fn alpha_mode(&self) -> AlphaMode {
        self.alpha_mode
    }
}

#[derive(Debug)]
enum MessageColorChannel {
    Hue,
    Saturation,
    Lightness,
}

#[derive(Message, Debug)]
enum DevUIControlMessage {
    WidthChanged(f32),
    SizeChanged(f32),
    OuterColorChanged(MessageColorChannel, f32),
    BlurChanged(f32),
    SpeedChanged(f32),
}

fn spawn_dev_sliders(mut commands: Commands, q_panel: Query<Entity, With<Panel>>) {
    let Ok(panel) = q_panel.single() else {
        return;
    };
    commands.entity(panel).with_children(|parent| {
        parent.spawn(labeled_slider(
            "Width",
            DevUISliderProps {
                value: 0.0,
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
            "Size",
            DevUISliderProps {
                value: 0.75,
                min: 0.0,
                max: 1.0,
                step: 0.01,
            },
            |event: On<ValueChange<f32>>,
             mut message_writer: MessageWriter<DevUIControlMessage>| {
                message_writer.write(DevUIControlMessage::SizeChanged(event.value));
            },
        ));
        parent.spawn(labeled_slider(
            "Blur",
            DevUISliderProps {
                value: 0.1,
                min: 0.0,
                max: 1.0,
                step: 0.01,
            },
            |event: On<ValueChange<f32>>,
             mut message_writer: MessageWriter<DevUIControlMessage>| {
                message_writer.write(DevUIControlMessage::BlurChanged(event.value));
            },
        ));
        parent.spawn(labeled_slider(
            "Speed",
            DevUISliderProps {
                value: 0.1,
                min: 0.0,
                max: 1.0,
                step: 0.01,
            },
            |event: On<ValueChange<f32>>,
             mut message_writer: MessageWriter<DevUIControlMessage>| {
                message_writer.write(DevUIControlMessage::SpeedChanged(event.value));
            },
        ));
        parent.spawn(labeled_color_slider(
            "Outer color hue",
            ColorChannel::HslHue,
            |event: On<ValueChange<f32>>,
             mut message_writer: MessageWriter<DevUIControlMessage>| {
                message_writer.write(DevUIControlMessage::OuterColorChanged(
                    MessageColorChannel::Hue,
                    event.value,
                ));
            },
        ));
        parent.spawn(labeled_color_slider(
            "Outer color saturation",
            ColorChannel::HslSaturation,
            |event: On<ValueChange<f32>>,
             mut message_writer: MessageWriter<DevUIControlMessage>| {
                message_writer.write(DevUIControlMessage::OuterColorChanged(
                    MessageColorChannel::Saturation,
                    event.value,
                ));
            },
        ));
        parent.spawn(labeled_color_slider(
            "Outer color lightness",
            ColorChannel::HslLightness,
            |event: On<ValueChange<f32>>,
             mut message_writer: MessageWriter<DevUIControlMessage>| {
                message_writer.write(DevUIControlMessage::OuterColorChanged(
                    MessageColorChannel::Lightness,
                    event.value,
                ));
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
            width: 0.0,
            size: 0.5,
            blur: 0.1,
            speed: 0.5,
            outer_color: Srgba::hex("ff4e1d").unwrap().into(),
            mid_color: Srgba::hex("ffaa00").unwrap().into(),
            inner_color: Srgba::hex("ffcd8b").unwrap().into(),
            alpha_mode: AlphaMode::Blend,
        })),
        Transform::from_xyz(0.0, 0.0, 0.0)
            .with_rotation(Quat::from_rotation_x(std::f32::consts::FRAC_PI_2)),
    ));
}

// TODO: Query Single material withou q_material
fn on_dev_control_change(
    mut messages: MessageReader<DevUIControlMessage>,
    mut materials: ResMut<Assets<CustomMaterial>>,
    material: Single<&MeshMaterial3d<CustomMaterial>>,
) {
    for message in messages.read() {
        match message {
            DevUIControlMessage::WidthChanged(value) => {
                if let Some(mat) = materials.get_mut(&material.0) {
                    mat.width = *value;
                }
            }
            DevUIControlMessage::SizeChanged(value) => {
                if let Some(mat) = materials.get_mut(&material.0) {
                    mat.size = *value;
                }
            }
            DevUIControlMessage::BlurChanged(value) => {
                if let Some(mat) = materials.get_mut(&material.0) {
                    mat.blur = *value;
                }
            }
            DevUIControlMessage::OuterColorChanged(channel, value) => {
                if let Some(mat) = materials.get_mut(&material.0) {
                    let mut new_color_hsl: Hsla = mat.outer_color.into();

                    match channel {
                        MessageColorChannel::Hue => new_color_hsl.hue = *value,
                        MessageColorChannel::Saturation => new_color_hsl.saturation = *value,
                        MessageColorChannel::Lightness => new_color_hsl.lightness = *value,
                    }

                    mat.outer_color = new_color_hsl.into();
                }
            }
            DevUIControlMessage::SpeedChanged(value) => {
                if let Some(mat) = materials.get_mut(&material.0) {
                    mat.speed = *value;
                }
            }
        }
    }
}
