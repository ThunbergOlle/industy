use bevy::{prelude::Component, reflect::Reflect};

#[derive(Component, Reflect)]
struct Item(String);
