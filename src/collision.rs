use crate::{map_generator, player};
use bevy::prelude::*;

pub fn check_collision_with_tile(
    query_tiles: Query<&mut Transform, (With<map_generator::MapTile>, Without<player::Player>)>,
    mut query_player: Query<
        &mut Transform,
        (With<player::Player>, Without<map_generator::MapTile>),
    >,
) {
    let mut player_transform = query_player.single_mut();

    for tile_transform in &mut query_tiles.iter() {
        if is_player_overlapping_tile(&player_transform, &tile_transform) {
            let player_left = player_transform.translation.x - player::PLAYER_WIDTH / 2.0;
            let player_right = player_transform.translation.x + player::PLAYER_WIDTH / 2.0;
            let player_top = player_transform.translation.y + player::PLAYER_HEIGHT / 2.0;
            let player_bottom = player_transform.translation.y - player::PLAYER_HEIGHT / 2.0;
            let tile_left = tile_transform.translation.x - map_generator::TILE_WIDTH / 2.0;
            let tile_right = tile_transform.translation.x + map_generator::TILE_WIDTH / 2.0;
            let tile_top = tile_transform.translation.y + map_generator::TILE_HEIGHT / 2.0;
            let tile_bottom = tile_transform.translation.y - map_generator::TILE_HEIGHT / 2.0;

            let mut x_overlap = 0.0;
            let mut y_overlap = 0.0;

            if player_left < tile_right && player_right > tile_left {
                x_overlap = if player_transform.translation.x < tile_transform.translation.x {
                    player_right - tile_left
                } else {
                    tile_right - player_left
                };
            }

            if player_top > tile_bottom && player_bottom < tile_top {
                y_overlap = if player_transform.translation.y < tile_transform.translation.y {
                    player_top - tile_bottom
                } else {
                    tile_top - player_bottom
                };
            }

            if x_overlap < y_overlap {
                if player_transform.translation.x < tile_transform.translation.x {
                    player_transform.translation.x -= x_overlap;
                } else {
                    player_transform.translation.x += x_overlap;
                }
            } else {
                if player_transform.translation.y < tile_transform.translation.y {
                    player_transform.translation.y -= y_overlap;
                } else {
                    player_transform.translation.y += y_overlap;
                }
            }
        }
    }
}

fn is_player_overlapping_tile(player_transform: &Transform, tile_transform: &Transform) -> bool {
    let player_left = player_transform.translation.x - player::PLAYER_WIDTH / 2.0;
    let player_right = player_transform.translation.x + player::PLAYER_WIDTH / 2.0;
    let player_top = player_transform.translation.y + player::PLAYER_HEIGHT / 2.0;
    let player_bottom = player_transform.translation.y - player::PLAYER_HEIGHT / 2.0;
    let tile_left = tile_transform.translation.x - map_generator::TILE_WIDTH / 2.0;
    let tile_right = tile_transform.translation.x + map_generator::TILE_WIDTH / 2.0;
    let tile_top = tile_transform.translation.y + map_generator::TILE_HEIGHT / 2.0;
    let tile_bottom = tile_transform.translation.y - map_generator::TILE_HEIGHT / 2.0;

    player_left < tile_right
        && player_right > tile_left
        && player_top > tile_bottom
        && player_bottom < tile_top
}

pub fn is_player_standing_on_tile(
    query_tiles: Query<&Transform, (With<map_generator::MapTile>, Without<player::Player>)>,
    mut query_player: Query<
        (&mut crate::player::Player, &Transform),
        (With<player::Player>, Without<map_generator::MapTile>),
    >,
) {
    let (mut player, player_transform) = query_player.single_mut();

    for tile_transform in &mut query_tiles.iter() {
        let mut temp_tile_transform = tile_transform.clone();
        temp_tile_transform.translation.y += 1.0;
        if is_player_overlapping_tile(&player_transform, &temp_tile_transform) {
            player.movement = crate::player::PlayerMovement::Idle;
            player.velocity.y = 0.0;
        }else {
            player.movement = crate::player::PlayerMovement::Falling;
        }
    }
}
