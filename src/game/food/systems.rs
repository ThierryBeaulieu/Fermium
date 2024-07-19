use super::components::*;
use crate::game::systems_helper::get_random_coordinates;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;

pub fn spawn_food(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    let window = window_query.get_single().unwrap();
    let coord = get_random_coordinates(window.width(), window.height());

    commands.spawn((
        SpriteBundle {
            transform: Transform {
                translation: Vec3::new(coord.x, coord.y, 0.0),
                ..default()
            },
            texture: asset_server.load("sprites/test.png"),
            ..default()
        },
        Food {},
    ));
}
