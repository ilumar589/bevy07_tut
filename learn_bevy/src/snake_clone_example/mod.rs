use bevy::prelude::{Commands, Plugin, Component, Color, SpriteBundle, Sprite, Transform, Vec3, Query, With, KeyCode, Input, Res, Windows};
use crate::{App, OrthographicCameraBundle};

pub struct SnakeClonePlugin;

impl Plugin for SnakeClonePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_startup_system(setup_camera)
            .add_startup_system(spawn_snake)
            .add_system(snake_movement);
    }
}

const SNAKE_HEAD_COLOR: Color = Color::rgb(0.7, 0.7, 0.7);
const ARENA_WIDTH: u32 = 10;
const ARENA_HEIGHT: u32 = 10;

#[derive(Component)]
struct SnakeHead;

#[derive(Component, Clone, Copy, PartialEq, Eq)]
struct Position {
    x: i32,
    y: i32,
}

#[derive(Component)]
struct Size {
    width: f32,
    height: f32,
}

impl Size {
    pub fn square(x: f32) -> Self {
        Self {
            width: x,
            height: x,
        }
    }
}

fn setup_camera(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
}

fn spawn_snake(mut commands: Commands) {
    commands
        .spawn_bundle(SpriteBundle {
            sprite: Sprite {
                color: SNAKE_HEAD_COLOR,
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(SnakeHead)
        .insert(Position { x: 3, y: 3})
        .insert(Size::square(0.8));
}
fn size_scaling(windows: Res<Windows>,
                mut query: Query<(&mut Transform, &Size)>) {
    let window = windows.get_primary().unwrap();
    for (mut transform, sprite_size) in query.iter_mut() {
        transform.scale = Vec3::new(
            sprite_size.width / ARENA_WIDTH as f32 * window.width() as f32,
            sprite_size.height / ARENA_HEIGHT as f32 * window.height() as f32,
            1.0,
        );
    }
}

fn snake_movement(keyboard_input: Res<Input<KeyCode>>,
                  mut query: Query<&mut Transform, With<SnakeHead>>) {
    for mut transform in query.iter_mut() {
        if keyboard_input.pressed(KeyCode::Left) {
            transform.translation.x -= 2.;
        }
        if keyboard_input.pressed(KeyCode::Right) {
            transform.translation.x += 2.;
        }
        if keyboard_input.pressed(KeyCode::Down) {
            transform.translation.y -= 2.;
        }
        if keyboard_input.pressed(KeyCode::Up) {
            transform.translation.y += 2.;
        }
    }
}