use bevy::prelude::*;

#[path = "./components/collider.rs"]
mod collider;
#[path = "./plugins/debug/debug.rs"]
mod debug_plugin;
#[path = "./plugins/item/item.rs"]
mod item_plugin;
#[path = "./resources/item_sheet.rs"]
mod item_resource;
#[path = "./plugins/player/player.rs"]
mod player;
#[path = "./components/position.rs"]
mod position;
#[path = "./components/tag.rs"]
mod tag;
#[path = "./plugins/world/world.rs"]
mod world_plugin;
fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_startup_system(setup_world)
        .add_startup_system_to_stage(StartupStage::PreStartup, load_item_resource)
        .add_plugin(player::PlayerPlugin)
        .add_plugin(debug_plugin::DebugPlugin)
        .add_plugin(item_plugin::ItemPlugin)
        .add_plugin(world_plugin::WorldPlugin)
        .run();
}

fn setup_world(mut commands: Commands, _asset_server: Res<AssetServer>) {
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

// load the resource sheet as a resource
fn load_item_resource(
    mut commands: Commands,
    assets: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let image = assets.load("itemsheet.png");
    let texture_atlas = TextureAtlas::from_grid(image, Vec2::splat(16.), 8, 8, None, None);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);
    commands.insert_resource(item_resource::ItemSheet(texture_atlas_handle));
}
