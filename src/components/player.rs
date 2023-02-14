use bevy::{
    prelude::Component,
    reflect::{FromReflect, Reflect},
};

use crate::item_plugin::Item;
#[derive(Component, Debug, Clone, Reflect, FromReflect)]
pub struct Player {
    pub inventory: Vec<InventoryItem>,
}

#[derive(Debug, Clone, Reflect, FromReflect)]
pub struct InventoryItem {
    pub item: Item,
    pub quantity: u32,
}
