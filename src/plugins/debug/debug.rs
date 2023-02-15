use bevy::{
    diagnostic::{Diagnostics, FrameTimeDiagnosticsPlugin},
    prelude::{App, AssetServer, Color, Commands, Plugin, Query, Res, TextBundle, With},
    text::{Text, TextSection, TextStyle},
};
use bevy_inspector_egui::quick::WorldInspectorPlugin;

use crate::FpsText;
#[path = "../../components/player.rs"]
mod player;
#[path = "../../components/velocity.rs"]
mod velocity;
pub struct DebugPlugin;

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        if cfg!(debug_assertions) {
            app.add_plugin(WorldInspectorPlugin)
                .add_plugin(FrameTimeDiagnosticsPlugin::default())
                .add_startup_system(frame_rate_text)
                .add_system(text_update_system)
                .register_type::<player::Player>()
                .register_type::<velocity::Velocity>();
        }
    }
}
fn frame_rate_text(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Show FPS
    commands.spawn((
        TextBundle::from_sections([
            TextSection::new(
                "FPS: ",
                TextStyle {
                    font: asset_server.load("fonts/FiraMono-Regular.ttf"),
                    font_size: 20.0,
                    color: Color::WHITE,
                },
            ),
            TextSection::from_style(TextStyle {
                font: asset_server.load("fonts/FiraMono-Medium.ttf"),
                font_size: 20.0,
                color: Color::GOLD,
            }),
        ]),
        FpsText,
    ));
}
fn text_update_system(diagnostics: Res<Diagnostics>, mut query: Query<&mut Text, With<FpsText>>) {
    for mut text in &mut query {
        if let Some(fps) = diagnostics.get(FrameTimeDiagnosticsPlugin::FPS) {
            if let Some(value) = fps.smoothed() {
                // Update the value of the second section
                text.sections[1].value = format!("{value:.0}");
            }
        }
    }
}
