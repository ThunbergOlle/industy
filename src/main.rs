use bevy::prelude::*;

#[path = "./components/collider.rs"]
mod collider;
#[path = "./plugins/debug/debug.rs"]
mod debug_plugin;
#[path = "./plugins/player/player.rs"]
mod player;
#[path = "./components/position.rs"]
mod position;
#[path = "./components/tag.rs"]
mod tag;
#[path = "./plugins/tilemap/tilemap.rs"]
mod tilemap_plugin;
fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_plugin(player::PlayerPlugin)
        .add_plugin(debug_plugin::DebugPlugin)
        .add_plugin(tilemap_plugin::TileMapPlugin)
        .add_startup_system(setup_world)
        .run();
}

fn setup_world(mut commands: Commands, asset_server: Res<AssetServer>) {
    println!("Setting up a game world!");

    commands.spawn(Camera2dBundle::default());
    // commands
    //     .spawn(SpriteBundle {
    //         texture: asset_server.load("map.png").into(),
    //         transform: Transform {
    //             translation: Vec3::new(0.0, 0.0, 0.0),
    //             scale: Vec3::new(4.0, 4.0, 1.0),
    //             ..Default::default()
    //         },
    //         ..Default::default()
    //     })
    //     .insert(tag::Tag("map".to_string()))
    //     .insert(position::Position { x: 0, y: 0 });
}
