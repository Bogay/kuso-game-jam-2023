use crate::config::data_enemies::EnemiesData;
use crate::game::combat::{DropTable, Enemy, EnemyId};
use crate::game::dungeon_components::TextType;
use crate::game::sim::dungeon_components::Room;
use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::hash::Hash;

use super::dungeon_components::{TimePoint, TimePointLevel};

pub const TIMEPOINT_NOW: i32 = 400;
pub const TIMEPOINT_ANCIENT: i32 = 0;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct LevelBlueprint {
    pub depth: i32,
    pub default_loot: DropTable,
    pub segments: Vec<SegmentBlueprint>,
}

/// Base building block for the .ron dungeon designs
/// Contains possible room types, custom loot, custom flavour texts, and monster spawn rates.
/// One "segment" results in one room generated.
/// Enemy and room spawn percentages must add up to 100.
/// NOTE: Custom loot works only in empty rooms. Corridors don't yield loot, enemies have their own loot.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct SegmentBlueprint {
    pub types: HashMap<RoomType, u32>,
    pub enemies: Option<HashMap<EnemyId, u32>>,
    pub custom_loot: Option<DropTable>,
    pub custom_flavour: Option<TextType>,
}

#[derive(Default, Clone, Deserialize, Serialize, Eq, PartialEq, Hash, Debug)]
pub enum RoomType {
    #[default]
    Empty,
    Fight,
    Corridor,
    Start,
    End,
}

fn gen_timepoint(time: i32) -> TimePoint {
    TimePoint {
        timepoint: time,
        flavour: None,
    }
}

pub fn generate_level(mut _cmd: &mut Commands) -> TimePointLevel {
    let timepoints = vec![
        gen_timepoint(TIMEPOINT_ANCIENT),
        gen_timepoint(TIMEPOINT_NOW),
    ];

    TimePointLevel {
        // TODO: does this always == timepoints.len()?
        timenum: 2,
        timepoints,
    }
}

fn generate_first_room() -> Room {
    Room {
        start: true,
        ..Default::default()
    }
}

fn generate_last_room() -> Room {
    Room {
        end: true,
        ..Default::default()
    }
}

fn generate_corridor() -> Room {
    Room {
        corridor: true,
        ..Default::default()
    }
}

fn generate_empty() -> Room {
    Room {
        door: true,
        description: true,
        search: true,
        ..Default::default()
    }
}

fn generate_fight() -> Room {
    Room {
        door: true,
        search: true,
        combat: true,
        ..Default::default()
    }
}

fn get_enemy(enemies: &Res<EnemiesData>, enemy_id: EnemyId) -> Enemy {
    if let Some(nmy) = enemies
        .enemies
        .clone()
        .into_iter()
        .find(|p| p.enemy_id == enemy_id)
    {
        return nmy;
    }
    error!("Error during enemy generation, returning default enemy!");
    return Enemy::default();
}
