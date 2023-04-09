use crate::{
    config::data_layout::LayoutData,
    game::{AssetStorage, CleanupOnGameplayEnd},
    positioning::{Depth, Dimens},
};
use bevy::prelude::*;

const INSTRUCTION_HEIGHT: f32 = 2.;
const INSTRUCTION_CONTENT_PADDING: f32 = 0.1;

pub fn create_layout_instruction(
    mut commands: Commands,
    layout: Res<LayoutData>,
    assets: Res<AssetStorage>,
) {
    let instruction_pos = (
        layout.middle_x(),
        layout.c_mid.toasts.height_with_margin() + layout.c_mid.inventory.height_with_margin(),
    );
    let inventory_dimens = Dimens::new(8, 5);

    commands
        .spawn_bundle(SpriteBundle {
            sprite: Sprite {
                color: Color::rgba(0.2, 0.2, 0.2, 0.8),
                custom_size: Some(Vec2::new(layout.middle_width(), INSTRUCTION_HEIGHT)),
                ..Default::default()
            },
            // Calculate sprite center
            transform: Transform::from_xyz(
                instruction_pos.0 + inventory_dimens.x as f32 * 0.5,
                instruction_pos.1 + inventory_dimens.y as f32 * 0.5,
                Depth::Grid.z(),
            ),
            ..Default::default()
        })
        .insert(Name::new("Instruction"))
        .insert(CleanupOnGameplayEnd)
        .with_children(|parent| {
            let style = TextStyle {
                font: assets.font(&crate::game::FontId::MSBold),
                font_size: 72.,
                color: Color::WHITE,
            };
            let instruction_content = [
                "- 按住滑鼠左鍵可拖曳道具",
                "- 拖曳到右側的道具可以帶著穿越時空",
                "- 按住 L-Ctrl + L-Alt 後點擊左鍵可刪除道具",
            ]
            .join("\n");
            debug!("Instruction content: {:?}", instruction_content);
            let transform = Transform::from_xyz(
                -inventory_dimens.x as f32 * 0.5 + INSTRUCTION_CONTENT_PADDING,
                INSTRUCTION_HEIGHT * 0.5 - INSTRUCTION_CONTENT_PADDING,
                0.1,
            )
            .with_scale(Vec3::new(
                1. / layout.text_factor,
                1. / layout.text_factor,
                1.,
            ));
            parent.spawn_bundle(Text2dBundle {
                text: Text::from_section(instruction_content, style),
                transform,
                ..Default::default()
            });
        });
}
