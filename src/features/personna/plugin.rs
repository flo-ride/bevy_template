use crate::features::personna::components::PersonnaConfig;
use bevy::prelude::*;
use bevy_common_assets::ron::RonAssetPlugin;

pub struct PersonnaPlugin;

impl Plugin for PersonnaPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(RonAssetPlugin::<PersonnaConfig>::new(&["personna.ron"]))
            .init_resource::<PersonnaHandle>()
            .add_systems(Update, print_pnj_names);
    }
}

#[derive(Resource)]
struct PersonnaHandle(Handle<PersonnaConfig>);

impl FromWorld for PersonnaHandle {
    fn from_world(world: &mut World) -> Self {
        let asset_server = world.resource::<AssetServer>();
        PersonnaHandle(asset_server.load("personna.ron"))
    }
}

fn print_pnj_names(
    assets: Res<Assets<PersonnaConfig>>,
    handle: Res<PersonnaHandle>,
    asset_server: Res<AssetServer>,
) {
    let handle = &handle.0;

    if let Some(config) = assets.get(handle) {
        info!("--- Données trouvées ---");
        for name in config.personas.keys() {
            info!("Persona: {}", name);
        }
    } else {
        info!("L'asset n'est pas encore chargé dans Assets<>");
    }
}
