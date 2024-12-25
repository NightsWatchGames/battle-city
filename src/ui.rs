use crate::common::{AppState, GameSounds, MultiplayerMode, SPRITE_GAME_OVER_ORDER, TANK_SIZE};
use bevy::prelude::*;
use std::time::Duration;

#[derive(Component)]
pub struct OnStartMenuScreen;
#[derive(Component)]
pub struct OnStartMenuScreenMultiplayerModeFlag;

#[derive(Component)]
pub struct OnGameOverScreen;

pub fn setup_start_menu(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    let player1_texture_handle = asset_server.load("textures/tank1.bmp");
    let player1_texture_atlas =
        TextureAtlasLayout::from_grid(UVec2::new(TANK_SIZE, TANK_SIZE), 8, 4, None, None);
    let player1_atlas_layout_handle = atlas_layouts.add(player1_texture_atlas);

    commands
        .spawn((
            Node {
                width: Val::Percent(100.),
                height: Val::Percent(100.),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..default()
            },
            OnStartMenuScreen,
        ))
        .with_children(|parent| {
            parent.spawn(ImageNode {
                image: asset_server.load("textures/title.bmp").into(),
                ..default()
            });
            parent.spawn((
                Node {
                    width: Val::Px(20.),
                    height: Val::Px(20.),
                    margin: UiRect::all(Val::Px(10.0)),
                    position_type: PositionType::Absolute,
                    top: Val::Px(412.),
                    left: Val::Px(520.),
                    ..default()
                },
                ImageNode {
                    image: player1_texture_handle,
                    texture_atlas: Some(TextureAtlas {
                        index: 0,
                        layout: player1_atlas_layout_handle,
                    }),
                    ..default()
                },
                OnStartMenuScreenMultiplayerModeFlag,
            ));
        });
    commands.spawn((
        AudioPlayer::new(asset_server.load("sounds/start_menu.ogg")),
        PlaybackSettings::DESPAWN,
    ));
}

pub fn setup_game_over(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    game_sounds: Res<GameSounds>,
) {
    let game_over_texture = asset_server.load("textures/game_over.bmp");
    commands.spawn((
        Sprite {
            image: game_over_texture,
            ..default()
        },
        Transform::from_translation(Vec3::new(0., -400., SPRITE_GAME_OVER_ORDER)),
        OnGameOverScreen,
    ));
    commands.spawn((
        AudioPlayer(game_sounds.game_over.clone()),
        PlaybackSettings::DESPAWN,
    ));
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
            transform.translation.y += time.delta_secs() * 150.;
            *stop_secs = 0.0;
        } else {
            // 停顿1秒后，切换到Start Menu
            *stop_secs += time.delta_secs();
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

pub fn switch_multiplayer_mode(
    mut commands: Commands,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut multiplayer_mode: ResMut<MultiplayerMode>,
    mut q_multiplayer_mode_flag: Query<&mut Node, With<OnStartMenuScreenMultiplayerModeFlag>>,
    game_sounds: Res<GameSounds>,
) {
    if keyboard_input.any_just_pressed([KeyCode::ArrowUp, KeyCode::ArrowDown]) {
        for mut node in &mut q_multiplayer_mode_flag {
            if *multiplayer_mode == MultiplayerMode::SinglePlayer {
                node.top = Val::Px(440.);
                *multiplayer_mode = MultiplayerMode::TwoPlayers;
            } else if *multiplayer_mode == MultiplayerMode::TwoPlayers {
                node.top = Val::Px(412.);
                *multiplayer_mode = MultiplayerMode::SinglePlayer;
            }
            commands.spawn((
                AudioPlayer(game_sounds.mode_switch.clone()),
                PlaybackSettings::DESPAWN,
            ));
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
    // 增加冷启动防止 pause_game 和 unpause_game 都会收到input，导致Paued<->Playing不断循环
    *cold_start += time.delta();
    if cold_start.as_millis() > 100 {
        if keyboard_input.just_released(KeyCode::Escape) {
            info!("Pause game");
            commands.spawn((
                AudioPlayer(game_sounds.game_pause.clone()),
                PlaybackSettings::DESPAWN,
            ));
            app_state.set(AppState::Paused);
            *cold_start = Duration::ZERO;
        }
    }
}

pub fn unpause_game(
    mut app_state: ResMut<NextState<AppState>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut cold_start: Local<Duration>,
    time: Res<Time>,
) {
    *cold_start += time.delta();
    if cold_start.as_millis() > 100 {
        if keyboard_input.just_released(KeyCode::Escape) {
            info!("Unpause game");
            app_state.set(AppState::Playing);
            *cold_start = Duration::ZERO;
        }
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
