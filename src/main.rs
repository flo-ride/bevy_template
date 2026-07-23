use bevy::prelude::*;
mod alchemy;
mod core;
mod environment;
mod features;
mod interaction;
mod physics;
mod ui;
mod views;

use features::personna::plugin::PersonnaPlugin;

pub fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(AssetPlugin {
                    file_path: "assets".into(),
                    ..default()
                })
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        canvas: Some("#bevy-game-canvas".into()),
                        title: "Template".into(),
                        name: Some("template.app".into()),
                        // Tells wasm not to override default event handling, like F5, Ctrl+R etc.
                        prevent_default_event_handling: false,
                        window_theme: Some(bevy::window::WindowTheme::Dark),
                        ..default()
                    }),
                    ..default()
                }),
        )
        .add_plugins(core::states::StatePlugin)
        .add_plugins(physics::PhysicsPlugin)
        .add_plugins(interaction::InteractionPlugin)
        .add_plugins(alchemy::AlchemyPlugin)
        .add_systems(Startup, setup_camera)
        .add_plugins(PersonnaPlugin)
        .add_plugins(ui::MenuPlugin)
        .add_plugins(views::ViewsPlugin)
        .add_plugins(environment::EnvironmentPlugin)
        .run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn((Camera2d, core::components::MainCamera));
}
