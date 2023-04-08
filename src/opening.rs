use bevy::prelude::*;
use iyes_loopless::prelude::{AppLooplessStateExt, ConditionSet};

use crate::game::{AssetStorage, TextureId, assets, CleanupOnGameplayEnd};
use crate::AppState;
use crate::positioning::Depth;

pub struct OpeningPlugin;

impl Plugin for OpeningPlugin {
    fn build(&self, app: &mut App) {
        app.add_enter_system_set(
                AppState::Opening,
                ConditionSet::new()
                    .run_in_state(AppState::Opening)
                    .with_system(init_opening)
                    .into(),
            );
    }
}
pub fn init_opening(
    mut commands: Commands,
    assets: Res<AssetStorage>,
){
    info!("add animation assets");
    commands
        .spawn_bundle(SpriteBundle {
            sprite: Sprite {
                ..default()
            },
            texture: assets.texture(&TextureId::Start01),
            transform: Transform::from_xyz(0.0, 0.0, Depth::Background.z()),
            ..default()
        })
        .insert(CleanupOnGameplayEnd);
}
