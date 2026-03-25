use bevy::{
    prelude::*, render::render_resource::AsBindGroup, shader::ShaderRef, ui_widgets::ValueChange,
};

use crate::dev_ui::{ColorChannel, DevUISliderProps, Panel, labeled_color_slider, labeled_slider};

pub struct ColorChangePlugin;

impl Plugin for ColorChangePlugin {
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
    image_texture: Option<Handle<Image>>,
    #[uniform(2)]
    colors: [LinearRgba; 4],
    alpha_mode: AlphaMode,
}

impl Material for CustomMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/06_color_change.wgsl".into()
    }

    fn alpha_mode(&self) -> AlphaMode {
        self.alpha_mode
    }
}

#[derive(Debug)]
enum MessageColorChannel {}

#[derive(Message, Debug)]
enum DevUIControlMessage {
    // WidthChanged(f32),
}

fn spawn_dev_sliders(mut commands: Commands, q_panel: Query<Entity, With<Panel>>) {
    let Ok(panel) = q_panel.single() else {
        return;
    };
    commands.entity(panel).with_children(|parent| {
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
                // message_writer.write(DevUIControlMessage::BlurChanged(event.value));
            },
        ));
        parent.spawn(labeled_color_slider(
            "Outer color hue",
            ColorChannel::HslHue,
            |event: On<ValueChange<f32>>,
             mut message_writer: MessageWriter<DevUIControlMessage>| {
                // message_writer.write(DevUIControlMessage::OuterColorChanged(
                //     MessageColorChannel::Hue,
                //     event.value,
                // ));
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
            image_texture: Some(asset_server.load("blob.png")),
            colors: [
                LinearRgba::new(1.0, 0.0, 0.0, 1.0),
                LinearRgba::new(0.0, 1.0, 0.0, 1.0),
                LinearRgba::new(0.0, 0.0, 1.0, 1.0),
                LinearRgba::new(1.0, 1.0, 0.0, 1.0),
            ],
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
    // for message in messages.read() {
    //     match message {
    //         DevUIControlMessage::SpeedChanged(value) => {
    //             if let Some(mat) = materials.get_mut(&material.0) {
    //                 mat.speed = *value;
    //             }
    //         }
    //     }
    // }
}
