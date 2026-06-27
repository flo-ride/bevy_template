use bevy::prelude::*;

pub mod components;
pub mod resources;
pub mod systems;

pub struct HudPlugin;

impl Plugin for HudPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<resources::Score>()
            .init_resource::<resources::Health>()
            .add_systems(Startup, setup_hud)
            .add_systems(
                Update,
                (systems::update_score_hud, systems::update_health_hud),
            );
    }
}

fn setup_hud(mut commands: Commands) {
    commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                justify_content: JustifyContent::SpaceBetween,
                align_items: AlignItems::FlexStart,
                ..default()
            },
            components::HUDElement,
        ))
        .with_children(|parent| {
            parent.spawn((
                Text::new("Score: 0"),
                TextFont::from_font_size(30.0),
                TextColor(Color::WHITE),
                components::ScoreLabel,
            ));
            parent.spawn((
                Text::new("Health: 100"),
                TextFont::from_font_size(30.0),
                TextColor(Color::WHITE),
                components::HealthLabel,
            ));
        });
}
