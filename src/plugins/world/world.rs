use std::fs::File;

use crate::item_resource::ItemSheet;
use bevy::{
    prelude::{
        App, AssetServer, Assets, Commands, Handle, Mesh, Plugin, Res, ResMut, Resource,
        StartupStage, Transform, Vec2, Vec3,
    },
    sprite::{ColorMaterial, SpriteSheetBundle, TextureAtlas, TextureAtlasSprite},
};
use serde::{Deserialize, Serialize};

mod resource;

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system_to_stage(StartupStage::PreStartup, load_tile_map_resource)
            .add_startup_system(create_tile_map);
    }
}
#[derive(Resource, Default, Debug)]
pub struct TileMapSheet(Handle<TextureAtlas>);

#[derive(Serialize, Deserialize, Debug)]
pub struct TileMapJSON {
    pub world: Vec<Chunk>,
    pub name: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Chunk {
    x: i32,
    y: i32,
    background_layer: Vec<Vec<u32>>,
    foreground_layer: Vec<Vec<u32>>,
    resources: Vec<ItemResource>,
}
#[derive(Serialize, Deserialize, Debug)]
struct ItemResource {
    item_id: u32,
    uid: String,
    local_y: i32,
    local_x: i32,
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
fn create_tile_map(
    mut commands: Commands,
    tilemap: Res<TileMapSheet>,
    item_sheet: Res<ItemSheet>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let map = load_map_data();
    let chunk = &map.world;

    for chunk in chunk {
        let x_offset = chunk.x;
        let y_offset = chunk.y;
        let background_layer = &chunk.background_layer;

        for (y, row) in background_layer.iter().enumerate() {
            for (x, tile_type_id) in row.iter().enumerate() {
                spawn_tile(
                    &mut commands,
                    &tilemap,
                    *tile_type_id,
                    x as i32 + x_offset * 8,
                    y as i32 + y_offset * 8 * -1,
                    0,
                );
            }
        }
        for (y, row) in chunk.foreground_layer.iter().enumerate() {
            for (x, tile_type_id) in row.iter().enumerate() {
                spawn_tile(
                    &mut commands,
                    &tilemap,
                    *tile_type_id,
                    x as i32 + x_offset * 8,
                    y as i32 + y_offset * 8 * -1,
                    10,
                );
            }
        }

        // // load resource data
        // for resource in chunk.resources {
        //     let x = resource.local_x + x_offset * 8;
        //     let y = resource.local_y + y_offset * 8 * -1;
        //     let item_id = resource.item_id;
        //     let uid = resource.uid;
        //     resource::spawn_resource(&mut commands, &item_sheet, item_id, x, y, &uid);
        // }
    }
    resource::spawn_resources(
        &mut commands,
        &item_sheet,
        &mut meshes,
        &mut materials,
        &map,
    );
}

pub enum TileType {
    Air,
    Grass,
    Water,
    Flower,
}
trait TileTypeTrait {
    fn to_type_id(&self) -> u32;
    fn from_type_id(type_id: u32) -> TileType;
    fn to_sprite_index(&self) -> usize;
}
impl TileTypeTrait for TileType {
    fn from_type_id(type_id: u32) -> TileType {
        match type_id {
            0 => TileType::Air,
            1 => TileType::Grass,
            2 => TileType::Water,
            50 => TileType::Flower,
            _ => TileType::Grass,
        }
    }
    fn to_type_id(&self) -> u32 {
        match self {
            TileType::Air => 0,
            TileType::Grass => 1,
            TileType::Water => 2,
            TileType::Flower => 50,
        }
    }
    fn to_sprite_index(&self) -> usize {
        match self {
            TileType::Air => 0,
            TileType::Grass => 23,
            TileType::Water => 27,
            TileType::Flower => 111,
        }
    }
}
fn spawn_tile(
    commands: &mut Commands,
    tilemap: &TileMapSheet,
    tile_type_id: u32,
    x: i32,
    y: i32,
    layer: i32,
) {
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
            translation: Vec3::new((x as f32) * 4. * 16., (y as f32) * 4. * 16., layer as f32),
            scale: Vec3::new(4.0, 4.0, 1.0),
            ..Default::default()
        },
        ..Default::default()
    });
}
