use bevy::prelude::*;

#[derive(Resource, Default)]
pub struct Score(pub u32);

#[derive(Resource, Default)]
pub struct Health(pub u32);
