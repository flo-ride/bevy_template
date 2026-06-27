use crate::core::states::GameState;
use bevy::prelude::*;

pub mod builder;
pub mod components;
pub mod placement;
pub mod resources;
pub mod systems;

pub struct HudPlugin;

impl Plugin for HudPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<resources::Score>()
            .init_resource::<resources::Health>()
            .add_systems(OnEnter(GameState::Playing), setup_hud)
            .add_systems(OnExit(GameState::Playing), despawn_hud)
            .add_systems(
                Update,
                (systems::update_score_hud, systems::update_health_hud)
                    .run_if(in_state(GameState::Playing)),
            );
    }
}

fn setup_hud(mut commands: Commands) {
    commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                padding: UiRect::all(Val::Px(20.0)),
                display: Display::Grid,
                grid_template_columns: vec![
                    GridTrack::flex(1.0),
                    GridTrack::flex(1.0),
                    GridTrack::flex(1.0),
                ],
                grid_template_rows: vec![
                    GridTrack::flex(1.0),
                    GridTrack::flex(1.0),
                    GridTrack::flex(1.0),
                ],
                ..default()
            },
            components::HUDElement,
        ))
        .with_children(|parent| {
            builder::spawn_hud_item(parent, placement::HudPlacement::TopLeft, |p| {
                p.spawn((
                    Text::new("Score: 0"),
                    TextFont::from_font_size(30.0),
                    TextColor(Color::WHITE),
                    components::ScoreLabel,
                ));
            });
            builder::spawn_hud_item(parent, placement::HudPlacement::TopRight, |p| {
                p.spawn((
                    Text::new("Health: 100"),
                    TextFont::from_font_size(30.0),
                    TextColor(Color::WHITE),
                    components::HealthLabel,
                ));
            });
            builder::spawn_hud_item(parent, placement::HudPlacement::BottomCenter, |p| {
                p.spawn((
                    components::InventoryElement,
                    Node {
                        width: Val::Px(200.0),
                        height: Val::Px(50.0),
                        flex_direction: FlexDirection::Row,
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        column_gap: Val::Px(5.0),
                        padding: UiRect::all(Val::Px(5.0)),
                        ..default()
                    },
                    BackgroundColor(Color::srgba(0.2, 0.2, 0.2, 0.8)),
                ))
                .with_children(|inventory| {
                    for _ in 0..3 {
                        inventory.spawn((
                            Node {
                                width: Val::Px(40.0),
                                height: Val::Px(40.0),
                                ..default()
                            },
                            BackgroundColor(Color::srgba(0.5, 0.5, 0.5, 1.0)),
                        ));
                    }
                });
            });
        });
}

fn despawn_hud(mut commands: Commands, query: Query<Entity, With<components::HUDElement>>) {
    for entity in &query {
        commands.entity(entity).despawn();
    }
}
