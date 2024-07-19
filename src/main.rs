use bevy::prelude::*;

mod game;
mod systems;

use game::GamePlugin;
use systems::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(GamePlugin)
        .add_systems(Startup, spawn_camera)
        .run();
}
