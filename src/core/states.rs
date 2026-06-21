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

impl GameState {
    pub fn initial() -> Self {
        #[cfg(feature = "dev")]
        {
            GameState::Playing
        }

        #[cfg(not(feature = "dev"))]
        {
            GameState::Loading
        }
    }
}
