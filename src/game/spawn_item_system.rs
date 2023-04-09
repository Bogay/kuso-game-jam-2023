use bevy::prelude::*;
use iyes_loopless::prelude::ConditionSet;

use super::backpack::BackpackInUse;
use super::ItemStack;
use crate::config::data_layout::LayoutData;
use crate::game::backpack::Backpack;
use crate::game::items::Item;
use crate::game::{AssetStorage, CleanupOnGameplayEnd, FallingItem, Silhouette};
use crate::mouse::MouseInteractive;
use crate::positioning::{Coords, GridData};
use crate::positioning::{Depth, Dimens, Pos};
use crate::states::AppState;

pub struct SpawnItemPlugin;

impl Plugin for SpawnItemPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<StackItemEvent>().add_system_set(
            ConditionSet::new()
                .run_in_state(AppState::InGame)
                .with_system(stack_item)
                .into(),
        );
    }
}

/// Broadcast this as an event to spawn an item.
#[derive(Debug)]
pub struct SpawnItemEvent {
    item: Item,
    coords: Coords,
    /// If it spawns as an animated FallingItem, where does it appear?
    ///
    /// Set to to None for any items that are present at the start of the game. They will spawn
    /// in the inventory without any animations.
    source: Option<Vec2>,
    combine: bool,
    /// Which backpack this item should be put it, default to current backpack in use
    backpack: Option<usize>,
}

#[derive(Debug)]
pub struct StackItemEvent {
    pub item: Item,
    pub count: usize,
    /// Which backpack this item should be put it, default to current backpack in use
    pub backpack: Option<usize>,
}

// TODO: impl builder to simplify construction process
impl SpawnItemEvent {
    pub fn new(item: Item, coords: Coords, source: Vec2, combine: bool) -> Self {
        SpawnItemEvent {
            item,
            coords,
            source: Some(source),
            combine,
            backpack: None,
        }
    }
    /// Use this for items that already exist in the backpack at the start of the game.
    pub fn without_anim(item: Item, coords: Coords) -> Self {
        SpawnItemEvent {
            item,
            coords,
            source: None,
            combine: false,
            backpack: None,
        }
    }
    /// Use this for items that should be spawned to specific backpack
    pub fn with_backpack(item: Item, coords: Coords, source: Vec2, backpack: usize) -> Self {
        SpawnItemEvent {
            item,
            coords,
            source: Some(source),
            combine: false,
            backpack: Some(backpack),
        }
    }
}

pub fn spawn_item(
    mut commands: Commands,
    mut events: EventReader<SpawnItemEvent>,
    backpack_in_use: Query<&BackpackInUse>,
    assets: Res<AssetStorage>,
    grid: Res<GridData>,
    layout: Res<LayoutData>,
) {
    let default_backpack_id = match backpack_in_use.get_single() {
        Ok(BackpackInUse(backpack_id)) => *backpack_id,
        Err(e) => {
            error!(
                "There should be only one BadInUse component in game.\n{}",
                e
            );
            return;
        }
    };

    for evt in events.iter() {
        debug!("Received {:?}", evt);
        let SpawnItemEvent {
            item,
            coords,
            source,
            combine,
            backpack,
        } = evt;
        if let Some(source) = source {
            // Spawn the animating item.
            commands
                .spawn_bundle(SpriteBundle {
                    sprite: Sprite {
                        // BOGAY: affects falling item
                        custom_size: Some(coords.dimens.as_vec2()),
                        ..default()
                    },
                    texture: assets.texture(&item.texture_id),
                    transform: Transform::from_xyz(source.x, source.y, Depth::FloatingItem.z()),
                    ..Default::default()
                })
                .insert(Name::new("FallingItem"))
                .insert(FallingItem::new(
                    *coords,
                    *source,
                    coords.pos.as_vec2() + grid.offset,
                    if *combine { 0.75 } else { 1.25 },
                ))
                .insert(CleanupOnGameplayEnd);
        }
        // Spawn the silhouette.
        let backpack_id = backpack.unwrap_or(default_backpack_id);
        let mut builder = commands.spawn();
        builder
            .insert_bundle(SpriteBundle {
                sprite: Sprite {
                    // BOGAY: this affects silhouette size, of course
                    custom_size: Some(coords.dimens.as_vec2()),
                    ..default()
                },
                texture: assets.texture(&item.texture_id),
                transform: Transform::from_xyz(
                    grid.offset.x + coords.pos.x as f32 + coords.dimens.x as f32 * 0.5,
                    grid.offset.y + coords.pos.y as f32 + coords.dimens.y as f32 * 0.5,
                    Depth::Item.z(),
                ),
                ..Default::default()
            })
            .insert(Name::new(item.name.clone()))
            .insert(item.clone())
            .insert(*coords)
            .insert(MouseInteractive::new(coords.dimens.as_vec2(), true))
            .insert(CleanupOnGameplayEnd)
            .insert(Backpack(backpack_id))
            // create child to render stack count
            .with_children(|parent| {
                let font = &crate::game::FontId::MSBold;
                let text_style = TextStyle {
                    font: assets.font(font),
                    font_size: 60.0,
                    color: Color::WHITE,
                };
                parent.spawn_bundle(Text2dBundle {
                    text: Text::from_section("NULL", text_style),
                    transform: Transform::from_translation(Vec3::new(0., 0., 1.0)).with_scale(
                        Vec3::new(1. / layout.text_factor, 1. / layout.text_factor, 1.),
                    ),
                    ..Default::default()
                });
            });
        if source.is_some() {
            builder.insert(Silhouette);
        }
    }
}

pub fn animate_falling_item(
    mut commands: Commands,
    time: Res<Time>,
    mut query_falling: Query<(Entity, &mut FallingItem, &mut Transform)>,
    query_cleanup: Query<(Entity, &Coords), With<Silhouette>>,
) {
    for (entity, mut item, mut transform) in query_falling.iter_mut() {
        item.timer.tick(time.delta());
        if item.timer.finished() {
            commands.entity(entity).despawn_recursive();
            if let Some((silhouette_entity, _)) = query_cleanup
                .iter()
                .find(|(_, coords)| **coords == item.coords)
            {
                commands.entity(silhouette_entity).remove::<Silhouette>();
            }
        } else {
            let progress = item.timer.percent().powi(2);
            let delta_total = item.target - item.source;
            let delta_current = delta_total * progress;
            let current_pos = delta_current + item.source;
            transform.translation.x = current_pos.x + item.coords.dimens.x as f32 * 0.5;
            transform.translation.y = current_pos.y + item.coords.dimens.y as f32 * 0.5;
            transform.scale.x = 1. + (1. - progress);
            transform.scale.y = 1. + (1. - progress);
        }
    }
}

pub fn find_free_space<'a, I>(
    grid: &GridData,
    dimens: Dimens,
    items_query: &'a I,
    same_tick_items: &[Coords], // Pass this an emtpy vec if not multiple spawn
) -> Option<Coords>
where
    &'a I: IntoIterator<Item = &'a Coords>,
{
    for y in 0..grid.inventory.dimens.y {
        for x in 0..grid.inventory.dimens.x {
            let coords = Coords {
                pos: Pos::new(x, y),
                dimens,
            };

            let overlap_conflict = items_query.into_iter().any(|item| coords.overlaps(item))
                || same_tick_items.iter().any(|item| coords.overlaps(item));
            let bound_conflict = !grid.inventory.encloses(&coords);
            if !overlap_conflict && !bound_conflict {
                return Some(coords);
            }
        }
    }
    None
}

fn stack_item(
    mut commands: Commands,
    mut events: EventReader<StackItemEvent>,
    mut ew_spanw_item: EventWriter<SpawnItemEvent>,
    mut items_query: Query<(Entity, &Item, &Backpack, &Coords, Option<&mut ItemStack>)>,
    backpack_in_use: Query<&BackpackInUse>,
    grid: Res<GridData>,
) {
    let default_backpack_id = match backpack_in_use.get_single() {
        Ok(BackpackInUse(backpack_id)) => *backpack_id,
        Err(e) => {
            error!(
                "There should be only one BadInUse component in game.\n{}",
                e
            );
            return;
        }
    };

    let mut new_coords_this_round: Vec<(usize, Coords)> = vec![];
    for evt in events.iter() {
        debug!("Received {:?}", evt);
        let StackItemEvent {
            item,
            count,
            backpack,
        } = evt;
        let backpack_id = backpack.unwrap_or(default_backpack_id);
        if let Some((ent, _, _, _, item_stack)) = items_query
            .iter_mut()
            // search item in target backpack
            .find(|(_, c_item, backpack, _, _)| backpack.0 == backpack_id && c_item.id == item.id)
        {
            if let Some(mut item_stack) = item_stack {
                item_stack.0 += count;
            } else {
                commands.entity(ent).insert(ItemStack(*count + 1));
            }
        } else {
            let new_coords_in_backpack = new_coords_this_round
                .iter()
                .filter_map(|(c, it)| (*c == backpack_id).then(|| it.clone()));
            let curr_coords = items_query
                .iter()
                .filter(|(_, _, backpack, _, _)| backpack.0 == backpack_id)
                .map(|(_, _, _, coords, _)| *coords)
                .chain(new_coords_in_backpack)
                .collect::<Vec<_>>();
            let coords = find_free_space(&grid, Dimens::unit(), &vec![], &curr_coords)
                .expect("should find a free space");
            ew_spanw_item.send(SpawnItemEvent::without_anim(item.clone(), coords));
            new_coords_this_round.push((backpack_id, coords));
        }
    }
}
