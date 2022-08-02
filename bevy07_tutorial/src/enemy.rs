use crate::{ENEMY_LASER_SIZE, ENEMY_SIZE, EnemyCount, GameTextures, MAX_ENEMIES, SPRITE_SCALE, WinSize};
use bevy::prelude::*;
use rand::{Rng, thread_rng};
use crate::components::{Enemy, FromEnemy, SpriteSize};

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_system( enemy_spawn_bundle);
    }
}

fn enemy_spawn_bundle(
    mut commands: Commands,
    mut enemy_count: ResMut<EnemyCount>,
    game_textures: Res<GameTextures>,
    win_size: Res<WinSize>
) {

    if enemy_count.0 < MAX_ENEMIES {
        // compute x/y
        let  mut rng = thread_rng();
        let w_span = win_size.w / 2. -100.;
        let h_span = win_size.h / 2. -100.;
        let x = rng.gen_range(-w_span..w_span);
        let y = rng.gen_range(-h_span..h_span);

        commands
            .spawn_bundle(SpriteBundle {
                texture: game_textures.enemy.clone(),
                transform: Transform {
                    translation: Vec3::new(x, y, 10.),
                    scale: Vec3::new(SPRITE_SCALE, SPRITE_SCALE, 1.),
                    ..Default::default()
                },
                ..Default::default()
            })
            .insert(Enemy)
            .insert(SpriteSize::from(ENEMY_SIZE));

        enemy_count.0 += 1;
    }
}