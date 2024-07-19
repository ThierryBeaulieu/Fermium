use crate::game::snake::systems::SNAKE_SPEED;
use bevy::prelude::*;
use rand::random;

pub fn get_random_coordinates(window_width: f32, window_height: f32) -> Vec2 {
    let spawn_position_x =
        ((window_width / SNAKE_SPEED).floor() * random::<f32>()).ceil() * SNAKE_SPEED;
    let spawn_position_y =
        ((window_height / SNAKE_SPEED).floor() * random::<f32>()).ceil() * SNAKE_SPEED;

    return Vec2::new(spawn_position_x, spawn_position_y);
}
