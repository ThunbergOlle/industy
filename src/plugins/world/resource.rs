use bevy::{
    prelude::{
        default, shape, Assets, BuildChildren, Color, Commands, Mesh, Res, ResMut, SpatialBundle,
        Transform, Vec3,
    },
    sprite::{ColorMaterial, MaterialMesh2dBundle, SpriteSheetBundle, TextureAtlasSprite},
};
use bevy_mod_raycast::RaycastMesh;

use crate::item_resource::ItemSheet;
use crate::uid::UID;
use crate::{item_plugin::Item, raycast_plugin};

#[path = "../item/item.rs"]
mod item_plugin;

pub fn spawn_resources(
    commands: &mut Commands,
    item_resource: &Res<ItemSheet>,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
    tilemap: &super::TileMapJSON,
) {
    // loop over every chunk in the world
    for chunk in &tilemap.world {
        let x_offset = chunk.x;
        let y_offset = chunk.y;
        // loop over every resource in the chunk
        for resource in &chunk.resources {
            // spawn the resource
            let x = resource.local_x + x_offset * 8;
            let y = resource.local_y + y_offset * 8 * -1;
            println!("Spawning resource at x: {}, y: {}", x, y);
            spawn_resource(
                commands,
                &item_resource,
                meshes,
                materials,
                resource.item_id,
                x,
                y,
                &resource.uid,
            );
        }
    }
}
pub fn spawn_resource(
    commands: &mut Commands,
    item_sheet: &Res<ItemSheet>,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
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
            let mesh_bundle = commands
                .spawn(MaterialMesh2dBundle {
                    // why does this have to be a mesh?
                    mesh: meshes.add(Mesh::from(shape::Quad::default())).into(),
                    transform: Transform {
                        scale: Vec3::new(16.0, 16.0, 1.0),
                        ..Default::default()
                    },
                    material: materials.add(ColorMaterial::from(Color::PURPLE)),
                    ..default()
                })
                .insert(RaycastMesh::<raycast_plugin::ClickableRayCast>::default())
                .id();
            let sprite_sheet_bundle = commands
                .spawn(SpriteSheetBundle {
                    texture_atlas: item_sheet.0.clone(),
                    sprite,
                    transform: Transform {
                        ..Default::default()
                    },
                    ..Default::default()
                })
                .insert(UID(uid.to_string()))
                .id();
            let mut bundle = commands.spawn(SpatialBundle {
                transform: Transform {
                    translation: Vec3::new(x as f32, y as f32, 50.),
                    scale: Vec3::new(4.0, 4.0, 1.0),
                    ..Default::default()
                },
                visibility: Default::default(),
                computed: Default::default(),
                global_transform: Default::default(),
            });
            bundle.push_children(&[mesh_bundle, sprite_sheet_bundle]);
        }
        Err(e) => {
            println!("Could not spawn resource error: {}", e);
        }
    }
}
