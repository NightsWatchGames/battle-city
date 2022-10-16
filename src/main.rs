mod collision;
mod common;
mod tank;
mod wall;

use collision::*;
use common::*;
use tank::*;
use wall::*;

use bevy::prelude::*;

const BACKGROUND_COLOR: Color = Color::BLACK;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(ClearColor(BACKGROUND_COLOR))
        .add_startup_system(setup)
        .add_system(move_tank_system)
        .add_system(shield_animate_system)
        .add_system(shield_remove_system)
        .add_system(bevy::window::close_on_esc)
        .run();
}

// setup系统 添加entities到世界
fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    // 相机
    commands.spawn_bundle(Camera2dBundle::default());

    let shield_texture_handle = asset_server.load("textures/shield.bmp");
    let shield_texture_atlas =
        TextureAtlas::from_grid(shield_texture_handle, Vec2::new(30.0, 30.0), 1, 2);
    let shield_texture_atlas_handle = texture_atlases.add(shield_texture_atlas);

    // 保护盾
    let shield = commands
        .spawn_bundle(SpriteSheetBundle {
            texture_atlas: shield_texture_atlas_handle,
            transform: Transform::from_scale(Vec3::splat(3.0)),
            ..default()
        })
        .insert(Shield)
        .insert(AnimationTimer(Timer::from_seconds(0.2, true)))
        .insert(ShieldRemoveTimer(Timer::from_seconds(5.0, false)))
        .id();

    // 坦克
    let tank = commands
        .spawn()
        .insert(Tank)
        .insert_bundle(SpriteBundle {
            transform: Transform {
                translation: Vec3::new(0.0, BOTTOM_WALL + 100.0, 0.0),
                ..default()
            },
            sprite: Sprite {
                custom_size: Some(TANK_SIZE),
                ..default()
            },
            texture: asset_server.load("textures/tank.png"),
            ..default()
        })
        .insert(Collider)
        .id();

    commands.entity(tank).add_child(shield);

    // 墙壁
    commands.spawn_bundle(WallBundle::new(WallLocation::Left));
    commands.spawn_bundle(WallBundle::new(WallLocation::Right));
    commands.spawn_bundle(WallBundle::new(WallLocation::Bottom));
    commands.spawn_bundle(WallBundle::new(WallLocation::Top));
}