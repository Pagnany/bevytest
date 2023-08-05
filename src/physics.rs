use bevy::prelude::*;

const GRAVITY: f32 = 9.8 * 100.0;

pub fn player_add_velocity_gravity(
    mut query_player: Query<&mut crate::player::Player, With<crate::player::Player>>,
    time_step: Res<FixedTime>,
) {
    match query_player.single_mut().movement {
        crate::player::PlayerMovement::Falling => {
            let mut player = query_player.single_mut();
            player.velocity.y -= GRAVITY * time_step.period.as_secs_f32();
            if player.velocity.y < -500.0 {
                player.velocity.y = -500.0;
            }
        }
        _ => return,
    }
}

pub fn player_update_transform(
    mut query_player: Query<(&crate::player::Player, &mut Transform), With<crate::player::Player>>,
    time: Res<Time>,
) {
    let player_velocity = query_player.single_mut().0.velocity.clone();
    println!("player_velocity: {:?}", player_velocity);
    let mut player_transform = query_player.single_mut().1;

    player_transform.translation.x += player_velocity.x * time.delta_seconds();
    player_transform.translation.y += player_velocity.y * time.delta_seconds();
}
