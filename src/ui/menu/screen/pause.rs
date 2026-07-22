use bevy::prelude::*;

use crate::core::components::PauseOverlay;
use crate::ui::UiFont;

pub fn spawn_pause_overlay(mut commands: Commands, ui_font: Res<UiFont>) {
    commands
        .spawn((
            PauseOverlay,
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                position_type: PositionType::Absolute,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                flex_direction: FlexDirection::Column,
                ..default()
            },
            BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.5)),
        ))
        .with_children(|parent| {
            parent.spawn((
                Text::new("PAUSE"),
                ui_font.text(60.0),
                TextColor(Color::WHITE),
            ));
            parent.spawn((
                Text::new("Appuyer sur Échap pour relancer"),
                ui_font.text(20.0),
                TextColor(Color::srgb(0.8, 0.8, 0.8)),
            ));
        });
}

pub fn despawn_pause_overlay(mut commands: Commands, query: Query<Entity, With<PauseOverlay>>) {
    for entity in &query {
        commands.entity(entity).despawn();
    }
}
