mod bullet;
mod collision;
mod common;
mod tank;
mod wall;

use bullet::*;
use collision::*;
use common::*;
use tank::*;
use wall::*;

use bevy::{prelude::*, time::FixedTimestep};

const BACKGROUND_COLOR: Color = Color::BLACK;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(ClearColor(BACKGROUND_COLOR))
        .add_startup_system(setup)
        .add_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::step(TIME_STEP as f64))
                .with_system(tank_move_system)
                .with_system(bullet_move_system),
        )
        .add_system(tank_attack_system)
        .add_system(tank_animate_system)
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
    commands.spawn(Camera2dBundle::default());

    let shield_texture_handle = asset_server.load("textures/shield.bmp");
    let shield_texture_atlas = TextureAtlas::from_grid(
        shield_texture_handle,
        Vec2::new(30.0, 30.0),
        1,
        2,
        None,
        None,
    );
    let shield_texture_atlas_handle = texture_atlases.add(shield_texture_atlas);

    let tank_texture_handle = asset_server.load("textures/tank1.bmp");
    let tank_texture_atlas =
        TextureAtlas::from_grid(tank_texture_handle, Vec2::new(28.0, 28.0), 2, 4, None, None);
    let tank_texture_atlas_handle = texture_atlases.add(tank_texture_atlas);

    // 保护盾
    let shield = commands
        .spawn(Shield)
        .insert(SpriteSheetBundle {
            texture_atlas: shield_texture_atlas_handle,
            ..default()
        })
        .insert(AnimationTimer(Timer::from_seconds(
            0.2,
            TimerMode::Repeating,
        )))
        .insert(ShieldRemoveTimer(Timer::from_seconds(5.0, TimerMode::Once)))
        .id();

    // 坦克
    let tank = commands
        .spawn(Tank)
        .insert(SpriteSheetBundle {
            texture_atlas: tank_texture_atlas_handle,
            transform: Transform {
                translation: Vec3::new(0.0, BOTTOM_WALL + 100.0, 0.0),
                ..default()
            },
            ..default()
        })
        .insert(AnimationTimer(Timer::from_seconds(
            0.2,
            TimerMode::Repeating,
        )))
        .insert(TankRefreshBulletTimer(Timer::from_seconds(
            TANK_REFRESH_BULLET_INTERVAL,
            TimerMode::Once,
        )))
        .insert(Collider)
        .insert(common::Direction::Up)
        .id();

    commands.entity(tank).add_child(shield);

    // 墙壁
    commands.spawn(WallBundle::new(WallLocation::Left));
    commands.spawn(WallBundle::new(WallLocation::Right));
    commands.spawn(WallBundle::new(WallLocation::Bottom));
    commands.spawn(WallBundle::new(WallLocation::Top));
}
