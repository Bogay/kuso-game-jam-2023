use bevy::prelude::*;
use iyes_loopless::prelude::ConditionSet;
use iyes_loopless::prelude::NextState;
use crate::audio::sound_event::SoundEvent;
use crate::config::data_items::ItemsData;
use crate::config::data_recipes::RecipesData;
use crate::game::items::Item;
use crate::game::recipes::Recipe;
use crate::game::{find_free_space, ItemId, SoundId, SpawnItemEvent};
use crate::game::{GameResult};
use crate::mouse::MouseInteractive;
use crate::positioning::{Coords, Dimens, GridData};
use crate::states::AppState;

use super::backpack::Backpack;
use super::dungeon_sim::{ContinuePrompt, JumpTimepointEvent};
use super::items::CraftItem;

#[derive(Component)]
pub struct CombineButton {
    pub coords: Coords,
}

pub struct WinGamePlugin;

impl Plugin for WinGamePlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            ConditionSet::new()
                .run_in_state(AppState::InGame)
                .with_system(wingame)
                .into(),
        );
    }
}

fn contains<'a>(items: impl IntoIterator<Item = &'a &'a Item>, id: ItemId) -> bool {
    items.into_iter().find(|it| it.id == id).is_some()
}

fn wingame(
    items: Query<(Entity, &Item, &Backpack, &Coords)>,
    items_data: Res<ItemsData>,
    mut cmd: Commands,
    mut victory: ResMut<State<GameResult>>,
) {
    let now_items = items
        .iter()
        .filter(|(_, _, backpack, _)| backpack.0 == 400)
        .map(|(_, item, _, _)| item)
        .collect::<Vec<_>>();

    let win_conds = [ItemId::Theocracy, ItemId::PermanentMember, ItemId::Empire, ItemId::Totalitarian];
    for win_cond in win_conds {
        if (contains(&now_items, win_cond)) {
            if victory.current().clone() == GameResult::Won {
                victory.set(GameResult::Lost).unwrap();
            }
            info!("Win!");
            cmd.insert_resource(NextState(AppState::GameEnded));
        }
    }
}