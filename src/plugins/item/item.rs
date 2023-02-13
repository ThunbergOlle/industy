use bevy::{
    prelude::{App, AssetServer, Assets, Commands, Plugin, Res, ResMut, StartupStage, Vec2},
    sprite::TextureAtlas,
};

#[path = "../../resources/item_sheet.rs"]
mod item_resource;

pub struct ItemPlugin;

impl Plugin for ItemPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system_to_stage(StartupStage::PreStartup, load_item_resource);
    }
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
