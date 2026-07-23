use crate::core::components::MainCamera;
use crate::core::states::{GameState, InGameView};
use bevy::prelude::*;

pub struct ViewsPlugin;

const CUSTOMERS_VIEW_X: f32 = 0.0;
const ALCHEMY_VIEW_X: f32 = 5000.0; // Pushed much further to the right!
const CAMERA_MOVE_SPEED: f32 = 5.0; // Speed of the lerp

impl Plugin for ViewsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Playing), setup_view_ui)
            .add_systems(
                Update,
                (
                    handle_view_switch_input,
                    handle_view_switch_buttons,
                    animate_camera_transition,
                    update_button_visibility,
                )
                    .run_if(in_state(GameState::Playing)),
            );
    }
}

#[derive(Component)]
struct ViewSwitchButton(InGameView);

fn setup_view_ui(mut commands: Commands) {
    // Right arrow (Go to Alchemy) - visible in Customers view
    commands
        .spawn((
            Button,
            Node {
                position_type: PositionType::Absolute,
                right: Val::Px(20.0),
                top: Val::Percent(50.0),
                width: Val::Px(60.0),
                height: Val::Px(60.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            BackgroundColor(Color::srgba(0.2, 0.2, 0.2, 0.8)),
            ViewSwitchButton(InGameView::Alchemy),
            Visibility::Visible,
        ))
        .with_child((Text::new(">"), TextColor(Color::WHITE)));

    // Left arrow (Go to Customers) - visible in Alchemy view
    commands
        .spawn((
            Button,
            Node {
                position_type: PositionType::Absolute,
                left: Val::Px(20.0),
                top: Val::Percent(50.0),
                width: Val::Px(60.0),
                height: Val::Px(60.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            BackgroundColor(Color::srgba(0.2, 0.2, 0.2, 0.8)),
            ViewSwitchButton(InGameView::Customers),
            Visibility::Hidden,
        ))
        .with_child((Text::new("<"), TextColor(Color::WHITE)));
}

fn handle_view_switch_input(
    input: Res<ButtonInput<KeyCode>>,
    current_view: Res<State<InGameView>>,
    mut next_view: ResMut<NextState<InGameView>>,
) {
    if input.just_pressed(KeyCode::Space) || input.just_pressed(KeyCode::Tab) {
        match current_view.get() {
            InGameView::Customers => next_view.set(InGameView::Alchemy),
            InGameView::Alchemy => next_view.set(InGameView::Customers),
        }
    }
}

fn handle_view_switch_buttons(
    interaction_query: Query<(&Interaction, &ViewSwitchButton), Changed<Interaction>>,
    mut next_view: ResMut<NextState<InGameView>>,
) {
    for (interaction, target) in interaction_query.iter() {
        if *interaction == Interaction::Pressed {
            next_view.set(target.0.clone());
        }
    }
}

fn update_button_visibility(
    current_view: Res<State<InGameView>>,
    mut button_query: Query<(&mut Visibility, &ViewSwitchButton)>,
) {
    // Only update if state changed
    if current_view.is_changed() {
        for (mut visibility, target) in button_query.iter_mut() {
            // The button that points to Alchemy is shown when in Customers (which means current target != current view)
            if target.0 != *current_view.get() {
                *visibility = Visibility::Visible;
            } else {
                *visibility = Visibility::Hidden;
            }
        }
    }
}

fn animate_camera_transition(
    time: Res<Time>,
    current_view: Res<State<InGameView>>,
    mut camera_query: Query<&mut Transform, With<MainCamera>>,
) {
    let target_x = match current_view.get() {
        InGameView::Customers => CUSTOMERS_VIEW_X,
        InGameView::Alchemy => ALCHEMY_VIEW_X,
    };

    if let Some(mut transform) = camera_query.iter_mut().next() {
        transform.translation.x = transform
            .translation
            .x
            .lerp(target_x, time.delta_secs() * CAMERA_MOVE_SPEED);
    }
}
