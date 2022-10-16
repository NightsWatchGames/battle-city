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
        .add_system(animate_shield_system)
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

    // 坦克
    let tank_y = BOTTOM_WALL + 100.0;
    commands
        .spawn()
        .insert(Tank)
        .insert_bundle(SpriteBundle {
            transform: Transform {
                translation: Vec3::new(0.0, tank_y, 0.0),
                ..default()
            },
            sprite: Sprite {
                custom_size: Some(TANK_SIZE),
                ..default()
            },
            texture: asset_server.load("textures/tank.png"),
            ..default()
        })
        .insert(Collider);

    // 墙壁
    commands.spawn_bundle(WallBundle::new(WallLocation::Left));
    commands.spawn_bundle(WallBundle::new(WallLocation::Right));
    commands.spawn_bundle(WallBundle::new(WallLocation::Bottom));
    commands.spawn_bundle(WallBundle::new(WallLocation::Top));

    let texture_handle = asset_server.load("textures/shield.bmp");
    let texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::new(30.0, 30.0), 1, 2);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);
    commands
        .spawn_bundle(SpriteSheetBundle {
            texture_atlas: texture_atlas_handle,
            transform: Transform::from_scale(Vec3::splat(6.0)),
            ..default()
        })
        .insert(Shield)
        .insert(AnimationTimer(Timer::from_seconds(0.2, true)));
        
}