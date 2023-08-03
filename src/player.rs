use bevy::prelude::*;

const PLAYER_SPEED: f32 = 400.0;

pub const PLAYER_HEIGHT: f32 = 50.0;
pub const PLAYER_WIDTH: f32 = 30.0;

#[derive(Component)]
pub struct Player{
    pub velocity: Vec2,
}

pub fn move_character(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&mut Transform), With<Player>>,
    time_step: Res<FixedTime>,
) {
    let mut player_transform = query.single_mut();
    let mut direction_x = 0.0;
    let mut direction_y = 0.0;

    if keyboard_input.pressed(KeyCode::Left) {
        direction_x -= 1.0;
    }

    if keyboard_input.pressed(KeyCode::Right) {
        direction_x += 1.0;
    }

    if keyboard_input.pressed(KeyCode::Up) {
        direction_y += 1.0;
    }

    if keyboard_input.pressed(KeyCode::Down) {
        direction_y -= 1.0;
    }

    if keyboard_input.pressed(KeyCode::Space) {
        player_transform.translation.y += 10.0;
    }

    player_transform.translation.x += direction_x * PLAYER_SPEED * time_step.period.as_secs_f32();
    player_transform.translation.y += direction_y * PLAYER_SPEED * time_step.period.as_secs_f32();
}
