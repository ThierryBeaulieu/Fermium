use bevy::prelude::*;

mod game;
mod systems;

use game::GamePlugin;
use systems::*;

// definition of done : https://playsnake.org/

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(ClearColor(Color::linear_rgb(
            185.0 / 255.0,
            152.0 / 255.0,
            102.0 / 255.0,
        ))) //185, 152, 102
        .add_plugins(GamePlugin)
        .add_systems(Startup, spawn_camera)
        .run();
}
