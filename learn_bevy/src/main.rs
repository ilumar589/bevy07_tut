mod gltf_load_example;
mod orthographic_example;

use bevy::prelude::*;
use crate::gltf_load_example::GltfLoadExamplePlugin;
use crate::orthographic_example::OrthographicExamplePlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        // .add_plugin(GltfLoadExamplePlugin)
        .add_plugin(OrthographicExamplePlugin)
        .run();
}

