use crate::{map_generator, player};
use bevy::prelude::*;

pub fn check_collision_with_tile(
    query_tiles: Query<&mut Transform, (With<map_generator::MapTile>, Without<player::Player>)>,
    mut query_player: Query<&mut Transform, (With<player::Player>, Without<map_generator::MapTile>)>,
) {
    let mut player_transform = query_player.single_mut();

    for tile_transform in &mut query_tiles.iter() {
        if is_player_overlapping_tile(&player_transform, &tile_transform) {
            println!("Collision detected!");
        }
    }
}

fn is_player_overlapping_tile(player_transform: &Transform, tile_transform: &Transform) -> bool {
    let player_x = player_transform.translation.x;
    let player_y = player_transform.translation.y;
    let tile_x = tile_transform.translation.x;
    let tile_y = tile_transform.translation.y;
    let player_half_width = player::PLAYER_WIDTH / 2.0;
    let player_half_height = player::PLAYER_HEIGHT / 2.0;
    let tile_half_width = map_generator::TILE_WIDTH / 2.0;
    let tile_half_height = map_generator::TILE_HEIGHT / 2.0;
    let player_left = player_x - player_half_width;
    let player_right = player_x + player_half_width;
    let player_top = player_y + player_half_height;
    let player_bottom = player_y - player_half_height;
    let tile_left = tile_x - tile_half_width;
    let tile_right = tile_x + tile_half_width;
    let tile_top = tile_y + tile_half_height;
    let tile_bottom = tile_y - tile_half_height;

    player_left < tile_right
        && player_right > tile_left
        && player_top > tile_bottom
        && player_bottom < tile_top
}
