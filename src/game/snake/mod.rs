use bevy::prelude::*;

pub mod components;
pub mod systems;

use systems::*;
pub struct SnakePlugin;

impl Plugin for SnakePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_snake)
            .add_systems(Update, input_handler_snake)
            .add_systems(Update, eat_food)
            .add_systems(Update, confine_snake_movement);
    }
}
