use bevy::{input::mouse::MouseButtonInput, prelude::*};

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
#[path = "./components/player.rs"]
mod player_component;
#[path = "./components/position.rs"]
mod position;
#[path = "./components/tag.rs"]
mod tag;
#[path = "./components/uid.rs"]
mod uid;
#[path = "./plugins/world/world.rs"]
mod world_plugin;
fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_plugin(player::PlayerPlugin)
        .add_plugin(debug_plugin::DebugPlugin)
        .add_plugin(item_plugin::ItemPlugin)
        .add_plugin(world_plugin::WorldPlugin)
        .add_startup_system(setup_world)
        .add_system(handle_mouse_clicks)
        .add_startup_system_to_stage(StartupStage::PreStartup, load_item_resource)
        .run();
}

#[derive(Component)]
struct FpsText;

fn setup_world(mut commands: Commands) {
    println!("Setting up a game world!");

    commands.spawn(Camera2dBundle::default());
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
