use std::time::Duration;

use bevy::prelude::*;

use crate::common::{AppState, GameSounds, MultiplayerMode, SPRITE_GAME_OVER_ORDER};

#[derive(Component)]
pub struct OnStartMenuScreen;
#[derive(Component)]
pub struct OnStartMenuScreenMultiplayerModeFlag;

#[derive(Component)]
pub struct OnGameOverScreen;

pub fn setup_start_menu(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    audio: Res<Audio>,
    game_sounds: Res<GameSounds>,
) {
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    size: Size::new(Val::Percent(100.), Val::Percent(100.)),
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
            // TODO 将texture_atlas 直接用于ui，issue https://github.com/bevyengine/bevy/issues/1169
            parent.spawn((
                ImageBundle {
                    image: asset_server.load("textures/tank.png").into(),
                    style: Style {
                        size: Size::new(Val::Px(20.), Val::Px(20.)),
                        margin: UiRect::all(Val::Px(10.0)),
                        position_type: PositionType::Absolute,
                        position: UiRect {
                            top: Val::Px(412.),
                            left: Val::Px(520.),
                            ..default()
                        },
                        ..default()
                    },
                    ..default()
                },
                OnStartMenuScreenMultiplayerModeFlag,
            ));
        });
    audio.play(game_sounds.start_menu.clone());
}

pub fn setup_game_over(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    audio: Res<Audio>,
    game_sounds: Res<GameSounds>,
) {
    let game_over_texture = asset_server.load("textures/game_over.bmp");
    commands.spawn((
        OnGameOverScreen,
        SpriteBundle {
            texture: game_over_texture,
            transform: Transform::from_translation(Vec3::new(0., -400., SPRITE_GAME_OVER_ORDER)),
            ..default()
        },
    ));
    audio.play(game_sounds.game_over.clone());
}

pub fn animate_game_over(
    mut q_game_over: Query<&mut Transform, With<OnGameOverScreen>>,
    mut app_state: ResMut<State<AppState>>,
    time: Res<Time>,
) {
    for mut transform in &mut q_game_over {
        // 上移game over图片
        if transform.translation.y < 0. {
            transform.translation.y += time.delta_seconds() * 150.
        } else {
            // 切换到Start Menu
            app_state.set(AppState::StartMenu);
        }
    }
}

pub fn start_game(keyboard_input: Res<Input<KeyCode>>, mut app_state: ResMut<State<AppState>>) {
    if keyboard_input.any_just_pressed([KeyCode::Return, KeyCode::Space]) {
        info!("Switch app state to playing");
        app_state.set(AppState::Playing).unwrap();
    }
}

pub fn switch_multiplayer_mode(
    keyboard_input: Res<Input<KeyCode>>,
    mut multiplayer_mode: ResMut<MultiplayerMode>,
    mut q_multiplayer_mode: Query<&mut Style, With<OnStartMenuScreenMultiplayerModeFlag>>,
    audio: Res<Audio>,
    game_sounds: Res<GameSounds>,
) {
    if keyboard_input.any_just_pressed([KeyCode::Up, KeyCode::Down]) {
        for mut style in &mut q_multiplayer_mode {
            // TODO 待优化
            if style.position.top == Val::Px(412.) {
                style.position.top = Val::Px(440.);
                *multiplayer_mode = MultiplayerMode::TwoPlayers;
            } else {
                style.position.top = Val::Px(412.);
                *multiplayer_mode = MultiplayerMode::SinglePlayer;
            }
            audio.play(game_sounds.mode_switch.clone());
        }
    }
}

pub fn pause_game(
    mut app_state: ResMut<State<AppState>>,
    keyboard_input: Res<Input<KeyCode>>,
    audio: Res<Audio>,
    game_sounds: Res<GameSounds>,
    mut cold_start: Local<Duration>,
    time: Res<Time>,
) {
    // 增加冷启动防止 pause_game 和 unpause_game 都会收到input，导致Paued<->Playing不断循环
    *cold_start += time.delta();
    if cold_start.as_millis() > 100 {
        if keyboard_input.just_released(KeyCode::Escape) {
            info!("Pause game");
            audio.play(game_sounds.game_pause.clone());
            app_state.set(AppState::Paused);
            *cold_start = Duration::ZERO;
        }
    }
}

pub fn unpause_game(
    mut app_state: ResMut<State<AppState>>,
    keyboard_input: Res<Input<KeyCode>>,
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
