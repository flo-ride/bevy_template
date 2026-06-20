use bevy::prelude::*;
use bevy::reflect::TypePath;
use serde::Deserialize;
use std::collections::HashMap;

#[derive(Asset, TypePath, Deserialize, Debug)]
pub struct Translation {
    pub translations: HashMap<String, String>,
}

#[derive(Resource, Default)]
pub struct Locale {
    pub current: String,
    pub handles: HashMap<String, Handle<Translation>>,
    pub active_handle: Option<Handle<Translation>>,
}

impl Locale {
    pub fn get_translation<'a>(
        &self,
        translations: &'a Assets<Translation>,
        key: &'a str,
    ) -> &'a str {
        let lookup = |handle: &Handle<Translation>| -> Option<&str> {
            translations
                .get(handle)
                .and_then(|t| t.translations.get(key))
                .map(|s| s.as_str())
        };

        if let Some(active) = &self.active_handle {
            if let Some(text) = lookup(active) {
                return text;
            }
        }

        // fallback anglais
        if let Some(en_handle) = self.handles.get("en") {
            if let Some(text) = lookup(en_handle) {
                return text;
            }
        }

        key
    }
}
