use bevy::prelude::*;

pub fn spawn_hud_item(
    parent: &mut ChildSpawnerCommands,
    placement: crate::features::hud::placement::HudPlacement,
    content_spawner: impl FnOnce(&mut ChildSpawnerCommands),
) {
    let (justify, align) = match placement {
        crate::features::hud::placement::HudPlacement::TopLeft => {
            (JustifyContent::FlexStart, AlignItems::FlexStart)
        }
        crate::features::hud::placement::HudPlacement::TopCenter => {
            (JustifyContent::Center, AlignItems::FlexStart)
        }
        crate::features::hud::placement::HudPlacement::TopRight => {
            (JustifyContent::FlexEnd, AlignItems::FlexStart)
        }
        crate::features::hud::placement::HudPlacement::CenterLeft => {
            (JustifyContent::FlexStart, AlignItems::Center)
        }
        crate::features::hud::placement::HudPlacement::Center => {
            (JustifyContent::Center, AlignItems::Center)
        }
        crate::features::hud::placement::HudPlacement::CenterRight => {
            (JustifyContent::FlexEnd, AlignItems::Center)
        }
        crate::features::hud::placement::HudPlacement::BottomLeft => {
            (JustifyContent::FlexStart, AlignItems::FlexEnd)
        }
        crate::features::hud::placement::HudPlacement::BottomCenter => {
            (JustifyContent::Center, AlignItems::FlexEnd)
        }
        crate::features::hud::placement::HudPlacement::BottomRight => {
            (JustifyContent::FlexEnd, AlignItems::FlexEnd)
        }
    };

    parent
        .spawn(Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            padding: UiRect::all(Val::Px(10.0)),
            justify_content: justify,
            align_items: align,
            position_type: PositionType::Absolute,
            ..default()
        })
        .with_children(|p| {
            content_spawner(p);
        });
}
