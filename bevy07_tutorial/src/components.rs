use bevy::prelude::{Component, Vec2};

// --- Common Components

#[derive(Component)]
pub struct Velocity {
    pub x: f32,
    pub y: f32,
}

#[derive(Component)]
pub struct Movable {
    pub auto_despawn: bool
}

#[derive(Component)]
pub struct Laser;

#[derive(Component)]
pub struct SpriteSize(pub Vec2);

impl From<(f32, f32)> for SpriteSize {
    fn from(val: (f32, f32)) -> Self {
        SpriteSize(Vec2::new(val.0, val.1))
    }
}

// --- Common Components

// --- Player Components

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct FromPlayer;

// --- Player Components

// --- Enemy Components

#[derive(Component)]
pub struct Enemy;

#[derive(Component)]
pub struct FromEnemy;

// --- Enemy Components