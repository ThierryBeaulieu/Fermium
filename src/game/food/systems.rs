use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use rand::random;

use super::components::*;
use crate::game::snake::systems::SNAKE_SIZE;

pub fn spawn_food(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    let window = window_query.get_single().unwrap();

    let absolute_pos_x = random::<f32>() * window.width();
    let random_x = absolute_pos_x - (absolute_pos_x % SNAKE_SIZE);

    let absolute_pox_y = random::<f32>() * window.height();
    let random_y = absolute_pox_y - (absolute_pox_y % SNAKE_SIZE);

    commands.spawn((
        SpriteBundle {
            transform: Transform {
                translation: Vec3::new(random_x, random_y, 0.0),
                scale: Vec3::new(0.4, 0.4, 1.0),
                ..default()
            },
            texture: asset_server.load("sprites/food.png"),
            ..default()
        },
        Food {},
    ));
}
