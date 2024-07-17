use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use rand::random;

// variables

pub const SNAKE_SPEED: f32 = 500.0;

// components

#[derive(Component)]
pub struct Snake {
    pub direction: Vec3,
}

// systems

pub fn spawn_camera(mut commands: Commands, window_query: Query<&Window, With<PrimaryWindow>>) {
    let window = window_query.get_single().unwrap();

    commands.spawn(Camera2dBundle {
        transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0),
        ..default()
    });
}

pub fn spawn_snake(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    let window = window_query.get_single().unwrap();

    let random_x = random::<f32>() * window.width();
    let random_y = random::<f32>() * window.height();

    let snake_bundle: (SpriteBundle, Snake) = (
        SpriteBundle {
            transform: Transform::from_xyz(random_x, random_y, 0.0),
            texture: asset_server.load("sprites/snake_body.png"),
            ..default()
        },
        Snake {
            direction: Vec3::new(1.0, 0.0, 0.0),
        },
    );

    commands.spawn(snake_bundle);
}

pub fn input_handler_snake(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut player_query: Query<(&mut Transform, &mut Snake), With<Snake>>,
    time: Res<Time>,
) {
    if let Ok((mut transform, mut snake)) = player_query.get_single_mut() {
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
            direction += snake.direction
            // todo: if a direction hasn't been set before,
            // continue using the direction that was planned before
        }
        transform.translation += direction * SNAKE_SPEED * time.delta_seconds();
    }
}
