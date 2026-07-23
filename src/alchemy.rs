use crate::interaction::{CursorWorldPos, Held};
use avian2d::prelude::*;
use bevy::prelude::*;

pub struct AlchemyPlugin;

impl Plugin for AlchemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (handle_pouring, update_liquid_visuals));
    }
}

#[derive(Component)]
pub struct LiquidContainer {
    pub doses: usize,
    pub max_doses: usize,
    pub color: Color,
    pub is_glass: bool, // Just to distinguish behavior if needed
}

#[derive(Component)]
pub struct LiquidVisual {
    pub container_height: f32,
    pub max_width: f32,
}

fn handle_pouring(
    buttons: Res<ButtonInput<MouseButton>>,
    cursor_pos: Res<CursorWorldPos>,
    spatial_query: SpatialQuery,
    mut container_query: Query<(Entity, &mut LiquidContainer)>,
    held_query: Query<Entity, With<Held>>,
) {
    if buttons.just_pressed(MouseButton::Right) {
        // Find the held entity
        let Some(held_entity) = held_query.iter().next() else {
            return;
        };

        // Find what we clicked on
        let filter = SpatialQueryFilter::default();
        let intersections = spatial_query.point_intersections(cursor_pos.0, &filter);

        let mut target_entity = None;
        for entity in intersections {
            if entity != held_entity && container_query.contains(entity) {
                target_entity = Some(entity);
                break;
            }
        }

        // Proceed if we have a valid target
        if let Some(target) = target_entity {
            // We need to borrow both the held container and the target container
            let combinations = container_query.get_many_mut([held_entity, target]);
            if let Ok([mut held_container, mut target_container]) = combinations {
                // Check if we can pour
                if held_container.1.doses > 0
                    && target_container.1.doses < target_container.1.max_doses
                {
                    // Transfer 1 dose
                    held_container.1.doses -= 1;
                    target_container.1.doses += 1;

                    // Simple color mix (average)
                    let c1 = held_container.1.color.to_srgba();
                    let c2 = target_container.1.color.to_srgba();

                    // If target was empty, it just takes the color
                    if target_container.1.doses == 1 {
                        target_container.1.color = held_container.1.color;
                    } else {
                        // Mix
                        target_container.1.color = Color::srgb(
                            (c1.red + c2.red) / 2.0,
                            (c1.green + c2.green) / 2.0,
                            (c1.blue + c2.blue) / 2.0,
                        );
                    }
                }
            }
        }
    }
}

fn update_liquid_visuals(
    container_query: Query<(&LiquidContainer, &Children), Changed<LiquidContainer>>,
    mut visual_query: Query<(
        &mut Transform,
        &mut MeshMaterial2d<ColorMaterial>,
        &LiquidVisual,
    )>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    for (container, children) in container_query.iter() {
        for child in children.iter() {
            if let Ok((mut transform, mut material_handle, visual)) = visual_query.get_mut(child) {
                let fill_ratio = container.doses as f32 / container.max_doses as f32;

                // Scale Y
                transform.scale.y = fill_ratio.max(0.001); // Avoid exactly 0 scale to prevent issues

                // Adjust Y position so the liquid stays at the bottom
                // The pivot is at the center of the mesh.
                let base_y = -visual.container_height / 2.0;
                let current_height = visual.container_height * fill_ratio;
                transform.translation.y = base_y + (current_height / 2.0);

                // Update color
                material_handle.0 = materials.add(container.color);
            }
        }
    }
}
