use crate::features::personna::components::PersonnaConfig;
use bevy::prelude::*;

pub fn print_pnj_names(assets: Res<Assets<PersonnaConfig>>) {
    for (handle, config) in assets.iter() {
        info!("--- Personas Loaded in Asset {:?} ---", handle);
        for name in config.personas.keys() {
            info!("Persona: {}", name);
        }
    }
}
