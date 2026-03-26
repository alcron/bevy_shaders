use bevy::prelude::*;

// use crate::crosshair::CrosshairPlugin;
use crate::dev_ui::DevUIPlugin;
// use crate::shield::ShieldPlugin;
// use crate::uv::UVPlugin;
// use crate::dissolve::DissolvePlugin;
// use crate::color_change::ColorChangePlugin;
// use crate::fire::FirePlugin;
// use crate::randomness::RandomnessPlugin;
// use crate::frame_animation::FrameAnimationPlugin;
// use crate::oribit::OrbitPlugin;
use crate::shine::ShinePlugin;

// mod color_change;
// mod crosshair;
mod dev_ui;
// mod dissolve;
// mod fire;
// mod frame_animation;
// mod randomness;
// mod shield;
// mod uv;
// mod oribit;
mod shine;

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
            DevUIPlugin,
            // CrosshairPlugin,
            // ShieldPlugin,
            // UVPlugin,
            // DissolvePlugin,
            // FirePlugin,
            // ColorChangePlugin,
            // RandomnessPlugin,
            // FrameAnimationPlugin,
            // OrbitPlugin,
            ShinePlugin,
        ))
        .add_systems(Startup, setup)
        .add_systems(Update, close_on_esc)
        .run();
}

fn setup(mut commands: Commands) {
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
}

fn close_on_esc(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut app_exit_events: MessageWriter<AppExit>,
) {
    if keyboard_input.just_pressed(KeyCode::Escape) {
        app_exit_events.write(AppExit::Success);
    }
}
