use bevy::prelude::*;
use std::fs::File;
use std::io::Read;

pub const TILE_WIDTH: f32 = 100.0;
pub const TILE_HEIGHT: f32 = 100.0;

#[derive(Debug)]
pub struct MapBuilder {
    pub rows: Vec<String>,
    pub width: usize,
    pub height: usize,
}

impl MapBuilder {
    pub fn new(file: &str) -> Self {
        let rows = file_rows_to_vec(file);
        let width = rows[0].len();
        let height = rows.len();
        MapBuilder {
            rows,
            width,
            height,
        }
    }
}

pub fn file_rows_to_vec(file: &str) -> Vec<String> {
    let mut file = File::open(file).expect("File not found");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Error reading file");
    let mut rows: Vec<String> = Vec::new();
    for row in contents.lines() {
        rows.push(row.to_string());
    }
    rows
}

pub fn build_map(map: &MapBuilder, commands: &mut Commands) {
    let left_edge = -1.0 * (crate::SCREEN_WIDTH / 2.0) + (TILE_WIDTH / 2.0);
    let mut x = left_edge;
    let mut y = (crate::SCREEN_HEIGHT / 2.0) - (TILE_HEIGHT / 2.0);
    for row in map.rows.iter() {
        for tile in row.chars() {
            match tile {
                'X' | 'x' => {
                    commands.spawn(SpriteBundle {
                        sprite: Sprite {
                            color: Color::rgb(1.0, 1.0, 1.0),
                            custom_size: Some(Vec2::new(TILE_WIDTH, TILE_HEIGHT)),
                            ..default()
                        },
                        transform: Transform::from_translation(Vec3::new(x, y, 0.)),
                        ..default()
                    });
                }
                _ => {}
            }
            x += TILE_WIDTH;
        }
        x = left_edge;
        y -= TILE_HEIGHT;
    }
}
