use bevy::prelude::*;

mod systems;

use systems::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, spawn_camera)
        .add_systems(Startup, spawn_snake)
        .add_systems(Update, input_handler_snake)
        .run();
}
