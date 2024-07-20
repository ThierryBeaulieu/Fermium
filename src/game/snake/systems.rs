use crate::game::food::components::Food;
use crate::game::systems_helper::get_random_coordinates;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;

use super::components::*;

pub const SNAKE_SPEED: f32 = 50.0;
pub const SNAKE_SIZE: f32 = 50.0;
pub const FOOD_SIZE: f32 = 50.0;
pub const GAME_BOARD_OFFSET: Vec3 = Vec3::new(50.0, 100.0, 0.0);

pub struct GameBoard {
    width: f32,
    height: f32,
}

pub const GAME_BOARD: GameBoard = GameBoard {
    width: 1400.0,
    height: 800.0,
};

pub fn spawn_board(mut commands: Commands, asset_server: Res<AssetServer>) {
    let transform = Transform {
        translation: Vec3::new(
            GAME_BOARD_OFFSET.x + GAME_BOARD.width / 2.0,
            GAME_BOARD_OFFSET.y + GAME_BOARD.height / 2.0,
            0.0,
        ),
        ..default()
    };

    let texture = asset_server.load("sprites/game_board.png");

    let board_bundle: (SpriteBundle, Board) = (
        SpriteBundle {
            transform: transform,
            texture: texture,
            ..default()
        },
        Board {},
    );
    commands.spawn(board_bundle);
}

pub fn spawn_snake(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    // tbeaulieu2: todo ensure that the snake spawns inside the board
    let window = window_query.get_single().unwrap();

    let coord = get_random_coordinates(window.width(), window.height());

    let transform = Transform {
        translation: Vec3::new(coord.x, coord.y, 0.0),
        ..default()
    };

    let texture = asset_server.load("sprites/snake_body.png");

    let snake_bundle: (SpriteBundle, Snake) = (
        SpriteBundle {
            transform: transform,
            texture: texture,
            ..default()
        },
        Snake {
            direction: Vec3::new(0.0, 0.0, 0.0),
        },
    );

    commands
        .spawn(snake_bundle)
        .insert(SnakeTimer(Timer::from_seconds(0.2, TimerMode::Repeating)));
}

pub fn input_handler_snake(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut player_query: Query<(&mut Transform, &mut Snake, &mut SnakeTimer), With<Snake>>,
    time: Res<Time>,
) {
    if let Ok((mut transform, mut snake, mut timer)) = player_query.get_single_mut() {
        timer.0.tick(time.delta());

        if timer.0.finished() {
            let mut direction = Vec3::ZERO;

            if keyboard_input.pressed(KeyCode::ArrowLeft) || keyboard_input.pressed(KeyCode::KeyA) {
                direction += Vec3::new(-1.0, 0.0, 0.0);
                snake.direction = Vec3::new(-1.0, 0.0, 0.0);
            } else if keyboard_input.pressed(KeyCode::ArrowLeft)
                || keyboard_input.pressed(KeyCode::KeyS)
            {
                direction += Vec3::new(0.0, -1.0, 0.0);
                snake.direction = Vec3::new(0.0, -1.0, 0.0);
            } else if keyboard_input.pressed(KeyCode::ArrowLeft)
                || keyboard_input.pressed(KeyCode::KeyD)
            {
                direction += Vec3::new(1.0, 0.0, 0.0);
                snake.direction = Vec3::new(1.0, 0.0, 0.0);
            } else if keyboard_input.pressed(KeyCode::ArrowLeft)
                || keyboard_input.pressed(KeyCode::KeyW)
            {
                direction += Vec3::new(0.0, 1.0, 0.0);
                snake.direction = Vec3::new(0.0, 1.0, 0.0);
            } else {
                direction += snake.direction;
            }

            // store sum_time somewhere so that it can be used at all time?
            transform.translation += direction * SNAKE_SPEED;
        }
    }
}

pub fn eat_food(
    mut commands: Commands,
    mut snake_query: Query<&Transform, With<Snake>>,
    food_query: Query<(Entity, &Transform), With<Food>>,
    asset_server: Res<AssetServer>,
) {
    if let Ok(snake_transform) = snake_query.get_single_mut() {
        for (food, food_transform) in food_query.iter() {
            let distance = snake_transform
                .translation
                .distance(food_transform.translation);
            let food_half_width = FOOD_SIZE / 2.0;
            let snake_half_width = SNAKE_SIZE / 2.0;
            if distance < food_half_width + snake_half_width {
                info!("Snake just ate food");
                let sound_effect = asset_server.load("sounds/upgrade_short.ogg");
                commands.spawn(AudioBundle {
                    source: sound_effect,
                    settings: PlaybackSettings::ONCE,
                    ..default()
                });
                commands.entity(food).despawn();
            }
        }
    }
}

pub fn confine_snake_movement(
    mut snake_query: Query<(Entity, &mut Transform), With<Snake>>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    // tbeaulieu2: ensures that the snake won't come out of the screen and the boundaries
    if let Ok((snake, snake_transform)) = snake_query.get_single_mut() {
        let mut faced_border = false;
        if snake_transform.translation.x + SNAKE_SIZE / 2.0 > GAME_BOARD_OFFSET.x + GAME_BOARD.width
            || snake_transform.translation.x < GAME_BOARD_OFFSET.x + SNAKE_SIZE / 2.0
            || snake_transform.translation.y + SNAKE_SIZE / 2.0
                > GAME_BOARD_OFFSET.y + GAME_BOARD.height
            || snake_transform.translation.y < GAME_BOARD_OFFSET.y + SNAKE_SIZE / 2.0
        {
            faced_border = true;
        }

        // tbeaulieu2 todo: afficher avec de l'ombrage les zones dans l'offset

        if faced_border {
            commands.entity(snake).despawn();
            let sound_effect = asset_server.load("sounds/game_over.ogg");
            commands.spawn(AudioBundle {
                source: sound_effect,
                settings: PlaybackSettings::ONCE,
                ..default()
            });
        }
    }
    // todo completed later
}
