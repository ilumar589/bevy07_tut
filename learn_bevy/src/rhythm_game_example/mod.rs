use bevy::DefaultPlugins;
use bevy::input::system::exit_on_esc_system;
use bevy::prelude::{App, Msaa, Plugin, WindowDescriptor};

pub struct RhythmGamePlugin;

impl Plugin for RhythmGamePlugin {
    fn build(&self, app: &mut App) {
        app
            // Set antialiasing to use 4 samples
            // .insert_resource(Msaa { samples: 4 })
            // Set WindowDescriptor Resource to change title and size
            .insert_resource(WindowDescriptor {
                title: "Rhythm!".to_string(),
                width: 800.,
                height: 600.,
                ..Default::default()
            })
            .add_plugins(DefaultPlugins)
            .add_system(exit_on_esc_system);
    }
}