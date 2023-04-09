use super::dungeon_components::TimePointLevel;
use crate::config::config_sim::SimConfig;
use crate::config::data_blueprint::BlueprintData;
use crate::config::data_enemies::EnemiesData;
use crate::game::backpack::{BackpackInUse, SwitchBackpackEvent};
use crate::game::combat::{DropTable, EnemyId};
use crate::game::event_handling::SimMessageEvent;
use crate::game::sim::combat::{process_combat, CombatState, Enemy, Hero};
use crate::game::sim::dungeon_components::{DungeonLevel, TextType};
use crate::game::sim::dungeon_gen::generate_level;
use crate::game::sim::event_handling::SimLootEvent;
use crate::game::{GameResult, ItemId};
use crate::AppState;
use bevy::prelude::*;
use iyes_loopless::prelude::NextState;
use rand::Rng;
use std::time::Duration;

const MAX_GAME_ROUND: i32 = 10;

/// Handle a state event. Mainly handle hero's death?
pub struct SimStateEvent(String);

#[derive(Default, Clone)]
pub struct DungeonState {
    pub max_depth: i32,
    pub cur_timepoint_idx: i32,
    pub current_level: Option<TimePointLevel>,
    pub msg_cooldown: Timer,
    pub running: bool,
    pub combat_state: CombatState,
    pub round: i32,
}

#[derive(Component)]
pub struct ContinuePrompt;

pub fn init_dungeon(
    mut commands: Commands,
    params: Res<SimConfig>,
    dungeon_bp: Res<BlueprintData>,
) {
    let mut state = DungeonState {
        max_depth: dungeon_bp.levels.len() as i32 - 1,
        cur_timepoint_idx: 0,
        current_level: None,
        msg_cooldown: Timer::new(Duration::from_millis(params.duration_millis), true),
        running: true,
        combat_state: CombatState::Init,
        round: 0,
    };
    state.current_level = Option::from(generate_level(&mut commands));
    commands.insert_resource(state);
}

pub fn sync_backpack_in_use(
    mut er_jump: EventReader<JumpTimepointEvent>,
    mut ew_switch: EventWriter<SwitchBackpackEvent>,
) {
    for JumpTimepointEvent { to, .. } in er_jump.iter() {
        ew_switch.send(SwitchBackpackEvent(*to));
    }
}

pub struct JumpTimepointEvent {
    pub from: usize,
    pub to: usize,
}

pub fn tick_timepoint(
    mut msg_events: EventWriter<SimMessageEvent>,
    mut loot_events: EventWriter<SimLootEvent>,
    dungeon_bp: Res<BlueprintData>,
    enemy_data: Res<EnemiesData>,
    time: Res<Time>,
    _config: ResMut<SimConfig>,
    mut state: ResMut<DungeonState>,
    mut hero: ResMut<Hero>,
    mut enemy: ResMut<Enemy>,
    input: Res<Input<KeyCode>>,
    mut cmd: Commands,
    mut victory: ResMut<State<GameResult>>,
    mut er_jump: EventReader<JumpTimepointEvent>,
) {
    for evt in er_jump.iter() {
        if evt.to == 0 {
            state.round += 1;
        }

        if state.round > MAX_GAME_ROUND {
            if victory.current().clone() == GameResult::Won {
                victory.set(GameResult::Lost).unwrap();
            }
            info!("Dungeon complete!");
            cmd.insert_resource(NextState(AppState::GameEnded));
            halt_dungeon_sim(&mut state);
            return;
        }

        halt_dungeon_sim(&mut state);
    }
}

pub fn halt_dungeon_sim(state: &mut DungeonState) {
    info!("Halting dungeon sim.");
    state.running = false;
}

pub fn resume_dungeon_sim(mut state: ResMut<DungeonState>) {
    info!("Resuming dungeon sim.");
    state.running = true;
}

fn pick_loot_from_drop_table(table: &DropTable) -> Vec<ItemId> {
    const MAX_ITEMS: usize = 3;
    let mut result = vec![];
    let mut rng = rand::thread_rng();
    for i in 0..table.items.len() {
        if result.len() == MAX_ITEMS {
            break;
        }
        let roll = rng.gen_range(1..=100);
        if roll <= table.chances[i] {
            result.push(table.items[i].clone());
        }
    }

    result
}

pub fn manage_continue_prompt(
    state: Res<DungeonState>,
    mut q: Query<&mut Text, With<ContinuePrompt>>,
) {
    let Ok(mut text) = q.get_single_mut() else {
        error!("Find more than one continue prompt");
        return;
    };

    if state.running {
        text.sections[0].value = "".to_string();
    } else if !state.running && state.combat_state != CombatState::HeroDead {
        // text.sections[0].value = "Press SPACE to continue exploring.".to_string();
    }
}
