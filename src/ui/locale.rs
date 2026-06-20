use crate::core::ressources::{Locale, Translation};
use bevy::prelude::*;
use bevy_common_assets::json::JsonAssetPlugin;

pub struct LocalePlugin;

impl Plugin for LocalePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(JsonAssetPlugin::<Translation>::new(&["json"]))
            .init_resource::<Locale>()
            .add_systems(Startup, load_locales);
    }
}

fn load_locales(asset_server: Res<AssetServer>, mut locale: ResMut<Locale>) {
    locale
        .handles
        .insert("en".to_string(), asset_server.load("locales/en.json"));
    locale
        .handles
        .insert("fr".to_string(), asset_server.load("locales/fr.json"));
    locale.active_handle = Some(locale.handles.get("en").cloned().unwrap());
}
