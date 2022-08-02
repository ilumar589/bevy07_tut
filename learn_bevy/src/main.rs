mod gltf_load_example;
mod orthographic_example;
mod breakout_game_example;
mod orthographic_with_movement_example;
mod rhythm_game_example;

use bevy::prelude::*;
use crate::breakout_game_example::BreakoutGameExamplePlugin;
use crate::gltf_load_example::GltfLoadExamplePlugin;
use crate::orthographic_example::OrthographicExamplePlugin;
use crate::orthographic_with_movement_example::OrthographicMovementExamplePlugin;
use crate::rhythm_game_example::RhythmGamePlugin;

fn main() {
    App::new()
        // .insert_resource(WindowDescriptor { // <--
        //     title: "Snake!".to_string(), // <--
        //     width: 500.0,                 // <--
        //     height: 500.0,                // <--
        //     ..Default::default()         // <--
        // })
        // .add_plugins(DefaultPlugins)
        // .add_plugin(GltfLoadExamplePlugin)
        // .add_plugin(OrthographicExamplePlugin)
        // .add_plugin(BreakoutGameExamplePlugin)
        // .add_plugin(OrthographicMovementExamplePlugin)
        // .add_plugin(SnakeClonePlugin)
        .add_plugin(RhythmGamePlugin)
        .run();
}

