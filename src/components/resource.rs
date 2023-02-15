use bevy::prelude::Component;
#[derive(Component, Clone, Copy, PartialEq, Eq, Reflect)]
pub struct Resource {
    pub local_x: i32,
    pub local_y: i32,
    pub resource_type: String,
    pub uid: String,
}
