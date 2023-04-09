use std::time::Duration;

use bevy::ecs::system::Resource;
use bevy::prelude::*;
use iyes_loopless::prelude::{AppLooplessStateExt, ConditionSet};
use iyes_loopless::state::NextState;

use crate::audio::sound_event::SoundEvent;
use crate::config::data_layout::LayoutData;
use crate::game::assets::{SoundId, FontId, AlbumId};
use crate::game::{AssetStorage, TextureId, assets, CleanupOnGameplayEnd, MENU_ZOOM};
use crate::AppState;
use crate::positioning::Depth;

pub struct OpeningPlugin;

#[derive(Component)]
pub struct StartEntity01 {
    timer: Timer,
    played: bool
}
#[derive(Component)]
pub struct StartEntity02 {
    timer: Timer,
    played: bool
}
#[derive(Component)]
pub struct StartEntity03 {
    timer: Timer,
    played: bool
}
#[derive(Component)]
pub struct Subtitle {
    title_text_style: TextStyle,
    text_alignment: TextAlignment
}

impl Plugin for OpeningPlugin {
    fn build(&self, app: &mut App) {
        app.add_enter_system_set(
                AppState::Opening,
                ConditionSet::new()
                    .run_in_state(AppState::Opening)
                    .with_system(init_opening)
                    .into(),
            )
            .add_system_set(
                ConditionSet::new()
                    .run_in_state(AppState::Opening)
                    .with_system(opening)
                    .into(),
            );
    }
}
pub fn init_opening(
    mut commands: Commands,
    assets: Res<AssetStorage>,
    layout: Res<LayoutData>
) {
    info!("add animation assets");
    commands
        .spawn_bundle(SpriteBundle {
            sprite: Sprite {
                ..default()
            },
            texture: assets.texture(&TextureId::Start01),
            transform: Transform::from_xyz(0.0, 0.0, Depth::Background.z()),
            visibility: Visibility {
                is_visible: true,
            },
            ..default()
        })
        .insert(CleanupOnGameplayEnd)
        .insert(StartEntity01 {
            timer: Timer::new(Duration::from_secs(5), true),
            played: false
        });

    commands
        .spawn_bundle(SpriteBundle {
            sprite: Sprite {
                ..default()
            },
            texture: assets.texture(&TextureId::Start02),
            transform: Transform::from_xyz(0.0, 0.0, Depth::Background.z()),
            visibility: Visibility {
                is_visible: false,
            },
            ..default()
        })
        .insert(CleanupOnGameplayEnd)
        .insert(StartEntity02 {
            timer: Timer::new(Duration::from_secs(5), false),
            played: false
        });

    commands
        .spawn_bundle(SpriteBundle {
            sprite: Sprite {
                ..default()
            },
            texture: assets.texture(&TextureId::Start03),
            transform: Transform::from_xyz(0.0, 0.0, Depth::Background.z()),
            visibility: Visibility {
                is_visible: false,
            },
            ..default()
        })
        .insert(CleanupOnGameplayEnd)
        .insert(StartEntity03 {
            timer: Timer::new(Duration::from_secs(5), false),
            played: false
        });
    
    let menu_screen_dimens = layout.screen_dimens;
    let screen_center = layout.screen_dimens * 0.5;
    let screen_anchor = screen_center - menu_screen_dimens * 0.5;
    let title_text_style = TextStyle {
        font: assets.font(&FontId::MSBold),
        font_size: 64.0,
        color: Color::BLACK,
    };
    let text_alignment = TextAlignment {
        horizontal: HorizontalAlign::Center,
        vertical: VerticalAlign::Bottom,
    };
    commands.spawn_bundle(Text2dBundle {
            text: Text::from_section("測試", title_text_style.clone()).with_alignment(text_alignment),
            transform: Transform::from_translation(Vec3::new(
                screen_anchor.x, 
                screen_anchor.y - menu_screen_dimens.y * 25.,
                Depth::Menu.z() + 10.,
            ))
            .with_scale(Vec3::new(
                1.,
                1.,
                1.,
            )),
            ..default()
        }).insert(Subtitle {
            title_text_style: title_text_style,
            text_alignment: text_alignment
        });
    
}

pub fn change_txt(
    txt: &mut Text,
    subtitle: &Subtitle,
    target:String
){
    *txt = Text::from_section(target, subtitle.title_text_style.clone()).with_alignment(subtitle.text_alignment)
}

pub fn change_white_txt(
    txt: &mut Text,
    subtitle: &Subtitle,
    target:String,
    assets: Res<AssetStorage>,
){
    *txt = Text::from_section(target, TextStyle {
        font: assets.font(&FontId::MSBold),
        font_size: 64.0,
        color: Color::WHITE,
    }).with_alignment(subtitle.text_alignment)
}

pub fn opening(
    mut commands: Commands,
    mut query: ParamSet<(
        Query<(&mut Visibility, &mut Transform, &mut StartEntity01), With<StartEntity01>>,
        Query<(&mut Visibility, &mut Transform, &mut StartEntity02), With<StartEntity02>>,
        Query<(&mut Visibility, &mut Transform, &mut StartEntity03), With<StartEntity03>>,
    )>,
    time: Res<Time>,
    mut txt: Query<(&mut Text, &Subtitle), With<Subtitle>>,
    mut audio: EventWriter<SoundEvent>,
    assets: Res<AssetStorage>,
){
    info!("start render");
    let (mut txt, subtitle) = txt.single_mut();

    //start0
    if !query.p0().single().2.timer.finished() {
        query.p0().single_mut().2.timer.tick(time.delta());
        query.p0().single_mut().1.scale.x += 0.001;
        query.p0().single_mut().1.scale.y += 0.001;

        if !query.p0().single().2.played {
            audio.send(SoundEvent::PlayAlbum(AlbumId::Opening));
            //audio.send(SoundEvent::Sfx(SoundId::Start01Nar));
            query.p0().single_mut().2.played = true;
            change_txt(&mut txt, subtitle, "你是部落中的巫師\n你一直以來都認為自己的部落是最強大的\n".to_string());
        }
    }
    else {
        query.p0().single_mut().0.is_visible = false;
        query.p1().single_mut().0.is_visible = true;

        //start1
        if !query.p1().single().2.timer.finished() {
            query.p1().single_mut().2.timer.tick(time.delta());
            query.p1().single_mut().1.scale.x += 0.001;
            query.p1().single_mut().1.scale.y += 0.001;

            if !query.p1().single().2.played {
                //audio.send(SoundEvent::Sfx(SoundId::Start02Nar));
                query.p1().single_mut().2.played = true;
                change_txt(&mut txt, subtitle, "直到外來勢力的入侵打破了這個平衡\n這些入侵者帶來了疾病、屠殺和毀滅\n你感到非常絕望".to_string());
            }
        }
        else {
            query.p1().single_mut().0.is_visible = false;
            query.p2().single_mut().0.is_visible = true;

            //start2
            if !query.p2().single().2.timer.finished() {
                query.p2().single_mut().2.timer.tick(time.delta());
                query.p2().single_mut().1.scale.x += 0.001;
                query.p2().single_mut().1.scale.y += 0.001;

                if !query.p2().single().2.played {
                    //audio.send(SoundEvent::Sfx(SoundId::Start03Nar));
                    query.p2().single_mut().2.played = true;
                    change_white_txt(
                        &mut txt, 
                        subtitle, 
                        "在你的絕望之中，你啟動了一個神秘的法術\n讓你連接到400年前的世界\n你可以帶著現代的東西回到過去\n並利用時間的力量去改變部落的命運".to_string(),
                        assets
                    );
                }
            }
            else {
                info!("kill & change");
    
                query.p2().single_mut().0.is_visible = false;
                audio.send(SoundEvent::KillAllMusic);
                audio.send(SoundEvent::KillAllSoundEffects);
                
                commands.insert_resource(NextState(AppState::MainMenu));
            }
        }
    }

}