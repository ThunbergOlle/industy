use bevy::{
    prelude::{Commands, Res, Transform, Vec3},
    sprite::{SpriteSheetBundle, TextureAtlasSprite},
};

use crate::item_plugin::Item;
use crate::item_resource::ItemSheet;

#[path = "../item/item.rs"]
mod item_plugin;

pub fn spawn_resources(
    commands: &mut Commands,
    item_resource: Res<ItemSheet>,
    tilemap: &super::TileMapJSON,
    x_offset: i32,
    y_offset: i32,
) {
    // loop over every chunk in the world
    for chunk in &tilemap.world {
        // loop over every resource in the chunk
        for resource in &chunk.resources {
            // spawn the resource
            spawn_resource(
                commands,
                &item_resource,
                resource.item_id,
                resource.local_x + x_offset,
                resource.local_y + y_offset,
                &resource.uid,
            );
        }
    }
}
pub fn spawn_resource(
    commands: &mut Commands,
    item_sheet: &Res<ItemSheet>,
    item_id: u32,
    local_x: i32,
    local_y: i32,
    uid: &str,
) {
    let item = Item::from_type_id(item_id);

    // if the result is error then return using match
    match item {
        Ok(item) => {
            let sprite_index = item.to_sprite_index();
            let sprite = TextureAtlasSprite {
                index: sprite_index,
                ..Default::default()
            };
            let x = local_x * 16;
            let y = local_y * 16 * -1;
            commands.spawn(SpriteSheetBundle {
                texture_atlas: item_sheet.0.clone(),
                sprite,
                transform: Transform {
                    translation: Vec3::new((x as f32), (y as f32), 50.),
                    scale: Vec3::new(4.0, 4.0, 1.0),
                    ..Default::default()
                },
                ..Default::default()
            });
        }
        Err(e) => {
            println!("Could not spawn resource error: {}", e);
        }
    }
}
