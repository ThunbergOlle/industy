use std::fmt::Error;

use bevy::{
    prelude::{App, Plugin},
    reflect::{FromReflect, Reflect},
};

#[path = "../../resources/item_sheet.rs"]
mod item_resource;

pub struct ItemPlugin;

impl Plugin for ItemPlugin {
    fn build(&self, _: &mut App) {}
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Reflect, FromReflect)]
pub enum Item {
    Stone,
    Wood,
    Sand,
    Gold,
    Lithium,
    Oil,
    NaturalGas,
    Plastic,
}

#[allow(dead_code)]
impl Item {
    pub fn to_type_id(&self) -> u32 {
        match self {
            Item::Stone => 0,
            Item::Wood => 1,
            Item::Sand => 2,
            Item::Gold => 3,
            Item::Lithium => 4,
            Item::Oil => 5,
            Item::NaturalGas => 6,
            Item::Plastic => 7,
        }
    }
    pub fn from_type_id(id: u32) -> Result<Item, Error> {
        match id {
            0 => Ok(Item::Stone),
            1 => Ok(Item::Wood),
            2 => Ok(Item::Sand),
            3 => Ok(Item::Gold),
            4 => Ok(Item::Lithium),
            5 => Ok(Item::Oil),
            6 => Ok(Item::NaturalGas),
            7 => Ok(Item::Plastic),
            _ => Err(Error),
        }
    }
    pub fn to_sprite_index(&self) -> usize {
        match self {
            Item::Stone => 0,
            Item::Wood => 1,
            Item::Sand => 2,
            Item::Gold => 3,
            Item::Lithium => 4,
            Item::Oil => 5,
            Item::NaturalGas => 6,
            Item::Plastic => 7,
        }
    }
}
