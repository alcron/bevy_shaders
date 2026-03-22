use bevy::{
    feathers::{
        FeathersPlugins,
        controls::{SliderProps, slider},
        dark_theme::create_dark_theme,
        theme::{ThemeBackgroundColor, ThemedText, UiTheme},
        tokens,
    },
    prelude::*,
    ui_widgets::{SliderPrecision, SliderStep, ValueChange, observe, slider_self_update},
};

pub struct DevUIPlugin;

impl Plugin for DevUIPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(FeathersPlugins)
            .insert_resource(UiTheme(create_dark_theme()))
            .add_message::<DevUIMessage>()
            .add_systems(Startup, setup);
    }
}

#[derive(Component)]
pub struct Panel;

#[derive(Message)]
pub struct DevUIMessage {
    pub entity: Entity,
    pub value: f32,
}

#[derive(Default)]
pub struct DevUISliderProps {
    pub value: f32,
    pub min: f32,
    pub max: f32,
    pub step: f32,
}

pub fn labeled_slider(label: &str, props: DevUISliderProps, extra: impl Bundle) -> impl Bundle {
    (
        Node {
            display: Display::Flex,
            flex_direction: FlexDirection::Column,
            row_gap: Val::Px(4.0),
            ..default()
        },
        children![
            (
                Text::new(label.to_string()),
                ThemedText,
                TextFont {
                    font_size: 10.0,
                    ..default()
                }
            ),
            (
                slider(
                    SliderProps {
                        value: props.value,
                        min: props.min,
                        max: props.max
                    },
                    (SliderStep(props.step), SliderPrecision(2), extra)
                ),
                observe(slider_self_update),
                observe(on_slider_change),
            ),
        ],
    )
}

fn on_slider_change(event: On<ValueChange<f32>>, mut mw: MessageWriter<DevUIMessage>) {
    mw.write(DevUIMessage {
        entity: event.source,
        value: event.value,
    });
}

fn setup(mut commands: Commands) {
    // right side panel with sliders
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
        children![],
        Panel,
    ));
}
