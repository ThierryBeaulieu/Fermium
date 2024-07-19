use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use rand::random;

// variables
pub const SNAKE_SPEED: f32 = 32.0;

// components
#[derive(Component)]
pub struct Snake {
    pub direction: Vec3,
}

#[derive(Component)]
pub struct SnakeTimer(Timer);

#[derive(Component)]
pub struct Food {}

// systems
pub fn spawn_camera(mut commands: Commands, window_query: Query<&Window, With<PrimaryWindow>>) {
    let window = window_query.get_single().unwrap();

    commands.spawn(Camera2dBundle {
        transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0),
        ..default()
    });
}

pub fn spawn_food(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    let window = window_query.get_single().unwrap();

    let random_x = random::<f32>() * window.width();
    let random_y = random::<f32>() * window.height();

    commands.spawn((
        SpriteBundle {
            transform: Transform {
                translation: Vec3::new(random_x, random_y, 0.0),
                scale: Vec3::new(0.3, 0.3, 1.0),
                ..default()
            },
            texture: asset_server.load("sprites/food.png"),
            ..default()
        },
        Food {},
    ));
}

pub fn spawn_snake(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    let window = window_query.get_single().unwrap();

    // todo ceil or floor it so that it can locked in a tile
    let random_x = random::<f32>() * window.width();
    let random_y = random::<f32>() * window.height();

    let transform = Transform {
        translation: Vec3::new(random_x, random_y, 0.0),
        scale: Vec3::new(0.3, 0.3, 1.0),
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
            direction: Vec3::new(1.0, 0.0, 0.0),
        },
    );

    commands
        .spawn(snake_bundle)
        .insert(SnakeTimer(Timer::from_seconds(0.2, TimerMode::Repeating)));
}

// todo, the snake needs to move in a grid the size of the tile needs to be the same as the sprite size

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
