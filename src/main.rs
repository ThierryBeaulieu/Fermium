use bevy::prelude::*;

mod game;
mod systems;

use game::GamePlugin;
use systems::*;

// definition of done : https://playsnake.org/

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(ClearColor(Color::linear_rgb(0.9, 0.3, 0.6)))
        .add_plugins(GamePlugin)
        .add_systems(Startup, spawn_camera)
        .run();
}
