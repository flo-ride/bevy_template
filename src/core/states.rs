use bevy::prelude::*;

#[allow(dead_code)]
#[derive(States, Default, Debug, Clone, PartialEq, Eq, Hash)]
pub enum GameState {
    #[default]
    Loading,
    StartMenu,
    Playing,
    Paused,
    GameOver,
    Victory,
}
