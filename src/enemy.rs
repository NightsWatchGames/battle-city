use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use rand::Rng;

use crate::{
    bullet::{spawn_bullet, Bullet},
    common::{
        self, AnimationIndices, AnimationTimer, TankRefreshBulletTimer, ENEMIES_PER_LEVEL,
        ENEMY_REFRESH_BULLET_INTERVAL, ENEMY_SPEED, MAX_LIVE_ENEMIES, TANK_SCALE, TANK_SIZE,
        TILE_SIZE,
    },
    level::{EnemiesMarker, LevelItem},
    player::PlayerNo,
};

// 当前关卡生成的敌人数量
#[derive(Resource)]
pub struct LevelSpawnedEnemies(pub i32);

#[derive(Component)]
pub struct Enemy;

// 转向计时器
#[derive(Component)]
pub struct EnemyChangeDirectionTimer(pub Timer);

pub fn auto_spawn_enemies(
    mut commands: Commands,
    mut level_spawned_enemies: ResMut<LevelSpawnedEnemies>,
    q_enemies: Query<&Transform, With<Enemy>>,
    q_enemies_marker: Query<&GlobalTransform, With<EnemiesMarker>>,
    q_players: Query<&Transform, With<PlayerNo>>,
    asset_server: Res<AssetServer>,
    mut atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
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
        spawn_enemy(
            choosed_pos,
            &mut commands,
            &asset_server,
            &mut atlas_layouts,
        );
        level_spawned_enemies.0 += 1;
    }
}

pub fn spawn_enemy(
    pos: Vec3,
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    atlas_layouts: &mut ResMut<Assets<TextureAtlasLayout>>,
) {
    let enemies_texture_handle = asset_server.load("textures/enemies.bmp");
    let enemies_texture_atlas =
        TextureAtlasLayout::from_grid(UVec2::new(TANK_SIZE, TANK_SIZE), 8, 8, None, None);
    let enemies_atlas_layout_handle = atlas_layouts.add(enemies_texture_atlas);

    // 随机颜色
    let indexes: Vec<i32> = enemies_sprite_index_sets()
        .iter()
        .map(|v| *v.get(0).unwrap())
        .collect();
    let mut rng = rand::thread_rng();
    let choosed_index = indexes
        .get(rng.gen_range(0..indexes.len()))
        .unwrap()
        .clone();

    commands.spawn((
        Enemy,
        Sprite {
            image: enemies_texture_handle,
            texture_atlas: Some(TextureAtlas {
                layout: enemies_atlas_layout_handle,
                index: choosed_index as usize,
            }),
            ..default()
        },
        Transform {
            translation: pos,
            scale: Vec3::splat(TANK_SCALE),
            ..default()
        },
        TankRefreshBulletTimer(Timer::from_seconds(
            ENEMY_REFRESH_BULLET_INTERVAL,
            TimerMode::Repeating,
        )),
        EnemyChangeDirectionTimer(Timer::from_seconds(1.0, TimerMode::Once)),
        AnimationTimer(Timer::from_seconds(0.2, TimerMode::Repeating)),
        AnimationIndices {
            first: choosed_index as usize,
            last: choosed_index as usize + 1,
        },
        common::Direction::Up,
        RigidBody::Dynamic,
        Collider::cuboid(
            TANK_SIZE as f32 * TANK_SCALE / 2.0,
            TANK_SIZE as f32 * TANK_SCALE / 2.0,
        ),
        ActiveEvents::COLLISION_EVENTS,
        LockedAxes::ROTATION_LOCKED,
    ));
}

// TODO 发现玩家后主动攻击
// TODO 树林可躲藏
pub fn enemies_move(
    mut q_enemies: Query<
        (
            &mut Transform,
            &mut common::Direction,
            &mut Sprite,
            &mut AnimationIndices,
            &mut EnemyChangeDirectionTimer,
        ),
        With<Enemy>,
    >,
    q_level_items: Query<(&LevelItem, &GlobalTransform)>,
    time: Res<Time>,
) {
    for (mut transform, mut direction, mut sprite, mut indices, mut timer) in &mut q_enemies {
        timer.0.tick(time.delta());
        if !timer.0.finished() {
            match *direction {
                common::Direction::Up => {
                    transform.translation.y += ENEMY_SPEED * time.delta_secs();
                }
                common::Direction::Right => {
                    transform.translation.x += ENEMY_SPEED * time.delta_secs();
                }
                common::Direction::Down => {
                    transform.translation.y -= ENEMY_SPEED * time.delta_secs();
                }
                common::Direction::Left => {
                    transform.translation.x -= ENEMY_SPEED * time.delta_secs();
                }
            }
            continue;
        }

        // 重新选择方向
        let mut can_left = true;
        let mut can_right = true;
        let mut can_up = true;
        let mut can_down = true;

        // 当前可走路径
        for (level_item, level_item_transform) in &q_level_items {
            if *level_item == LevelItem::Tree {
                continue;
            }
            if (level_item_transform.translation().x - transform.translation.x).abs()
                < (TANK_SIZE as f32 + TILE_SIZE) / 2.0 - 5.0
            {
                if level_item_transform.translation().y > transform.translation.y
                    && level_item_transform.translation().y - transform.translation.y < TILE_SIZE
                {
                    can_up = false;
                }
                if level_item_transform.translation().y < transform.translation.y
                    && transform.translation.y - level_item_transform.translation().y < TILE_SIZE
                {
                    can_down = false;
                }
            }
            if (level_item_transform.translation().y - transform.translation.y).abs()
                < (TANK_SIZE as f32 + TILE_SIZE) / 2. - 5.0
            {
                if level_item_transform.translation().x > transform.translation.x
                    && level_item_transform.translation().x - transform.translation.x < TILE_SIZE
                {
                    can_right = false;
                }
                if level_item_transform.translation().x < transform.translation.x
                    && transform.translation.x - level_item_transform.translation().x < TILE_SIZE
                {
                    can_left = false;
                }
            }
        }
        if !can_left && !can_right && !can_up && !can_down {
            continue;
        }

        // 根据权重随机一个方向
        let mut rng = rand::thread_rng();
        let choosed_direction = loop {
            let rand = rng.gen_range(0..9);
            match rand {
                0 => {
                    if can_up {
                        break common::Direction::Up;
                    }
                }
                1 | 2 => {
                    if can_left {
                        break common::Direction::Left;
                    }
                }
                3 | 4 => {
                    if can_right {
                        break common::Direction::Right;
                    }
                }
                5 | 6 | 7 | 8 => {
                    if can_down {
                        break common::Direction::Down;
                    }
                }
                _ => {}
            }
        };

        // 设置方向和sprite
        *direction = choosed_direction;
        if let Some(atlas) = &mut sprite.texture_atlas {
            atlas.index = new_sprite_index(atlas.index as i32, *direction) as usize;
            *indices = AnimationIndices {
                first: atlas.index,
                last: atlas.index + 1,
            };
        }

        // 重置转向计时器
        timer.0.reset();
    }
}

pub fn enemies_attack(
    mut q_players: Query<
        (&Transform, &common::Direction, &mut TankRefreshBulletTimer),
        With<Enemy>,
    >,
    time: Res<Time>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    for (transform, direction, mut refresh_bullet_timer) in &mut q_players {
        refresh_bullet_timer.tick(time.delta());
        if refresh_bullet_timer.just_finished() {
            spawn_bullet(
                &mut commands,
                &asset_server,
                &mut atlas_layouts,
                Bullet::Enemy,
                transform.translation,
                direction.clone(),
            );
        }
    }
}

pub fn handle_enemy_collision(
    mut q_enemies: Query<&mut EnemyChangeDirectionTimer, With<Enemy>>,
    mut collision_er: EventReader<CollisionEvent>,
) {
    for event in collision_er.read() {
        match event {
            CollisionEvent::Started(entity1, entity2, _flags)
            | CollisionEvent::Stopped(entity1, entity2, _flags) => {
                let enemy_entity = if q_enemies.contains(*entity1) {
                    *entity1
                } else if q_enemies.contains(*entity2) {
                    *entity2
                } else {
                    continue;
                };

                // 重置转向计时器
                let mut change_direction_timer = q_enemies.get_mut(enemy_entity).unwrap();
                change_direction_timer.0.reset();
            }
        }
    }
}

// 坦克移动动画播放
pub fn animate_enemies(
    time: Res<Time>,
    mut query: Query<(&mut AnimationTimer, &AnimationIndices, &mut Sprite), With<Enemy>>,
) {
    for (mut timer, indices, mut sprite) in &mut query {
        timer.0.tick(time.delta());
        if timer.0.just_finished() {
            // 切换到下一个sprite
            if let Some(atlas) = &mut sprite.texture_atlas {
                atlas.index = if atlas.index == indices.last {
                    indices.first
                } else {
                    atlas.index + 1
                };
            }
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

pub fn enemies_sprite_index_sets() -> Vec<Vec<i32>> {
    vec![
        // 上右下左 + 其他可能index
        vec![0, 8, 16, 24, 1, 9, 17, 25],
        vec![2, 10, 18, 26, 3, 11, 19, 27],
        vec![4, 12, 20, 28, 5, 13, 21, 29],
        vec![6, 14, 22, 30, 7, 15, 23, 31],
        vec![32, 40, 48, 56, 33, 41, 49, 57],
        vec![34, 42, 50, 58, 35, 43, 51, 59],
        vec![36, 44, 52, 60, 37, 45, 53, 61],
        vec![38, 46, 54, 62, 39, 47, 55, 63],
    ]
}
pub fn new_sprite_index(current_index: i32, direction: common::Direction) -> i32 {
    let index_sets = enemies_sprite_index_sets();
    for index_set in index_sets {
        if index_set.contains(&current_index) {
            info!("found index_set");
            match direction {
                common::Direction::Up => {
                    return *index_set.get(0).unwrap();
                }
                common::Direction::Right => {
                    return *index_set.get(1).unwrap();
                }
                common::Direction::Down => {
                    return *index_set.get(2).unwrap();
                }
                common::Direction::Left => {
                    return *index_set.get(3).unwrap();
                }
            }
        }
    }
    return 0;
}
