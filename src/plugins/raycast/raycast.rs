use bevy::{prelude::*, window::CursorMoved};
use bevy_mod_raycast::{
    DefaultRaycastingPlugin, Intersection, RaycastMethod, RaycastSource, RaycastSystem,
};

pub struct ClickableRayCast; // if this strucet is added to an entity. it means that it will be clickable.

pub struct RayCastPlugin;

impl Plugin for RayCastPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(DefaultRaycastingPlugin::<ClickableRayCast>::default())
            .add_system_to_stage(
                CoreStage::First,
                update_raycast_with_cursor.before(RaycastSystem::BuildRays::<ClickableRayCast>),
            )
            .add_system(intersection);
    }
}
// Update our `RaycastSource` with the current cursor position every frame.
fn update_raycast_with_cursor(
    mut cursor: EventReader<CursorMoved>,
    mut query: Query<&mut RaycastSource<ClickableRayCast>>,
) {
    // Grab the most recent cursor event if it exists:
    let cursor_position = match cursor.iter().last() {
        Some(cursor_moved) => cursor_moved.position,
        None => return,
    };
    for mut pick_source in &mut query {
        pick_source.cast_method = RaycastMethod::Screenspace(cursor_position);
    }
}
// Report intersections
fn intersection(query: Query<(&Intersection<ClickableRayCast>)>) {
    for (intersection) in &query {
        // get the entity that was clicked on

        println!("Entity {:?} was clicked on with a raycast!", intersection);
    }
}
