mod gltf_load_example;
mod orthographic_example;
mod breakout_game_example;
mod orthographic_with_movement_example;
mod rhythm_game_example;
mod snake_clone_example;

use bevy::prelude::*;
use crate::breakout_game_example::BreakoutGameExamplePlugin;
use crate::gltf_load_example::GltfLoadExamplePlugin;
use crate::orthographic_example::OrthographicExamplePlugin;
use crate::orthographic_with_movement_example::OrthographicMovementExamplePlugin;
use crate::snake_clone_example::SnakeClonePlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        // .add_plugin(GltfLoadExamplePlugin)
        // .add_plugin(OrthographicExamplePlugin)
        // .add_plugin(BreakoutGameExamplePlugin)
        // .add_plugin(OrthographicMovementExamplePlugin)
        .add_plugin(SnakeClonePlugin)
        .run();
}

