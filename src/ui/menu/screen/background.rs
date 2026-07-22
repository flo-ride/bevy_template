use bevy::prelude::*;

use crate::core::components::{AbstractBackground, BackgroundBlob};

const BLOB_LAYOUT: [(f32, f32, f32, f32, f32, f32, f32); 9] = [
    (-12.0, -8.0, 520.0, 28.0, 22.0, 0.18, 0.0),
    (62.0, -18.0, 440.0, 24.0, 30.0, 0.22, 1.2),
    (18.0, 38.0, 600.0, 32.0, 18.0, 0.15, 2.4),
    (78.0, 52.0, 380.0, 20.0, 26.0, 0.25, 3.1),
    (-8.0, 68.0, 460.0, 26.0, 20.0, 0.20, 4.5),
    (42.0, 8.0, 320.0, 18.0, 24.0, 0.28, 0.8),
    (88.0, 22.0, 280.0, 14.0, 16.0, 0.32, 5.2),
    (8.0, 22.0, 360.0, 22.0, 28.0, 0.17, 1.9),
    (55.0, 72.0, 400.0, 30.0, 14.0, 0.21, 2.8),
];

const BLOB_COLORS: [Color; 9] = [
    Color::srgba(0.55, 0.22, 0.78, 0.38),
    Color::srgba(0.18, 0.48, 0.92, 0.32),
    Color::srgba(0.92, 0.28, 0.52, 0.28),
    Color::srgba(0.12, 0.72, 0.68, 0.26),
    Color::srgba(0.38, 0.20, 0.82, 0.34),
    Color::srgba(0.95, 0.45, 0.18, 0.22),
    Color::srgba(0.22, 0.55, 0.95, 0.30),
    Color::srgba(0.72, 0.18, 0.62, 0.24),
    Color::srgba(0.15, 0.35, 0.75, 0.36),
];

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
            BackgroundColor(Color::srgb(0.04, 0.05, 0.12)),
            ZIndex(-10),
        ))
        .with_children(|parent| {
            for (index, (x, y, size, amp_x, amp_y, speed, phase)) in BLOB_LAYOUT.iter().enumerate()
            {
                let radius = size / 2.0;
                parent.spawn((
                    BackgroundBlob {
                        base_x: *x,
                        base_y: *y,
                        amplitude_x: *amp_x,
                        amplitude_y: *amp_y,
                        speed: *speed,
                        phase: *phase,
                        spin: 0.15 + index as f32 * 0.04,
                    },
                    Node {
                        position_type: PositionType::Absolute,
                        left: Val::Percent(*x),
                        top: Val::Percent(*y),
                        width: Val::Px(*size),
                        height: Val::Px(*size),
                        border_radius: BorderRadius::all(Val::Px(radius)),
                        ..default()
                    },
                    BackgroundColor(BLOB_COLORS[index]),
                ));
            }

            parent.spawn((
                Node {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    position_type: PositionType::Absolute,
                    ..default()
                },
                BackgroundColor(Color::srgba(0.02, 0.03, 0.08, 0.45)),
            ));
        });
}

pub fn animate_abstract_background(
    time: Res<Time>,
    mut query: Query<(&BackgroundBlob, &mut Node)>,
) {
    let t = time.elapsed_secs();

    for (blob, mut node) in &mut query {
        let x = blob.base_x
            + blob.amplitude_x * (t * blob.speed + blob.phase).sin()
            + blob.amplitude_y * (t * blob.speed * 0.7 + blob.phase).cos() * 0.35;
        let y = blob.base_y
            + blob.amplitude_y * (t * blob.speed * 0.9 + blob.phase + 1.0).cos()
            + blob.amplitude_x * (t * blob.speed * 0.6).sin() * 0.25;

        node.left = Val::Percent(x);
        node.top = Val::Percent(y);

        let pulse = 1.0 + 0.06 * (t * blob.spin + blob.phase).sin();
        let size = 520.0 * pulse;
        node.width = Val::Px(size);
        node.height = Val::Px(size);
        node.border_radius = BorderRadius::all(Val::Px(size / 2.0));
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
