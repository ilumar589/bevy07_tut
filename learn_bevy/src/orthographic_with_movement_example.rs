use bevy::core::FixedTimestep;
use bevy::prelude::{Assets, Color, Commands, Mesh, Msaa, OrthographicCameraBundle, PbrBundle, Plugin, PointLightBundle, ResMut, shape, StandardMaterial, Vec3, Component, Res, Input, KeyCode, Query, SystemSet};
use crate::{App, Transform};

pub struct OrthographicMovementExamplePlugin;

impl Plugin for OrthographicMovementExamplePlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(Msaa { samples: 4 })
            .add_startup_system(setup)
            .add_system_set(
                SystemSet::new()
                    .with_run_criteria(FixedTimestep::step(TIME_STEP as f64))
                    .with_system(player_movement_system),
            );
    }
}

const TIME_STEP: f32 = 1.0 / 60.0;


#[derive(Component)]
struct Player {
    speed: f32
}

// setup a simple 3D scene
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>
) {
    // set up camera
    let mut camera = OrthographicCameraBundle::new_3d();
    camera.orthographic_projection.scale = 3.0;
    camera.transform = Transform::from_xyz(5.0, 5.0, 5.0).looking_at(Vec3::ZERO, Vec3::Y);

    // camera
    commands.spawn_bundle(camera);

    // plane
    commands.spawn_bundle(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Plane { size: 10.0 })),
        material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
        ..Default::default()
    });

    // player
    commands.spawn_bundle(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Capsule { radius: 0.2, ..Default::default() })),
        material: materials.add(Color::rgb(0.2, 0.3, 0.6).into()),
        transform: Transform::from_xyz(2.5, 0.5, 1.5),
        ..Default::default()
    })
        .insert(Player { speed: 1.0 });

    // cubes
    commands.spawn_bundle(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
        material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
        transform: Transform::from_xyz(1.5, 0.5, 1.5),
        ..Default::default()
    });
    commands.spawn_bundle(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
        material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
        transform: Transform::from_xyz(1.5, 0.5, -1.5),
        ..Default::default()
    });
    commands.spawn_bundle(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
        material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
        transform: Transform::from_xyz(-1.5, 0.5, 1.5),
        ..Default::default()
    });
    commands.spawn_bundle(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
        material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
        transform: Transform::from_xyz(-1.5, 0.5, -1.5),
        ..Default::default()
    });
    // light
    commands.spawn_bundle(PointLightBundle {
        transform: Transform::from_xyz(3.0, 8.0, 5.0),
        ..Default::default()
    });
}

fn player_movement_system(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&Player, &mut Transform)>,
) {
    let (player, mut transform) = query.single_mut();

    let mut direction_x = 0.0;
    let mut direction_z = 0.0;

    if keyboard_input.pressed(KeyCode::Up) {
        direction_x -= 1.0;
    }

    if keyboard_input.pressed(KeyCode::Down) {
        direction_x += 1.0;
    }

    if keyboard_input.pressed(KeyCode::Right) {
        direction_z -= 1.0;
    }

    if keyboard_input.pressed(KeyCode::Left) {
        direction_z += 1.0;
    }

    let translation = &mut transform.translation;
    // move the paddle horizontally
    translation.x += direction_x * player.speed * TIME_STEP;
    translation.z += direction_z * player.speed * TIME_STEP;
    // bound the paddle within the walls
    translation.x = translation.x.min(380.0).max(-380.0);
    translation.z = translation.z.min(380.0).max(-380.0);
}