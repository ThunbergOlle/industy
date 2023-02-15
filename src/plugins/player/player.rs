use crate::player_component::{self};

use bevy::{
    prelude::{
        default, App, AssetServer, Assets, Camera2d, Commands, Component, Handle, Input,
        IntoSystemDescriptor, KeyCode, Name, Plugin, Query, Res, ResMut, Resource, StartupStage,
        Transform, Vec2, Vec3, With, Without,
    },
    sprite::{SpriteSheetBundle, TextureAtlas, TextureAtlasSprite},
    time::{Time, Timer, TimerMode},
};
#[path = "../../components/collider.rs"]
mod collider;
#[path = "../../components/playable_character.rs"]
mod playable_character;

#[path = "../../components/tag.rs"]
mod tag;
#[path = "../../components/velocity.rs"]
mod velocity;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system_to_stage(StartupStage::PreStartup, setup_player)
            .add_system(player_movement.label("player_movement"))
            .add_system(player_velocity_animation.after("player_movement"))
            .add_system(
                follow_player
                    .after("player_movement")
                    .label("follow_player"),
            );
    }
}
#[derive(Resource, Default, Debug)]
struct PlayerCharacterSheet(Handle<TextureAtlas>);

fn setup_player(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let texture_handle = asset_server.load("character/blue.png");
    let texture_atlas =
        TextureAtlas::from_grid(texture_handle, Vec2::new(24.0, 24.0), 24, 2, None, None);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);
    commands.insert_resource(PlayerCharacterSheet(texture_atlas_handle.clone()));

    println!("Setting up a player!");

    commands
        .spawn(SpriteSheetBundle {
            texture_atlas: texture_atlas_handle,
            transform: Transform {
                scale: Vec3::new(4.0, 4.0, 1.0),
                translation: Vec3::new(0.0, 0.0, 100.0), // 100.0 is the z-index of the player. This makes sure the player is always on top of the map.
                ..Default::default()
            },
            ..default()
        })
        .insert(AnimationComponent {
            timer: Timer::from_seconds(0.1, TimerMode::Repeating),
            current_frame: 0,
            state: AnimationState::Idle,
            animation: Animation {
                frames: vec![],
                frame_duration: 1.,
            },
        })
        .insert(collider::Collider {
            width: 24.,
            height: 24.,
        })
        .insert(player_component::Player { inventory: vec![] })
        .insert(velocity::Velocity { x: 0., y: 0. })
        .insert(playable_character::PlayableCharacter {})
        .insert(tag::Tag("player".to_string()))
        .insert(Name::new("Player"));
}

fn player_movement(
    keyboard_input: Res<Input<KeyCode>>,
    mut positions: Query<(
        &playable_character::PlayableCharacter,
        &mut Transform,
        &mut velocity::Velocity,
    )>,
) {
    for (_head, mut transform, mut velocity) in positions.iter_mut() {
        let mut new_velocity = velocity::Velocity { x: 0., y: 0. };

        // convert pressed input to integers
        let left: i32 = keyboard_input.pressed(KeyCode::Left).into();
        let right: i32 = keyboard_input.pressed(KeyCode::Right).into();

        new_velocity.x = (right - left) as f32; // 0 stationary 1 right -1 left

        // do the same for y axis
        let up: i32 = keyboard_input.pressed(KeyCode::Up).into();
        let down: i32 = keyboard_input.pressed(KeyCode::Down).into();

        new_velocity.y = (up - down) as f32; // 0 stationary 1 up -1 down

        transform.translation.x += 2. * new_velocity.x;
        transform.translation.y += 2. * new_velocity.y;
        *velocity = new_velocity;
    }
}
#[derive(Component)]
struct AnimationComponent {
    pub timer: Timer,
    pub state: AnimationState,
    pub animation: Animation,
    pub current_frame: usize,
}
#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
enum AnimationState {
    Idle,
    WalkRight,
    WalkLeft,
}

#[derive(PartialEq, Debug, Clone)]
struct Animation {
    pub frames: Vec<usize>,
    pub frame_duration: f32,
}

fn player_velocity_animation(
    time: Res<Time>,
    mut query: Query<(
        &mut AnimationComponent,
        &mut TextureAtlasSprite,
        &velocity::Velocity,
        With<player_component::Player>,
    )>,
) {
    let idle_animation = Animation {
        frames: vec![0, 1, 2, 3],
        frame_duration: 2.,
    };
    let walk_right_animation = Animation {
        frames: vec![4, 5, 6, 7, 8, 9],
        frame_duration: 1.,
    };
    let walk_left_animation = Animation {
        frames: vec![44, 43, 42, 41, 40, 39],
        frame_duration: 1.,
    };
    for (mut animation_component, mut sprite, velocity, _) in &mut query {
        // TODO: Better system for this
        let prev_state = animation_component.state;
        let new_state = get_new_state(velocity);

        let update_animation = prev_state != new_state;
        if new_state != prev_state {
            /* Reset animation states */
            animation_component.state = new_state;
            animation_component.current_frame = 0;
            animation_component.timer.reset();
        } else {
            let dur = time
                .delta()
                .div_f32(animation_component.animation.frame_duration);

            animation_component.timer.tick(dur);
        }

        if !update_animation && !animation_component.timer.just_finished() {
            return;
        }
        if velocity.x == 0. && velocity.y == 0. {
            animation_component.animation = idle_animation.clone();
            perform_animation(&mut sprite, &mut animation_component);
            return;
        } else if velocity.x == 1. {
            animation_component.animation = walk_right_animation.clone();
            perform_animation(&mut sprite, &mut animation_component);
        } else if velocity.x == -1. {
            animation_component.animation = walk_left_animation.clone();
            perform_animation(&mut sprite, &mut animation_component);
        }
    }
}

fn perform_animation(
    sprite: &mut TextureAtlasSprite,
    animation_component: &mut AnimationComponent,
) {
    animation_component.current_frame =
        (animation_component.current_frame + 1) % animation_component.animation.frames.len();
    sprite.index = animation_component.animation.frames[animation_component.current_frame];
}
fn get_new_state(velocity: &velocity::Velocity) -> AnimationState {
    if velocity.x == 0. && velocity.y == 0. {
        return AnimationState::Idle;
    }
    if velocity.x == 1. {
        return AnimationState::WalkRight;
    }
    if velocity.x == -1. {
        return AnimationState::WalkLeft;
    }
    AnimationState::Idle
}

fn follow_player(
    mut camera: Query<(
        &Camera2d,
        &mut Transform,
        Without<playable_character::PlayableCharacter>,
    )>,
    player: Query<&Transform, With<playable_character::PlayableCharacter>>,
) {
    for (_camera, mut transform, _) in camera.iter_mut() {
        for player_transform in player.iter() {
            transform.translation.x = player_transform.translation.x;
            transform.translation.y = player_transform.translation.y;
        }
    }
}
