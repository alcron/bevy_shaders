use bevy::{
    feathers::{
        FeathersPlugins,
        controls::{SliderProps, slider},
        dark_theme::create_dark_theme,
        theme::{ThemeBackgroundColor, ThemedText, UiTheme},
        tokens,
    },
    image::{ImageAddressMode, ImageLoaderSettings, ImageSampler, ImageSamplerDescriptor},
    prelude::*,
    render::render_resource::AsBindGroup,
    shader::ShaderRef,
    ui_widgets::{SliderPrecision, SliderStep, ValueChange, observe, slider_self_update},
};

pub struct ShieldPlugin;

impl Plugin for ShieldPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((MaterialPlugin::<CustomMaterial>::default(), FeathersPlugins))
            .insert_resource(UiTheme(create_dark_theme()))
            .add_systems(Startup, setup)
            .add_observer(update_dev_ui);
    }
}

#[derive(Component, PartialEq)]
enum DevUI {
    ShieldRadius,
    ShieldColorIntensity,
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

    // right side panel with slider
    commands.spawn((
        Node {
            position_type: PositionType::Absolute,
            right: Val::Px(0.0),
            top: Val::Px(0.0),
            width: Val::Px(150.0),
            height: Val::Percent(100.0),
            flex_direction: FlexDirection::Column,
            margin: UiRect::bottom(Val::Auto),
            padding: UiRect::all(Val::Px(10.0)),
            row_gap: Val::Px(8.0),
            ..default()
        },
        ThemeBackgroundColor(tokens::WINDOW_BG),
        // Wraps in a child to prevent slider expanding to the full height of the panel
        children![(
            Node {
                display: Display::Flex,
                flex_direction: FlexDirection::Column,
                row_gap: Val::Px(4.0),
                ..default()
            },
            children![
                (
                    Text::new("Shield radius"),
                    ThemedText,
                    TextFont {
                        font_size: 10.0,
                        ..default()
                    }
                ),
                (
                    slider(
                        SliderProps {
                            value: 0.5,
                            min: 0.0,
                            max: 1.0
                        },
                        (SliderStep(0.1), SliderPrecision(2), DevUI::ShieldRadius)
                    ),
                    observe(slider_self_update),
                ),
                (
                    Text::new("Shield color intensity"),
                    ThemedText,
                    TextFont {
                        font_size: 10.0,
                        ..default()
                    }
                ),
                (
                    slider(
                        SliderProps {
                            value: 0.5,
                            min: 0.0,
                            max: 1.0
                        },
                        (
                            SliderStep(0.1),
                            SliderPrecision(2),
                            DevUI::ShieldColorIntensity
                        )
                    ),
                    observe(slider_self_update),
                ),
            ],
        )],
    ));
}

fn update_dev_ui(
    event: On<ValueChange<f32>>,
    q_slider: Query<&DevUI>,
    mut materials: ResMut<Assets<CustomMaterial>>,
    q_material: Query<&MeshMaterial3d<CustomMaterial>>,
) {
    let Ok(dev_ui) = q_slider.get(event.source) else {
        return;
    };
    for mat_handle in &q_material {
        if let Some(mat) = materials.get_mut(mat_handle) {
            match dev_ui {
                DevUI::ShieldRadius => mat.shield_radius = event.value,
                DevUI::ShieldColorIntensity => {
                    mat.shield_color_intensity = (1.0 - event.value) / 2.0
                }
            }
        }
    }
}
