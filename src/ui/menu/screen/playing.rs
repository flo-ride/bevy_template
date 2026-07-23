use bevy::prelude::*;

use crate::core::components::PlayingHud;
use crate::ui::UiFont;

pub fn spawn_playing_hud(mut commands: Commands, _ui_font: Res<UiFont>) {
    commands.spawn((
        PlayingHud,
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            position_type: PositionType::Absolute,
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        },
    ));
}

pub fn despawn_playing_hud(mut commands: Commands, query: Query<Entity, With<PlayingHud>>) {
    for entity in &query {
        commands.entity(entity).despawn();
    }
}
