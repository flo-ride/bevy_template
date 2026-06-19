use bevy::prelude::*;

#[derive(Resource, Clone)]
pub struct UiFont(pub Handle<Font>);

impl UiFont {
    pub fn text(&self, size: f32) -> TextFont {
        TextFont {
            font: self.0.clone(),
            font_size: size,
            ..default()
        }
    }
}

impl FromWorld for UiFont {
    fn from_world(world: &mut World) -> Self {
        let asset_server = world.resource::<AssetServer>();
        Self(asset_server.load("fonts/NotoSans-Regular.ttf"))
    }
}
