use bevy::{ecs::query::WorldQuery, prelude::Component, reflect::Reflect};
#[derive(Component, Clone, PartialEq, Eq, Reflect, Debug)]
pub struct Resource {
    pub local_x: i32,
    pub local_y: i32,
    pub resource_type: String,
    pub uid: String,
}
