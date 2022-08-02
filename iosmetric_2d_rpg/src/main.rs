use bevy::prelude::{AssetServer, Commands, OrthographicCameraBundle, Res, Transform};
use bevy_ecs_tilemap::{ChunkSize, LayerBuilder, LayerSettings, Map, MapQuery, MapSize, TextureSize, Tile, TileBundle, TileSize};

fn main() {
    println!("Hello, world!");
}


fn startup(mut commands: Commands, asset_server: Res<AssetServer>, mut map_query: MapQuery) {
    commands.spawn_bundle(OrthographicCameraBundle {
        transform: Transform::from_xyz(1042.0, 1024.0, 1000.0 - 0.1),
        ..OrthographicCameraBundle::new_2d()
    });

    let texture_handle = asset_server.load("tiles.png");

    let map_size = MapSize(20, 20);

    let layer_settings = LayerSettings::new(
        map_size,
        ChunkSize(32, 32),
        TileSize(16.0, 16.0),
        TextureSize(96.0, 16.0)
    );

    let (mut layer_builder, layer_0_entity) =
        LayerBuilder::<TileBundle>::new(&mut commands, layer_settings.clone(), 0u16, 0u16);

    layer_builder.set_all(Tile::default().into());

    map_query.build_layer(&mut commands, layer_builder, texture_handle);


}