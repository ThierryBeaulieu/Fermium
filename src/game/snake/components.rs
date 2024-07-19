use bevy::prelude::*;

#[derive(Component)]
pub struct Snake {
    pub direction: Vec3,
}

#[derive(Component)]
pub struct SnakeTimer(pub Timer);
