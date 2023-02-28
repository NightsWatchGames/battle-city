use bevy::prelude::*;

use crate::common::{AppState, GameMode};

#[derive(Component)]
pub struct OnStartMenuScreen;
#[derive(Component)]
pub struct OnStartMenuScreenGameModeFlag;

#[derive(Component)]
pub struct OnGameOverMenuScreen;

pub fn setup_start_menu(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
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
                OnStartMenuScreenGameModeFlag,
            ));
        });
}

pub fn setup_game_over_menu(mut commands: Commands) {}

pub fn start_game(keyboard_input: Res<Input<KeyCode>>, mut app_state: ResMut<State<AppState>>) {
    if keyboard_input.any_just_pressed([KeyCode::Return, KeyCode::Space]) {
        info!("Switch app state to playing");
        app_state.set(AppState::Playing).unwrap();
    }
}

pub fn switch_game_mode(
    keyboard_input: Res<Input<KeyCode>>,
    mut game_mode: ResMut<GameMode>,
    mut q_multiplayer_mode: Query<&mut Style, With<OnStartMenuScreenGameModeFlag>>,
) {
    if keyboard_input.any_just_pressed([KeyCode::Up, KeyCode::Down]) {
        for mut style in &mut q_multiplayer_mode {
            // TODO 待优化
            if style.position.top == Val::Px(412.) {
                style.position.top = Val::Px(440.);
                *game_mode = GameMode::MultiPlayers;
            } else {
                style.position.top = Val::Px(412.);
                *game_mode = GameMode::SinglePlayer;
            }
        }
    }
}

pub fn despawn_screen<T: Component>(to_despawn: Query<Entity, With<T>>, mut commands: Commands) {
    for entity in &to_despawn {
        commands.entity(entity).despawn_recursive();
    }
}
