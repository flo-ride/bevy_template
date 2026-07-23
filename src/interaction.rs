use crate::core::components::MainCamera;
use avian2d::prelude::*;
use bevy::prelude::*;

pub struct InteractionPlugin;

impl Plugin for InteractionPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<CursorWorldPos>().add_systems(
            Update,
            (
                update_cursor_pos,
                handle_grab_and_release,
                update_held_position,
            ),
        );
    }
}

#[derive(Resource, Default)]
pub struct CursorWorldPos(pub Vec2);

#[derive(Component)]
pub struct Draggable;

#[derive(Component)]
pub struct Held;

fn update_cursor_pos(
    mut cursor_pos: ResMut<CursorWorldPos>,
    window_query: Query<&Window>,
    camera_query: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
) {
    let Some(window) = window_query.iter().next() else {
        return;
    };
    let Some((camera, camera_transform)) = camera_query.iter().next() else {
        return;
    };

    if let Some(pos) = window.cursor_position()
        && let Ok(world_pos) = camera.viewport_to_world_2d(camera_transform, pos)
    {
        cursor_pos.0 = world_pos;
    }
}

fn handle_grab_and_release(
    mut commands: Commands,
    buttons: Res<ButtonInput<MouseButton>>,
    cursor_pos: Res<CursorWorldPos>,
    spatial_query: SpatialQuery,
    held_query: Query<Entity, With<Held>>,
    draggable_query: Query<Entity, With<Draggable>>,
) {
    if buttons.just_pressed(MouseButton::Left) {
        // If we are already holding something, release it
        if let Some(held_entity) = held_query.iter().next() {
            commands
                .entity(held_entity)
                .remove::<Held>()
                .insert(RigidBody::Dynamic); // Make it fall again
        } else {
            // Otherwise, try to grab something under the cursor
            let filter = SpatialQueryFilter::default();
            let intersections = spatial_query.point_intersections(cursor_pos.0, &filter);

            for entity in intersections {
                if draggable_query.contains(entity) {
                    commands
                        .entity(entity)
                        .insert(Held)
                        .insert(RigidBody::Kinematic) // Stop physics from making it fall
                        .insert(LinearVelocity::ZERO)
                        .insert(AngularVelocity::ZERO);
                    break; // Only grab one item
                }
            }
        }
    }
}

fn update_held_position(
    cursor_pos: Res<CursorWorldPos>,
    mut held_query: Query<&mut Transform, With<Held>>,
) {
    for mut transform in held_query.iter_mut() {
        // Move the held object to the cursor position
        transform.translation.x = cursor_pos.0.x;
        transform.translation.y = cursor_pos.0.y;
    }
}
