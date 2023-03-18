use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use rand::Rng;

use crate::{
    common::{
        self, AnimationIndices, AnimationTimer, GameTextureAtlasHandles, TankRefreshBulletTimer,
        ENEMIES_PER_LEVEL, MAX_LIVE_ENEMIES, TANK_REFRESH_BULLET_INTERVAL, TANK_SCALE, TANK_SIZE,
        TILE_SIZE,
    },
    level::EnemiesMarker,
    player::PlayerNo,
};

// 当前关卡生成的敌人数量
#[derive(Resource)]
pub struct LevelSpawnedEnemies(pub i32);

#[derive(Component)]
pub struct Enemy;

pub fn auto_spawn_enemies(
    mut commands: Commands,
    mut level_spawned_enemies: ResMut<LevelSpawnedEnemies>,
    q_enemies: Query<&Transform, With<Enemy>>,
    q_enemies_marker: Query<&GlobalTransform, With<EnemiesMarker>>,
    q_players: Query<&Transform, With<PlayerNo>>,
    game_texture_atlas: Res<GameTextureAtlasHandles>,
) {
    if q_enemies.into_iter().len() >= MAX_LIVE_ENEMIES as usize {
        // 战场上存活敌人已达到最大值
        return;
    }
    if level_spawned_enemies.0 == ENEMIES_PER_LEVEL {
        // 本关卡已生成敌人数量达最大值
        return;
    }
    let mut marker_positions = Vec::new();
    for enemy_marker in &q_enemies_marker {
        // 防止player1_marker还未初始化
        if enemy_marker.translation() == Vec3::ZERO {
            continue;
        }
        marker_positions.push(enemy_marker.clone());
    }

    if marker_positions.len() > 0 {
        // 随机地点
        let mut rng = rand::thread_rng();
        let choosed_pos = marker_positions
            .get(rng.gen_range(0..marker_positions.len()))
            .unwrap()
            .translation();

        // 不能距离战场坦克过近
        for enemy_pos in &q_enemies {
            if choosed_pos.distance(enemy_pos.translation) < 2. * TILE_SIZE {
                return;
            }
        }
        for player_pos in &q_players {
            if choosed_pos.distance(player_pos.translation) < 2. * TILE_SIZE {
                return;
            }
        }
        spawn_enemy(choosed_pos, &mut commands, &game_texture_atlas);
        level_spawned_enemies.0 += 1;
    }
}

pub fn spawn_enemy(
    pos: Vec3,
    commands: &mut Commands,
    game_texture_atlas: &Res<GameTextureAtlasHandles>,
) {
    // 随机颜色
    let indexes = vec![0, 2, 4, 6, 32, 34, 36, 38];
    let mut rng = rand::thread_rng();
    let choosed_index = indexes
        .get(rng.gen_range(0..indexes.len()))
        .unwrap()
        .clone();

    commands.spawn((
        Enemy,
        SpriteSheetBundle {
            sprite: TextureAtlasSprite {
                index: choosed_index as usize,
                ..default()
            },
            texture_atlas: game_texture_atlas.enemies.clone(),
            transform: Transform {
                translation: pos,
                scale: Vec3::splat(TANK_SCALE),
                ..default()
            },
            ..default()
        },
        TankRefreshBulletTimer(Timer::from_seconds(
            TANK_REFRESH_BULLET_INTERVAL,
            TimerMode::Once,
        )),
        AnimationTimer(Timer::from_seconds(0.2, TimerMode::Repeating)),
        AnimationIndices {
            first: choosed_index,
            last: choosed_index + 1,
        },
        common::Direction::Up,
        RigidBody::Dynamic,
        Collider::cuboid(TANK_SIZE * TANK_SCALE / 2.0, TANK_SIZE * TANK_SCALE / 2.0),
        ActiveEvents::COLLISION_EVENTS,
        LockedAxes::ROTATION_LOCKED,
    ));
}

// 坦克移动动画播放
pub fn animate_enemies(
    time: Res<Time>,
    mut query: Query<
        (
            &mut AnimationTimer,
            &AnimationIndices,
            &mut TextureAtlasSprite,
        ),
        With<Enemy>,
    >,
) {
    for (mut timer, indices, mut sprite) in &mut query {
        timer.0.tick(time.delta());
        if timer.0.just_finished() {
            // 切换到下一个sprite
            sprite.index = if sprite.index == indices.last {
                indices.first
            } else {
                sprite.index + 1
            };
        }
    }
}

pub fn cleanup_enemies(mut commands: Commands, q_enemies: Query<Entity, With<Enemy>>) {
    for entity in &q_enemies {
        commands.entity(entity).despawn_recursive();
    }
}

pub fn reset_level_spawned_enemies(mut level_spawned_enemies: ResMut<LevelSpawnedEnemies>) {
    level_spawned_enemies.0 = 0;
}
