use bevy::{
    feathers::{
        FeathersPlugins,
        controls::{SliderProps, slider},
        dark_theme::create_dark_theme,
        theme::{ThemeBackgroundColor, ThemedText, UiTheme},
        tokens,
    },
    prelude::*,
    render::render_resource::AsBindGroup,
    shader::ShaderRef,
    ui_widgets::{SliderPrecision, SliderStep, ValueChange, observe, slider_self_update},
};

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    resolution: (500, 300).into(),
                    ..default()
                }),
                ..default()
            }),
            MaterialPlugin::<CustomMaterial>::default(),
            FeathersPlugins,
        ))
        .insert_resource(UiTheme(create_dark_theme()))
        .add_systems(Startup, setup)
        .add_systems(Update, close_on_esc)
        .add_observer(update_radius)
        .run();
}

#[derive(Component, PartialEq)]
enum DevUI {
    MainRadius,
    CircleCutoutWidth,
}

#[derive(Asset, TypePath, AsBindGroup, Clone)]
struct CustomMaterial {
    #[uniform(0)]
    color: LinearRgba,
    #[texture(1)]
    #[sampler(2)]
    color_texture: Option<Handle<Image>>,
    #[uniform(3)]
    center_size: f32,
    #[uniform(4)]
    cutout_width: f32,
    alpha_mode: AlphaMode,
}

impl Material for CustomMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/custom_material.wgsl".into()
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
            color_texture: Some(asset_server.load("icon.png")),
            // TODO: Connect with corresponding slider value in the dev UI
            center_size: 0.05,
            cutout_width: 0.125,
            alpha_mode: AlphaMode::Blend,
        })),
        Transform::from_xyz(0.0, 0.0, 0.0)
            .with_rotation(Quat::from_rotation_x(std::f32::consts::FRAC_PI_2)),
    ));

    // light
    commands.spawn((
        PointLight {
            shadows_enabled: true,
            ..default()
        },
        Transform::from_xyz(4.0, 8.0, 4.0),
    ));

    // camera
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(0.0, 0.0, 2.0).looking_at(Vec3::ZERO, Vec3::Y),
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
                    Text::new("Main radius"),
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
                        (SliderStep(0.1), SliderPrecision(2), DevUI::MainRadius)
                    ),
                    observe(slider_self_update),
                ),
                (
                    Text::new("Circle cutout width"),
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
                            DevUI::CircleCutoutWidth
                        )
                    ),
                    observe(slider_self_update),
                ),
            ],
        )],
    ));
}

fn update_radius(
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
                DevUI::MainRadius => mat.center_size = event.value * 0.1,
                DevUI::CircleCutoutWidth => mat.cutout_width = event.value * 0.25,
            }
        }
    }
}

fn close_on_esc(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut app_exit_events: MessageWriter<AppExit>,
) {
    if keyboard_input.just_pressed(KeyCode::Escape) {
        app_exit_events.write(AppExit::Success);
    }
}
