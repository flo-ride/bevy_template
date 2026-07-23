use bevy::prelude::*;

use crate::core::components::{AbstractBackground, BackgroundBlob};

// Remplacement des blobs par des rectangles type "particules de poussière" ou "rayons de lumière"
// On change légèrement la structure pour qu'ils ressemblent à des volutes de fumée ou particules de taverne.
const PARTICLE_COUNT: usize = 12;

pub fn spawn_abstract_background(mut commands: Commands) {
    commands
        .spawn((
            AbstractBackground,
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                position_type: PositionType::Absolute,
                overflow: Overflow::clip(),
                ..default()
            },
            BackgroundColor(Color::srgb(0.08, 0.06, 0.04)), // Fond sombre taverne
            ZIndex(-10),
        ))
        .with_children(|parent| {
            for i in 0..PARTICLE_COUNT {
                let size = 3.0 + (i as f32 % 3.0) * 2.0;
                parent.spawn((
                    BackgroundBlob {
                        base_x: (i as f32 * 15.0) % 100.0,
                        base_y: (i as f32 * 10.0) % 100.0,
                        speed: 0.2 + (i as f32 * 0.05),
                        phase: i as f32 * 0.8,
                    },
                    Node {
                        position_type: PositionType::Absolute,
                        left: Val::Percent(0.0),
                        top: Val::Percent(0.0),
                        width: Val::Px(size),
                        height: Val::Px(size * 2.5), // Plus allongé (effet étincelle)
                        ..default()
                    },
                    // Couleur orange vif/étincelle
                    BackgroundColor(Color::srgba(0.95, 0.6, 0.2, 0.4)),
                ));
            }
        });
}

pub fn animate_abstract_background(
    time: Res<Time>,
    mut query: Query<(&BackgroundBlob, &mut Node)>,
) {
    let t = time.elapsed_secs();

    for (blob, mut node) in &mut query {
        // Mouvement plus dynamique : balancement horizontal + ascension
        let x = blob.base_x + (t * blob.speed * 2.0 + blob.phase).sin() * 10.0;
        let y = (blob.base_y - (t * 15.0 * blob.speed)) % 100.0;

        node.left = Val::Percent(x.abs() % 100.0);
        node.top = Val::Percent(y.abs());
    }
}

pub fn despawn_abstract_background(
    mut commands: Commands,
    query: Query<Entity, With<AbstractBackground>>,
) {
    for entity in &query {
        commands.entity(entity).despawn();
    }
}
