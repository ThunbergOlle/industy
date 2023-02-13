use bevy::{
    prelude::{Handle, Resource},
    reflect::Reflect,
    sprite::TextureAtlas,
};

#[derive(Resource, Default, Debug, Clone, PartialEq, Eq, Reflect)]
pub struct ItemSheet(pub Handle<TextureAtlas>);
