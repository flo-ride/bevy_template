use bevy::prelude::*;
mod core;
mod features;
mod ui;

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
                        title: "Template".into(),
                        ..default()
                    }),
                    ..default()
                }),
        )
        .add_systems(Startup, setup_camera)
        .add_plugins((ui::MenuPlugin, features::hud::HudPlugin))
        .run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2d::default());
}
