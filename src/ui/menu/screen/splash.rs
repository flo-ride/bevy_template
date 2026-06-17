use bevy::prelude::*;

use crate::core::components::{LoadingScreen, LoadingText};
use crate::core::states::GameState;

pub fn spawn_loading_screen(mut commands: Commands) {
    commands
        .spawn((
            LoadingScreen,
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                justify_content: JustifyContent::End,
                align_items: AlignItems::Start,
                flex_direction: FlexDirection::Column,
                padding: UiRect {
                    left: Val::Percent(2.),
                    right: Val::Percent(2.),
                    top: Val::Percent(2.),
                    bottom: Val::Percent(2.),
                },
                ..default()
            },
            BackgroundColor(Color::BLACK),
        ))
        .with_children(|parent| {
            parent.spawn((
                Text::new("Loading"),
                TextFont {
                    font_size: 50.0,
                    ..default()
                },
                TextColor(Color::WHITE),
                LoadingText,
            ));
        });
}

pub fn animate_loading_screen(time: Res<Time>, mut query: Query<&mut Text, With<LoadingText>>) {
    let t = time.elapsed_secs();

    let phase = (t * 2.0) as u32 % 6;

    let dots = match phase {
        0 => "",
        1 => ".",
        2 => "..",
        3 => "...",
        4 => "....",
        _ => ".....",
    };

    for mut text in &mut query {
        text.0 = format!("Loading{}", dots);
    }
}

pub fn loading_to_playing(time: Res<Time>, mut next_state: ResMut<NextState<GameState>>) {
    if time.elapsed_secs() > 5.0 {
        next_state.set(GameState::Playing);
    }
}

pub fn despawn_loading_screen(mut commands: Commands, query: Query<Entity, With<LoadingScreen>>) {
    for entity in &query {
        commands.entity(entity).despawn();
    }
}
