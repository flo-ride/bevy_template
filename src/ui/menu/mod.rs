mod screen;

use crate::core::states::GameState;
use crate::ui::menu::screen::pause::{despawn_pause_overlay, spawn_pause_overlay};
use crate::ui::menu::screen::splash::{
    animate_loading_screen, despawn_loading_screen, loading_to_playing, spawn_loading_screen,
};
use bevy::prelude::*;
pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<GameState>()
            .add_systems(Update, change_state_from_input)
            .add_systems(OnEnter(GameState::Pause), spawn_pause_overlay)
            .add_systems(OnExit(GameState::Pause), despawn_pause_overlay)
            .add_systems(OnEnter(GameState::LoadingGame), spawn_loading_screen)
            .add_systems(
                Update,
                animate_loading_screen.run_if(in_state(GameState::LoadingGame)),
            )
            .add_systems(
                Update,
                loading_to_playing.run_if(in_state(GameState::LoadingGame)),
            )
            .add_systems(OnExit(GameState::LoadingGame), despawn_loading_screen);
    }
}

fn change_state_from_input(
    input: Res<ButtonInput<KeyCode>>,
    mut game_next_state: ResMut<NextState<GameState>>,
    game_state: Res<State<GameState>>,
) {
    if input.just_pressed(KeyCode::Escape) {
        match game_state.get() {
            GameState::Playing => game_next_state.set(GameState::Pause),
            _ => game_next_state.set(GameState::Playing),
        }
    };
}
