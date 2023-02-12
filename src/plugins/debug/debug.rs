use bevy::prelude::{App, Plugin};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
#[path = "../../components/player.rs"]
mod player;
#[path = "../../components/velocity.rs"]
mod velocity;
pub struct DebugPlugin;

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        if cfg!(debug_assertions) {
            app.add_plugin(WorldInspectorPlugin)
                .register_type::<player::Player>()
                .register_type::<velocity::Velocity>();
        }
    }
}
