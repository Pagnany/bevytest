use bevy::prelude::*;


const GRAVITY: f32 = 9.8 * 10.0;

pub fn gravity (
    mut query_player: Query<(&crate::player::Player, &mut Transform), With<crate::player::Player>>,
    time: Res<Time>,
) {
    let mut player_transform = query_player.single_mut().1;
    player_transform.translation.y -= GRAVITY * time.delta_seconds();

    let mut player_velocity = query_player.single_mut().0.velocity;
}