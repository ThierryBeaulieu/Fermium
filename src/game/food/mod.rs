use bevy::prelude::*;

pub mod components;

mod systems;
use systems::*;

pub struct FoodPlugin;

impl Plugin for FoodPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_food);
    }
}
