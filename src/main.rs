use bevy::prelude::*;
mod core;
mod ui;

pub fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(AssetPlugin {
            file_path: "assets".into(),
            ..default()
        }))
        .add_plugins(ui::MenuPlugin)
        .run();
}
