use super::backpack::Backpack;
use crate::game::items::Item;
use crate::game::GameResult;
use crate::game::ItemId;
use crate::positioning::Coords;
use crate::states::AppState;
use bevy::prelude::*;
use iyes_loopless::prelude::ConditionSet;
use iyes_loopless::prelude::NextState;

pub struct WinGamePlugin;

impl Plugin for WinGamePlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            ConditionSet::new()
                .run_in_state(AppState::InGame)
                .with_system(win_game)
                .into(),
        );
    }
}

fn contains<'a>(items: impl IntoIterator<Item = &'a &'a Item>, id: ItemId) -> bool {
    items.into_iter().any(|it| it.id == id)
}

fn win_game(
    items: Query<(Entity, &Item, &Backpack, &Coords)>,
    mut cmd: Commands,
    mut victory: ResMut<State<GameResult>>,
) {
    let now_items = items
        .iter()
        .filter(|(_, _, backpack, _)| backpack.0 == 400)
        .map(|(_, item, _, _)| item)
        .collect::<Vec<_>>();

    let win_conds = [
        ItemId::Theocracy,
        ItemId::PermanentMember,
        ItemId::Empire,
        ItemId::Totalitarian,
    ];
    for win_cond in win_conds {
        if contains(&now_items, win_cond) {
            if victory.current().clone() == GameResult::Lost {
                victory.set(GameResult::Won).unwrap();
            }
            info!("Win!");
            cmd.insert_resource(NextState(AppState::GameEnded));
        }
    }
}
