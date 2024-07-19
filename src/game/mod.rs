use bevy::prelude::*;

pub mod food;
pub mod snake;

pub mod systems_helper;

use food::FoodPlugin;
use snake::SnakePlugin;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(SnakePlugin).add_plugins(FoodPlugin);
    }
}
