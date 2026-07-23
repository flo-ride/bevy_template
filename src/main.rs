use bevy::{prelude::*, window::PresentMode};
mod core;
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
                        present_mode: PresentMode::AutoVsync,
                        // Tells Wasm to resize the window according to the available canvas
                        fit_canvas_to_parent: true,
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
        .add_systems(Startup, setup_camera)
        .add_plugins(ui::MenuPlugin)
        .run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2d);
}
