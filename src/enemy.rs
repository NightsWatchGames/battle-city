use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::{
    common::{self, AnimationTimer},
    player::{TankRefreshBulletTimer, TANK_REFRESH_BULLET_INTERVAL},
};

// 当前关卡生成的敌人数量
#[derive(Resource)]
pub struct LevelSpawnedEnemies(pub i32);

#[derive(Component)]
pub struct Enemy;

pub fn auto_spawn_enemies(
    q_enemies: Query<&Enemy>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    mut spawned_enemies: ResMut<LevelSpawnedEnemies>,
) {
    if q_enemies.into_iter().len() < 1 {
        spawn_enemy(&mut commands, &asset_server, &mut texture_atlases);
        spawned_enemies.0 += 1;
    }
}

pub fn spawn_enemy(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    texture_atlases: &mut ResMut<Assets<TextureAtlas>>,
) {
    let tank_texture_handle = asset_server.load("textures/enemies.bmp");
    let tank_texture_atlas =
        TextureAtlas::from_grid(tank_texture_handle, Vec2::new(28.0, 28.0), 2, 4, None, None);
    let tank_texture_atlas_handle = texture_atlases.add(tank_texture_atlas);

    commands.spawn((
        Enemy,
        SpriteSheetBundle {
            texture_atlas: tank_texture_atlas_handle,
            // TODO 随机地点
            transform: Transform {
                translation: Vec3::new(0.0, 150.0, 1.0),
                ..default()
            },
            ..default()
        },
        TankRefreshBulletTimer(Timer::from_seconds(
            TANK_REFRESH_BULLET_INTERVAL,
            TimerMode::Once,
        )),
        AnimationTimer(Timer::from_seconds(0.2, TimerMode::Repeating)),
        common::Direction::Up,
        RigidBody::Dynamic,
        Collider::cuboid(18.0, 18.0),
        ActiveEvents::COLLISION_EVENTS,
        LockedAxes::ROTATION_LOCKED,
    ));
}