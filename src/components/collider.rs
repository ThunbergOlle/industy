use bevy::prelude::Component;

#[derive(Component)]
pub struct Collider {
    pub width: f32,
    pub height: f32,
}
