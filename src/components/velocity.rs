use bevy::{prelude::Component, reflect::Reflect};
#[derive(Component, Clone, Copy, PartialEq, Debug, Reflect)]
// velocity
pub struct Velocity {
    pub x: f32,
    pub y: f32,
}
