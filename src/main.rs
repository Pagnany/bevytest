use bevy::{
    diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin},
    prelude::*,
    window::{PresentMode, WindowTheme},
};

mod collision;
mod map_generator;
mod physics;
mod player;

pub const SCREEN_WIDTH: f32 = 1280.0;
pub const SCREEN_HEIGHT: f32 = 720.0;

const TICK_TIME: f32 = 1.0 / 50.0;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    title: "Game".into(),
                    resolution: (SCREEN_WIDTH, SCREEN_HEIGHT).into(),
                    present_mode: PresentMode::AutoVsync,
                    window_theme: Some(WindowTheme::Dark),
                    ..default()
                }),
                ..default()
            }),
            FrameTimeDiagnosticsPlugin::default(),
        ))
        .insert_resource(FixedTime::new_from_secs(TICK_TIME))
        .add_systems(Startup, setup)
        .add_systems(
            FixedUpdate,
            (
                text_update_system,
                physics::gravity,
                player::move_character,
                collision::check_collision_with_tile,
            ),
        )
        .add_systems(Update, bevy::window::close_on_esc)
        .run();
}

#[derive(Component)]
struct FpsText;

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());

    commands.spawn((
        // Create a TextBundle that has a Text with a list of sections.
        TextBundle::from_sections([
            TextSection::new(
                "FPS: ",
                TextStyle {
                    font_size: 20.0,
                    color: Color::WHITE,
                    ..Default::default()
                },
            ),
            TextSection::from_style(TextStyle {
                font_size: 20.0,
                color: Color::GOLD,
                ..Default::default()
            }),
        ]),
        FpsText,
    ));

    // Player
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(0.25, 0.25, 0.75),
                custom_size: Some(Vec2::new(player::PLAYER_WIDTH, player::PLAYER_HEIGHT)),
                ..default()
            },
            transform: Transform::from_translation(Vec3::new(0., 0., 0.)),
            ..default()
        },
        player::Player{
            velocity: Vec2::new(0., 0.),
        },
    ));

    // Map
    let map = map_generator::MapBuilder::new("maps/map1.txt");
    map_generator::build_map(&map, &mut commands);
}

fn text_update_system(
    diagnostics: Res<DiagnosticsStore>,
    mut query: Query<&mut Text, With<FpsText>>,
) {
    for mut text in &mut query {
        if let Some(fps) = diagnostics.get(FrameTimeDiagnosticsPlugin::FPS) {
            if let Some(value) = fps.smoothed() {
                // Update the value of the second section
                text.sections[1].value = format!("{value:.2}");
            }
        }
    }
}
