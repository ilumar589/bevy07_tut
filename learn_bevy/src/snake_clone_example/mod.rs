use bevy::prelude::{Commands, Plugin, Component, Color, SpriteBundle, Sprite, Transform, Vec3, Query, With};
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

#[derive(Component)]
struct SnakeHead;

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
            transform: Transform {
                scale: Vec3::new(10.0, 10.0, 10.0),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(SnakeHead);
}

fn snake_movement(mut query: Query<&mut Transform, With<SnakeHead>>) {
    for mut transform in query.iter_mut() {
        transform.translation.y += 2.;
    }
}