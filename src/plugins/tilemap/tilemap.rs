use std::fs::File;

use bevy::{
    prelude::{
        App, AssetServer, Assets, Commands, Handle, Plugin, Res, ResMut, Resource, StartupStage,
        Transform, Vec2, Vec3,
    },
    sprite::{SpriteSheetBundle, TextureAtlas, TextureAtlasSprite},
};
use serde::{Deserialize, Serialize};

pub struct TileMapPlugin;

impl Plugin for TileMapPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system_to_stage(StartupStage::PreStartup, load_tile_map_resource)
            .add_startup_system(create_tile_map);
    }
}
#[derive(Resource, Default, Debug)]
pub struct TileMapSheet(Handle<TextureAtlas>);

#[derive(Serialize, Deserialize, Debug)]
struct TileMapJSON {
    tilemap: Vec<Vec<u32>>,
    name: String,
}
fn load_map_data() -> TileMapJSON {
    let file = File::open("./assets/map.json").expect("Unable to open file map file");
    // read the json file:
    let tile_map_json: TileMapJSON =
        serde_json::from_reader(file).expect("Unable to read JSON file");

    tile_map_json
}
fn load_tile_map_resource(
    mut commands: Commands,
    assets: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let image = assets.load("landscape.png");
    let texture_atlas = TextureAtlas::from_grid(image, Vec2::splat(16.), 15, 12, None, None);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);
    commands.insert_resource(TileMapSheet(texture_atlas_handle));
}
fn create_tile_map(mut commands: Commands, tilemap: Res<TileMapSheet>) {
    let map = load_map_data();
    let tiles = map.tilemap;
    for (y, row) in tiles.iter().enumerate() {
        for (x, tile) in row.iter().enumerate() {
            // instantiate a sprite for each tile
            spawn_tile(&mut commands, &tilemap, *tile, x as u32, y as u32);
        }
    }
}

pub enum TileType {
    Grass,
    Water,
}
trait TileTypeTrait {
    fn to_type_id(&self) -> u32;
    fn from_type_id(type_id: u32) -> TileType;
    fn to_sprite_index(&self) -> usize;
}
impl TileTypeTrait for TileType {
    fn from_type_id(type_id: u32) -> TileType {
        match type_id {
            0 => TileType::Water,
            1 => TileType::Grass,
            _ => TileType::Grass,
        }
    }
    fn to_type_id(&self) -> u32 {
        match self {
            TileType::Grass => 1,
            TileType::Water => 0,
        }
    }
    fn to_sprite_index(&self) -> usize {
        match self {
            TileType::Grass => 33,
            TileType::Water => 27,
        }
    }
}
fn spawn_tile(commands: &mut Commands, tilemap: &TileMapSheet, tile_type_id: u32, x: u32, y: u32) {
    let tile_type = TileType::from_type_id(tile_type_id);
    let sprite_index = TileType::to_sprite_index(&tile_type);

    let sprite = TextureAtlasSprite {
        index: sprite_index,
        ..Default::default()
    };
    commands.spawn(SpriteSheetBundle {
        texture_atlas: tilemap.0.clone(),
        sprite,
        transform: Transform {
            translation: Vec3::new((x as f32) * 4. * 16., (y as f32) * 4. * 16., 0.0),
            scale: Vec3::new(4.0, 4.0, 1.0),
            ..Default::default()
        },
        ..Default::default()
    });
}

pub fn tile_type_to_sprite_index(tile_type: TileType) -> usize {
    match tile_type {
        TileType::Grass => 33,
        TileType::Water => 27,
        _ => 0,
    }
}
pub fn tile_type_id_to_tile_type(tile_type_id: u32) -> TileType {
    match tile_type_id {
        0 => TileType::Water,
        1 => TileType::Grass,
        _ => TileType::Grass,
    }
}
