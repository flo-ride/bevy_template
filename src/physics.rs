use avian2d::prelude::*;
use bevy::prelude::*;

pub struct PhysicsPlugin;

impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(PhysicsPlugins::default())
            // Disable gravity for now or set it to 2D standard
            .insert_resource(Gravity(Vec2::NEG_Y * 1000.0));
    }
}
