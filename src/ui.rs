use bevy::prelude::*;

use crate::common::AppState;

#[derive(Component)]
pub struct OnStartMenuScreen;
#[derive(Component)]
pub struct OnGameOverMenuScreen;

pub fn setup_start_menu(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    commands
        .spawn((NodeBundle {
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
        });

    let tank_texture_handle = asset_server.load("textures/tank1.bmp");
    let tank_texture_atlas =
        TextureAtlas::from_grid(tank_texture_handle, Vec2::new(28.0, 28.0), 2, 4, None, None);
    let tank_texture_atlas_handle = texture_atlases.add(tank_texture_atlas);

    // TODO 将texture_atlas 其中一个sprite转换成Handle<Image>
    // commands.spawn(ImageBundle {
    // image: tank_texture_atlas_handle.into(),
    // ..default()
    // });
}

pub fn setup_game_over_menu(mut commands: Commands) {}

pub fn start_game(keyboard_input: Res<Input<KeyCode>>, mut app_state: ResMut<State<AppState>>) {
    if keyboard_input.any_just_pressed([KeyCode::Return, KeyCode::Space]) {
        info!("Switch app state to playing");
        app_state.set(AppState::Playing).unwrap();
    }
}

pub fn despawn_screen<T: Component>(to_despawn: Query<Entity, With<T>>, mut commands: Commands) {
    for entity in &to_despawn {
        commands.entity(entity).despawn_recursive();
    }
}
