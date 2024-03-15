use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use rand::Rng;

use crate::{
    bullet::{spawn_bullet, Bullet},
    common::{
        self, AnimationIndices, AnimationTimer, GameTextureAtlasHandles, TankRefreshBulletTimer,
        ENEMIES_PER_LEVEL, ENEMY_REFRESH_BULLET_INTERVAL, ENEMY_SPEED, MAX_LIVE_ENEMIES,
        TANK_SCALE, TANK_SIZE, TILE_SIZE, TANKS_SPRITE_COLS_AMOUNT, TANK_ROUND_CORNERS, PHYSICS_SCALE_PER_METER
    },
    level::{EnemiesMarker, LevelItem},
    player::PlayerNo,
};

// 当前关卡生成的敌人数量 // The number of enemies spawned in the current level
#[derive(Resource)]
pub struct LevelSpawnedEnemies(pub i32);

#[derive(Component)]
pub struct Enemy;

// 转向计时器 // Change direction timer
#[derive(Component)]
pub struct EnemyChangeDirectionTimer(pub Timer);

pub fn auto_spawn_enemies(
    mut commands: Commands,
    mut level_spawned_enemies: ResMut<LevelSpawnedEnemies>,
    q_enemies: Query<&Transform, With<Enemy>>,
    q_enemies_marker: Query<&GlobalTransform, With<EnemiesMarker>>,
    q_players: Query<&Transform, With<PlayerNo>>,
    game_texture_atlas: Res<GameTextureAtlasHandles>,
) {
    if q_enemies.into_iter().len() >= MAX_LIVE_ENEMIES as usize {
        // 战场上存活敌人已达到最大值 // The number of surviving enemies on the battlefield has reached the maximum value
        return;
    }
    if level_spawned_enemies.0 == ENEMIES_PER_LEVEL {
        // 本关卡已生成敌人数量达最大值 // The maximum number of enemies has been generated in this level
        return;
    }
    let mut marker_positions = Vec::new();
    for enemy_marker in &q_enemies_marker {
        // 防止player1_marker还未初始化 // Prevent player1_marker from not being initialized yet
        if enemy_marker.translation() == Vec3::ZERO {
            continue;
        }
        marker_positions.push(enemy_marker.clone());
    }

    if marker_positions.len() > 0 {
        // 随机地点 // Random location
        let mut rng = rand::thread_rng();
        let choosed_pos = marker_positions
            .get(rng.gen_range(0..marker_positions.len()))
            .unwrap()
            .translation();

        // 不能距离战场坦克过近 // Don’t get too close to tanks on the battlefield
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
    // 随机颜色 // Random type
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
        SpriteSheetBundle {
            sprite: TextureAtlasSprite {
                index: choosed_index as usize,
                ..default()
            },
            texture_atlas: game_texture_atlas.tanks.clone(),
            transform: Transform {
                translation: pos,
                scale: Vec3::splat(TANK_SCALE),
                ..default()
            },
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
        Collider::round_cuboid((TANK_SIZE * TANK_SCALE / 2.0) - TANK_ROUND_CORNERS, (TANK_SIZE * TANK_SCALE / 2.0) - TANK_ROUND_CORNERS, TANK_ROUND_CORNERS / PHYSICS_SCALE_PER_METER),
        LockedAxes::ROTATION_LOCKED,
    ));
}

// TODO 发现玩家后主动攻击 // TODO: Actively attack after discovering the player
pub fn enemies_move(
    mut q_enemies: Query<
        (
            &mut Transform,
            &mut common::Direction,
            &mut TextureAtlasSprite,
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
                    transform.translation.y += ENEMY_SPEED * time.delta_seconds();
                }
                common::Direction::Right => {
                    transform.translation.x += ENEMY_SPEED * time.delta_seconds();
                }
                common::Direction::Down => {
                    transform.translation.y -= ENEMY_SPEED * time.delta_seconds();
                }
                common::Direction::Left => {
                    transform.translation.x -= ENEMY_SPEED * time.delta_seconds();
                }
            }
            continue;
        }

        // 重新选择方向 // Re-select direction
        let mut can_left = true;
        let mut can_right = true;
        let mut can_up = true;
        let mut can_down = true;

        // 当前可走路径 // Current path available
        for (level_item, level_item_transform) in &q_level_items {
            if *level_item == LevelItem::Tree {
                continue;
            }
            if (level_item_transform.translation().x - transform.translation.x).abs()
                < (TANK_SIZE + TILE_SIZE) / 2.0 - 5.0
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
                < (TANK_SIZE + TILE_SIZE) / 2. - 5.0
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

        // 根据权重随机一个方向 // Randomly move in a direction based on weight
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

        // 设置方向和sprite // Set direction and sprite
        *direction = choosed_direction;
        sprite.index = new_sprite_index(sprite.index as i32, *direction) as usize;
        *indices = AnimationIndices {
            first: sprite.index,
            last: sprite.index + 1,
        };

        // 重置转向计时器 // Reset turn timer
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
    game_texture_atlas: Res<GameTextureAtlasHandles>,
) {
    for (transform, direction, mut refresh_bullet_timer) in &mut q_players {
        refresh_bullet_timer.tick(time.delta());
        if refresh_bullet_timer.just_finished() {
            spawn_bullet(
                &mut commands,
                &game_texture_atlas,
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

                // 重置转向计时器 // Reset turn timer
                let mut change_direction_timer = q_enemies
                    .get_component_mut::<EnemyChangeDirectionTimer>(enemy_entity)
                    .unwrap();
                change_direction_timer.0.reset();
            }
        }
    }
}

// 坦克移动动画播放 // Tank moving animation playback
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
            // 切换到下一个sprite // Switch to next sprite
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

// TODO: Refactor it.
pub fn enemies_sprite_index_sets() -> Vec<Vec<i32>> {
    vec![
        // 上右下左 + 其他可能index // Columns: top, right, bottom, left + the same postitions for movement
        vec![
            8 + TANKS_SPRITE_COLS_AMOUNT * 0,
            14 + TANKS_SPRITE_COLS_AMOUNT * 0,
            12 + TANKS_SPRITE_COLS_AMOUNT * 0,
            10 + TANKS_SPRITE_COLS_AMOUNT * 0,
            9 + TANKS_SPRITE_COLS_AMOUNT * 0,
            15 + TANKS_SPRITE_COLS_AMOUNT * 0,
            13 + TANKS_SPRITE_COLS_AMOUNT * 0,
            11 + TANKS_SPRITE_COLS_AMOUNT * 0
        ],
        vec![
            8 + TANKS_SPRITE_COLS_AMOUNT * 1,
            14 + TANKS_SPRITE_COLS_AMOUNT * 1,
            12 + TANKS_SPRITE_COLS_AMOUNT * 1,
            10 + TANKS_SPRITE_COLS_AMOUNT * 1,
            9 + TANKS_SPRITE_COLS_AMOUNT * 1,
            15 + TANKS_SPRITE_COLS_AMOUNT * 1,
            13 + TANKS_SPRITE_COLS_AMOUNT * 1,
            11 + TANKS_SPRITE_COLS_AMOUNT * 1
        ],
        vec![
            8 + TANKS_SPRITE_COLS_AMOUNT * 2,
            14 + TANKS_SPRITE_COLS_AMOUNT * 2,
            12 + TANKS_SPRITE_COLS_AMOUNT * 2,
            10 + TANKS_SPRITE_COLS_AMOUNT * 2,
            9 + TANKS_SPRITE_COLS_AMOUNT * 2,
            15 + TANKS_SPRITE_COLS_AMOUNT * 2,
            13 + TANKS_SPRITE_COLS_AMOUNT * 2,
            11 + TANKS_SPRITE_COLS_AMOUNT * 2
        ],
        vec![
            8 + TANKS_SPRITE_COLS_AMOUNT * 3,
            14 + TANKS_SPRITE_COLS_AMOUNT * 3,
            12 + TANKS_SPRITE_COLS_AMOUNT * 3,
            10 + TANKS_SPRITE_COLS_AMOUNT * 3,
            9 + TANKS_SPRITE_COLS_AMOUNT * 3,
            15 + TANKS_SPRITE_COLS_AMOUNT * 3,
            13 + TANKS_SPRITE_COLS_AMOUNT * 3,
            11 + TANKS_SPRITE_COLS_AMOUNT * 3
        ],
        vec![
            8 + TANKS_SPRITE_COLS_AMOUNT * 4,
            14 + TANKS_SPRITE_COLS_AMOUNT * 4,
            12 + TANKS_SPRITE_COLS_AMOUNT * 4,
            10 + TANKS_SPRITE_COLS_AMOUNT * 4,
            9 + TANKS_SPRITE_COLS_AMOUNT * 4,
            15 + TANKS_SPRITE_COLS_AMOUNT * 4,
            13 + TANKS_SPRITE_COLS_AMOUNT * 4,
            11 + TANKS_SPRITE_COLS_AMOUNT * 4
        ],
        vec![
            8 + TANKS_SPRITE_COLS_AMOUNT * 5,
            14 + TANKS_SPRITE_COLS_AMOUNT * 5,
            12 + TANKS_SPRITE_COLS_AMOUNT * 5,
            10 + TANKS_SPRITE_COLS_AMOUNT * 5,
            9 + TANKS_SPRITE_COLS_AMOUNT * 5,
            15 + TANKS_SPRITE_COLS_AMOUNT * 5,
            13 + TANKS_SPRITE_COLS_AMOUNT * 5,
            11 + TANKS_SPRITE_COLS_AMOUNT * 5
        ],
        vec![
            8 + TANKS_SPRITE_COLS_AMOUNT * 6,
            14 + TANKS_SPRITE_COLS_AMOUNT * 6,
            12 + TANKS_SPRITE_COLS_AMOUNT * 6,
            10 + TANKS_SPRITE_COLS_AMOUNT * 6,
            9 + TANKS_SPRITE_COLS_AMOUNT * 6,
            15 + TANKS_SPRITE_COLS_AMOUNT * 6,
            13 + TANKS_SPRITE_COLS_AMOUNT * 6,
            11 + TANKS_SPRITE_COLS_AMOUNT * 6
        ],
        vec![
            8 + TANKS_SPRITE_COLS_AMOUNT * 7,
            14 + TANKS_SPRITE_COLS_AMOUNT * 7,
            12 + TANKS_SPRITE_COLS_AMOUNT * 7,
            10 + TANKS_SPRITE_COLS_AMOUNT * 7,
            9 + TANKS_SPRITE_COLS_AMOUNT * 7,
            15 + TANKS_SPRITE_COLS_AMOUNT * 7,
            13 + TANKS_SPRITE_COLS_AMOUNT * 7,
            11 + TANKS_SPRITE_COLS_AMOUNT * 7
        ],
    ]
}
pub fn new_sprite_index(current_index: i32, direction: common::Direction) -> i32 {
    let index_sets = enemies_sprite_index_sets();
    for index_set in index_sets {
        if index_set.contains(&current_index) {
            trace!("found index_set");
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
