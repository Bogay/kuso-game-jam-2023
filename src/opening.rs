use crate::audio::sound_event::SoundEvent;
use crate::config::data_layout::LayoutData;
use crate::game::assets::{AlbumId, FontId, SoundId};
use crate::game::{AssetStorage, CleanupOnGameplayEnd, TextureId};
use crate::positioning::Depth;
use crate::AppState;
use bevy::prelude::*;
use iyes_loopless::prelude::{AppLooplessStateExt, ConditionSet};
use iyes_loopless::state::NextState;

pub struct OpeningPlugin;

#[derive(Component)]
pub struct PlayingOpening(pub usize);

#[derive(Component)]
pub struct OpeningAnimation {
    timer: Timer,
    played: bool,
    index: usize,
}

#[derive(Component)]
pub struct Subtitle {
    title_text_style: TextStyle,
    text_alignment: TextAlignment,
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

pub fn init_opening(mut commands: Commands, assets: Res<AssetStorage>, layout: Res<LayoutData>) {
    info!("Add animation assets");
    commands.spawn().insert(PlayingOpening(0));
    commands
        .spawn_bundle(SpriteBundle {
            texture: assets.texture(&TextureId::Start01),
            transform: Transform::from_xyz(0.0, 0.0, Depth::Background.z()),
            ..default()
        })
        .insert(CleanupOnGameplayEnd)
        .insert(OpeningAnimation {
            timer: Timer::from_seconds(5., false),
            played: false,
            index: 0,
        });
    commands
        .spawn_bundle(SpriteBundle {
            texture: assets.texture(&TextureId::Start02),
            transform: Transform::from_xyz(0.0, 0.0, Depth::Background.z()),
            visibility: Visibility { is_visible: false },
            ..default()
        })
        .insert(CleanupOnGameplayEnd)
        .insert(OpeningAnimation {
            timer: Timer::from_seconds(5., false),
            played: false,
            index: 1,
        });
    commands
        .spawn_bundle(SpriteBundle {
            texture: assets.texture(&TextureId::Start03),
            transform: Transform::from_xyz(0.0, 0.0, Depth::Background.z()),
            visibility: Visibility { is_visible: false },
            ..default()
        })
        .insert(CleanupOnGameplayEnd)
        .insert(OpeningAnimation {
            timer: Timer::from_seconds(5., false),
            played: false,
            index: 2,
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
    commands
        .spawn_bundle(Text2dBundle {
            text: Text::from_section("測試", title_text_style.clone())
                .with_alignment(text_alignment),
            transform: Transform::from_translation(Vec3::new(
                screen_anchor.x,
                screen_anchor.y - menu_screen_dimens.y * 25.,
                Depth::Menu.z() + 10.,
            ))
            .with_scale(Vec3::ONE),
            ..default()
        })
        .insert(Subtitle {
            title_text_style: title_text_style,
            text_alignment: text_alignment,
        });
}

pub fn change_txt(txt: &mut Text, subtitle: &Subtitle, target: String) {
    *txt = Text::from_section(target, subtitle.title_text_style.clone())
        .with_alignment(subtitle.text_alignment)
}

pub fn change_white_txt(
    txt: &mut Text,
    subtitle: &Subtitle,
    target: String,
    assets: &AssetStorage,
) {
    *txt = Text::from_section(
        target,
        TextStyle {
            font: assets.font(&FontId::MSBold),
            font_size: 64.0,
            color: Color::WHITE,
        },
    )
    .with_alignment(subtitle.text_alignment)
}

pub fn opening(
    mut commands: Commands,
    mut anim_query: Query<(&mut Visibility, &mut Transform, &mut OpeningAnimation)>,
    mut now_play_query: Query<&mut PlayingOpening>,
    time: Res<Time>,
    mut txt: Query<(&mut Text, &Subtitle), With<Subtitle>>,
    mut audio: EventWriter<SoundEvent>,
    assets: Res<AssetStorage>,
    input: Res<Input<KeyCode>>,
) {
    trace!("opening");

    let mut now_play = now_play_query.single_mut();
    let (mut txt, subtitle) = txt.single_mut();
    for (mut vis, mut transform, mut anim) in anim_query.iter_mut() {
        vis.is_visible = anim.index == now_play.0;

        if anim.index == now_play.0 {
            if !anim.timer.finished() {
                transform.scale.x += 0.001;
                transform.scale.y += 0.001;

                if !anim.played {
                    if now_play.0 == 0 {
                        audio.send(SoundEvent::PlayAlbum(AlbumId::Opening));
                        audio.send(SoundEvent::Sfx(SoundId::Start01Nar));
                        change_txt(
                            &mut txt,
                            subtitle,
                            "你是部落中的巫師\n你一直以來都認為自己的部落是最強大的\n".to_string(),
                        );
                    } else if now_play.0 == 1 {
                        audio.send(SoundEvent::Sfx(SoundId::Start02Nar));
                        change_txt(&mut txt, subtitle, "直到外來勢力的入侵打破了這個平衡\n這些入侵者帶來了疾病、屠殺和毀滅\n你感到非常絕望".to_string());
                    } else {
                        audio.send(SoundEvent::Sfx(SoundId::Start03Nar));
                        change_white_txt(
                        &mut txt,
                        subtitle,
                        "在你的絕望之中，你啟動了一個神秘的法術\n讓你連接到400年前的世界\n你可以帶著現代的東西回到過去\n並利用時間的力量去改變部落的命運".to_string(),
                        &assets
                    );
                    }
                    anim.played = true;
                }
            }

            if anim.timer.tick(time.delta()).just_finished() {
                now_play.0 += 1;
                break;
            }
        }
    }

    let skip = cfg!(debug_assertions) && input.just_pressed(KeyCode::Back);
    if skip || now_play.0 > 2 {
        audio.send(SoundEvent::KillAllMusic);
        audio.send(SoundEvent::KillAllSoundEffects);
        commands.insert_resource(NextState(AppState::MainMenu));
    }
}
