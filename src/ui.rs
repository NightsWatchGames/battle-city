use std::time::Duration;

use bevy::prelude::*;

use crate::common::{
    AppState, GameSounds, GameTextureHandles, GameTextureLayout, MultiplayerMode,
    SPRITE_GAME_OVER_Z_ORDER,
};

#[derive(Component)]
pub struct OnStartMenuScreen;
#[derive(Component)]
pub struct OnStartMenuScreenMultiplayerModeFlag;

#[derive(Component)]
pub struct OnGameOverScreen;

pub fn setup_start_menu(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    game_sounds: Res<GameSounds>,
    game_texture_handles: Res<GameTextureHandles>,
    game_texture_atlas: Res<GameTextureLayout>,
) {
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.),
                    height: Val::Percent(100.),
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    ..default()
                },
                ..default()
            },
            OnStartMenuScreen,
        ))
        .with_children(|parent| {
            parent.spawn(ImageBundle {
                image: asset_server.load("textures/title.bmp").into(),
                ..default()
            });
            parent.spawn((
                ImageBundle {
                    image: UiImage::from(game_texture_handles.tanks.clone()),
                    style: Style {
                        width: Val::Px(20.),
                        height: Val::Px(20.),
                        margin: UiRect::all(Val::Px(10.0)),
                        position_type: PositionType::Absolute,
                        top: Val::Px(412.),
                        left: Val::Px(520.),
                        ..default()
                    },
                    ..default()
                },
                TextureAtlas {
                    layout: game_texture_atlas.tanks.clone(),
                    index: 0,
                },
                OnStartMenuScreenMultiplayerModeFlag,
            ));
        });
    commands.spawn(AudioBundle {
        source: game_sounds.start_menu.clone() ,
        settings: PlaybackSettings::DESPAWN,
    });

}

pub fn setup_game_over(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    game_sounds: Res<GameSounds>,
) {
    let game_over_texture = asset_server.load("textures/game_over.bmp");
    commands.spawn((
        OnGameOverScreen,
        SpriteBundle {
            texture: game_over_texture,
            transform: Transform::from_translation(Vec3::new(0., -400., SPRITE_GAME_OVER_Z_ORDER)),
            ..default()
        },
    ));
    commands.spawn(AudioBundle {
        source: game_sounds.game_over.clone(),
        settings: PlaybackSettings::DESPAWN,
    });
}

pub fn animate_game_over(
    mut q_game_over: Query<&mut Transform, With<OnGameOverScreen>>,
    mut app_state: ResMut<NextState<AppState>>,
    time: Res<Time>,
    mut stop_secs: Local<f32>,
) {
    for mut transform in &mut q_game_over {
        // 上移game over图片
        if transform.translation.y < 0. {
            transform.translation.y += time.delta_seconds() * 150.;
            *stop_secs = 0.0;
        } else {
            // 停顿1秒后，切换到Start Menu
            *stop_secs += time.delta_seconds();
            if *stop_secs > 1.0 {
                app_state.set(AppState::StartMenu);
            }
        }
    }
}

pub fn start_game(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut app_state: ResMut<NextState<AppState>>,
) {
    if keyboard_input.any_just_pressed([KeyCode::Enter, KeyCode::Space]) {
        info!("Switch app state to playing");
        app_state.set(AppState::Playing);
    }
}

// Helper function to speed up development: allows to skip the first screen
pub fn dev_start_game(
    mut app_state: ResMut<NextState<AppState>>,
    mut multiplayer_mode: ResMut<MultiplayerMode>,
) {
    *multiplayer_mode = MultiplayerMode::TwoPlayers;
    app_state.set(AppState::Playing);
}

pub fn switch_multiplayer_mode(
    mut commands: Commands,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut multiplayer_mode: ResMut<MultiplayerMode>,
    mut q_multiplayer_mode_flag: Query<&mut Style, With<OnStartMenuScreenMultiplayerModeFlag>>,
    game_sounds: Res<GameSounds>,
) {
    if keyboard_input.any_just_pressed([KeyCode::ArrowUp, KeyCode::ArrowDown]) {
        for mut style in &mut q_multiplayer_mode_flag {
            if *multiplayer_mode == MultiplayerMode::SinglePlayer {
                style.top = Val::Px(440.);
                *multiplayer_mode = MultiplayerMode::TwoPlayers;
            } else if *multiplayer_mode == MultiplayerMode::TwoPlayers {
                style.top = Val::Px(412.);
                *multiplayer_mode = MultiplayerMode::SinglePlayer;
            }
            commands.spawn(AudioBundle {
                source: game_sounds.mode_switch.clone(),
                settings: PlaybackSettings::DESPAWN,
            });
        }
    }
}

pub fn pause_game(
    mut commands: Commands,
    mut app_state: ResMut<NextState<AppState>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    game_sounds: Res<GameSounds>,
    mut cold_start: Local<Duration>,
    time: Res<Time>,
) {
    // 增加冷启动防止 pause_game 和 unpause_game 都会收到input，导致Paused<->Playing不断循环 // Add cold start to prevent both pause_game and unpause_game from receiving input, causing Paused<->Playing to loop continuously.
    *cold_start += time.delta();
    if cold_start.as_millis() > 100 && keyboard_input.just_released(KeyCode::Escape) {
        info!("Pause game");
        commands.spawn(AudioBundle {
            source: game_sounds.game_pause.clone(),
            settings: PlaybackSettings::DESPAWN,
        });
        app_state.set(AppState::Paused);
        *cold_start = Duration::ZERO;
    }
}

pub fn unpause_game(
    mut app_state: ResMut<NextState<AppState>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut cold_start: Local<Duration>,
    time: Res<Time>,
) {
    *cold_start += time.delta();
    if cold_start.as_millis() > 100 && keyboard_input.just_released(KeyCode::Escape) {
        info!("Unpause game");
        app_state.set(AppState::Playing);
        *cold_start = Duration::ZERO;
    }
}

pub fn despawn_screen<T: Component>(to_despawn: Query<Entity, With<T>>, mut commands: Commands) {
    for entity in &to_despawn {
        commands.entity(entity).despawn_recursive();
    }
}

pub fn reset_multiplayer_mode(mut multiplayer_mode: ResMut<MultiplayerMode>) {
    *multiplayer_mode = MultiplayerMode::SinglePlayer;
}
