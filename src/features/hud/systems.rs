use super::components::*;
use super::resources::*;
use bevy::prelude::*;

pub fn update_score_hud(score: Res<Score>, mut query: Query<&mut Text, With<ScoreLabel>>) {
    if score.is_changed() {
        for mut text in &mut query {
            *text = Text::new(format!("Score: {}", score.0));
        }
    }
}

pub fn update_health_hud(health: Res<Health>, mut query: Query<&mut Text, With<HealthLabel>>) {
    if health.is_changed() {
        for mut text in &mut query {
            *text = Text::new(format!("Health: {}", health.0));
        }
    }
}
