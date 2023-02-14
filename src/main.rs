use bevy::{
    diagnostic::{Diagnostics, FrameTimeDiagnosticsPlugin},
    prelude::*,
};

#[path = "./components/collider.rs"]
mod collider;
#[path = "./plugins/debug/debug.rs"]
mod debug_plugin;
#[path = "./plugins/player/player.rs"]
mod player;
#[path = "./components/position.rs"]
mod position;
#[path = "./components/tag.rs"]
mod tag;
#[path = "./plugins/world/world.rs"]
mod world_plugin;
fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_plugin(player::PlayerPlugin)
        .add_plugin(debug_plugin::DebugPlugin)
        .add_plugin(world_plugin::WorldPlugin)
        .add_plugin(FrameTimeDiagnosticsPlugin::default()) // FPS
        .add_startup_system(setup_world)
        .add_system(text_update_system) // FPS
        .run();
}

#[derive(Component)]
struct FpsText;

fn setup_world(mut commands: Commands, asset_server: Res<AssetServer>) {
    println!("Setting up a game world!");

    commands.spawn(Camera2dBundle::default());
    // commands
    //     .spawn(SpriteBundle {
    //         texture: asset_server.load("map.png").into(),
    //         transform: Transform {
    //             translation: Vec3::new(0.0, 0.0, 0.0),
    //             scale: Vec3::new(4.0, 4.0, 1.0),
    //             ..Default::default()
    //         },
    //         ..Default::default()
    //     })
    //     .insert(tag::Tag("map".to_string()))
    //     .insert(position::Position { x: 0, y: 0 });

    
    // Show FPS
    commands.spawn((
        TextBundle::from_sections([
            TextSection::new(
                "FPS: ",
                TextStyle {
                    font: asset_server.load("fonts/FiraMono-Regular.ttf"),
                    font_size: 60.0,
                    color: Color::WHITE,
                },
            ),
            TextSection::from_style(TextStyle {
                font: asset_server.load("fonts/FiraMono-Medium.ttf"),
                font_size: 60.0,
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
                text.sections[1].value = format!("{value:.2}");
            }
        }
    }
}