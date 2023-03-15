use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::bullet::*;
use crate::common::{self, *};
use crate::level::Player1Marker;
use crate::level::Player2Marker;

pub const TANK_SPEED: f32 = 200.0;
pub const TANK_SIZE: f32 = 28.0;

// 玩家1
#[derive(Component)]
pub struct Player1;

// 玩家2
#[derive(Component)]
pub struct Player2;

// 出生保护盾
#[derive(Component)]
pub struct Shield;

// 出生保护盾计时
#[derive(Component, Deref, DerefMut)]
pub struct ShieldRemoveTimer(pub Timer);

// 出生特效
#[derive(Component)]
pub struct Born;

#[derive(Debug, Clone, Component)]
pub struct PlayerNo(pub u32);

#[derive(Debug)]
pub struct SpawnPlayerEvent {
    pos: Vec2,
    player_no: PlayerNo,
}

pub fn auto_spawn_player1(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    q_player1: Query<(), With<Player1>>,
    q_player1_marker: Query<&GlobalTransform, With<Player1Marker>>,
    mut spawn_player_er: EventReader<SpawnPlayerEvent>,
    mut spawning_player: Local<bool>,
) {
    if q_player1.iter().len() > 0 {
        return;
    }
    for player1_marker in &q_player1_marker {
        // 防止player1_marker还未初始化
        if player1_marker.translation() == Vec3::ZERO {
            continue;
        }
        if !*spawning_player {
            // 出生动画
            spawn_born(
                player1_marker.translation(),
                PlayerNo(1),
                &mut commands,
                &asset_server,
                &mut texture_atlases,
            );
            *spawning_player = true;
        }
    }
    // 出生动画完毕后，进行player创建
    for spawn_player_event in spawn_player_er.iter() {
        if spawn_player_event.player_no.0 == 1 {
            let shield_texture_handle = asset_server.load("textures/shield.bmp");
            let shield_texture_atlas = TextureAtlas::from_grid(
                shield_texture_handle,
                Vec2::new(31.0, 31.0),
                1,
                2,
                None,
                None,
            );
            let shield_texture_atlas_handle = texture_atlases.add(shield_texture_atlas);

            let tank_texture_handle = asset_server.load("textures/tank1.bmp");
            let tank_texture_atlas = TextureAtlas::from_grid(
                tank_texture_handle,
                Vec2::new(TANK_SIZE, TANK_SIZE),
                2,
                4,
                None,
                None,
            );
            let tank_texture_atlas_handle = texture_atlases.add(tank_texture_atlas);

            // 保护盾
            let shield = commands
                .spawn((
                    Shield,
                    SpriteSheetBundle {
                        texture_atlas: shield_texture_atlas_handle,
                        transform: Transform::from_translation(Vec3::new(0.0, 0.0, -1.0)), // 通过z轴控制sprite order
                        ..default()
                    },
                    AnimationTimer(Timer::from_seconds(0.2, TimerMode::Repeating)),
                    AnimationIndices { first: 0, last: 1 },
                    ShieldRemoveTimer(Timer::from_seconds(5.0, TimerMode::Once)),
                ))
                .id();

            // 坦克
            let tank = commands
                .spawn((
                    Player1,
                    SpriteSheetBundle {
                        texture_atlas: tank_texture_atlas_handle,
                        transform: Transform::from_translation(
                            spawn_player_event.pos.extend(SPRITE_PLAYER_ORDER),
                        ),
                        ..default()
                    },
                    TankRefreshBulletTimer(Timer::from_seconds(
                        TANK_REFRESH_BULLET_INTERVAL,
                        TimerMode::Once,
                    )),
                    common::Direction::Up,
                    AnimationTimer(Timer::from_seconds(0.2, TimerMode::Repeating)),
                    AnimationIndices { first: 0, last: 1 },
                    RigidBody::Dynamic,
                    Collider::cuboid(TANK_SIZE / 2.0, TANK_SIZE / 2.0),
                    ActiveEvents::COLLISION_EVENTS,
                    LockedAxes::ROTATION_LOCKED,
                ))
                .id();

            commands.entity(tank).add_child(shield);

            // 重置状态
            *spawning_player = false;
        }
    }
}

pub fn auto_spawn_player2(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    game_mode: Res<GameMode>,
    q_player2: Query<(), With<Player2>>,
    q_player2_marker: Query<&GlobalTransform, With<Player2Marker>>,
    mut spawn_player_er: EventReader<SpawnPlayerEvent>,
    mut spawning_player: Local<bool>,
) {
    if *game_mode == GameMode::SinglePlayer {
        return;
    }
    if q_player2.iter().len() > 0 {
        return;
    }
    for player2_marker in &q_player2_marker {
        // 防止player2_marker还未初始化
        if player2_marker.translation() == Vec3::ZERO {
            continue;
        }
        if !*spawning_player {
            // 出生动画
            spawn_born(
                player2_marker.translation(),
                PlayerNo(2),
                &mut commands,
                &asset_server,
                &mut texture_atlases,
            );
            *spawning_player = true;
        }
    }
    // 出生动画完毕后，进行player创建
    for spawn_player_event in spawn_player_er.iter() {
        if spawn_player_event.player_no.0 == 2 {
            let shield_texture_handle = asset_server.load("textures/shield.bmp");
            let shield_texture_atlas = TextureAtlas::from_grid(
                shield_texture_handle,
                Vec2::new(31.0, 31.0),
                1,
                2,
                None,
                None,
            );
            let shield_texture_atlas_handle = texture_atlases.add(shield_texture_atlas);

            let tank_texture_handle = asset_server.load("textures/tank2.bmp");
            let tank_texture_atlas = TextureAtlas::from_grid(
                tank_texture_handle,
                Vec2::new(TANK_SIZE, TANK_SIZE),
                2,
                4,
                None,
                None,
            );
            let tank_texture_atlas_handle = texture_atlases.add(tank_texture_atlas);

            // 保护盾
            let shield = commands
                .spawn((
                    Shield,
                    SpriteSheetBundle {
                        texture_atlas: shield_texture_atlas_handle,
                        transform: Transform::from_translation(Vec3::new(0.0, 0.0, -1.0)), // 通过z轴控制sprite order
                        ..default()
                    },
                    AnimationTimer(Timer::from_seconds(0.2, TimerMode::Repeating)),
                    AnimationIndices { first: 0, last: 1 },
                    ShieldRemoveTimer(Timer::from_seconds(5.0, TimerMode::Once)),
                ))
                .id();

            // 坦克
            let tank = commands
                .spawn((
                    Player2,
                    SpriteSheetBundle {
                        texture_atlas: tank_texture_atlas_handle,
                        transform: Transform::from_translation(
                            spawn_player_event.pos.extend(SPRITE_PLAYER_ORDER),
                        ),
                        ..default()
                    },
                    TankRefreshBulletTimer(Timer::from_seconds(
                        TANK_REFRESH_BULLET_INTERVAL,
                        TimerMode::Once,
                    )),
                    common::Direction::Up,
                    AnimationTimer(Timer::from_seconds(0.2, TimerMode::Repeating)),
                    AnimationIndices { first: 0, last: 1 },
                    RigidBody::Dynamic,
                    Collider::cuboid(TANK_SIZE / 2.0, TANK_SIZE / 2.0),
                    ActiveEvents::COLLISION_EVENTS,
                    LockedAxes::ROTATION_LOCKED,
                ))
                .id();

            commands.entity(tank).add_child(shield);

            // 重置状态
            *spawning_player = false;
        }
    }
}

pub fn spawn_born(
    pos: Vec3,
    player_no: PlayerNo,
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    texture_atlases: &mut ResMut<Assets<TextureAtlas>>,
) {
    let born_texture_handle = asset_server.load("textures/born.bmp");
    let born_texture_atlas =
        TextureAtlas::from_grid(born_texture_handle, Vec2::new(32.0, 32.0), 4, 1, None, None);
    let born_texture_atlas_handle = texture_atlases.add(born_texture_atlas);

    // 出生特效
    println!("spawn born once");
    commands.spawn((
        Born,
        player_no,
        SpriteSheetBundle {
            texture_atlas: born_texture_atlas_handle,
            transform: Transform::from_translation(pos),
            ..default()
        },
        AnimationTimer(Timer::from_seconds(0.2, TimerMode::Repeating)),
        AnimationIndices { first: 0, last: 3 },
    ));
}

// 玩家1移动坦克
pub fn player1_move(
    keyboard_input: Res<Input<KeyCode>>,
    mut transform_query: Query<
        (
            &mut Transform,
            &mut common::Direction,
            &mut TextureAtlasSprite,
            &mut AnimationIndices,
        ),
        With<Player1>,
    >,
) {
    for (mut tank_transform, mut direction, mut sprite, mut indices) in &mut transform_query {
        let mut tank_x_position = tank_transform.translation.x;
        let mut tank_y_position = tank_transform.translation.y;

        let ori_direction = direction.clone();
        // 一次只能移动一个方向
        // 根据速度时间计算新坐标
        if keyboard_input.pressed(KeyCode::A) {
            tank_x_position -= 1.0 * TANK_SPEED * TIME_STEP;
            *direction = common::Direction::Left;
        } else if keyboard_input.pressed(KeyCode::D) {
            tank_x_position += 1.0 * TANK_SPEED * TIME_STEP;
            *direction = common::Direction::Right;
        } else if keyboard_input.pressed(KeyCode::W) {
            tank_y_position += 1.0 * TANK_SPEED * TIME_STEP;
            *direction = common::Direction::Up;
        } else if keyboard_input.pressed(KeyCode::S) {
            tank_y_position -= 1.0 * TANK_SPEED * TIME_STEP;
            *direction = common::Direction::Down;
        } else {
            return;
        }

        if direction.clone() != ori_direction {
            match *direction {
                common::Direction::Up => {
                    sprite.index = 0;
                    *indices = AnimationIndices { first: 0, last: 1 };
                }
                common::Direction::Right => {
                    sprite.index = 2;
                    *indices = AnimationIndices { first: 2, last: 3 };
                }
                common::Direction::Down => {
                    sprite.index = 4;
                    *indices = AnimationIndices { first: 4, last: 5 };
                }
                common::Direction::Left => {
                    sprite.index = 6;
                    *indices = AnimationIndices { first: 6, last: 7 };
                }
            }
        }

        tank_transform.translation.x = tank_x_position;
        tank_transform.translation.y = tank_y_position;
    }
}

// 玩家2移动坦克
pub fn player2_move(
    keyboard_input: Res<Input<KeyCode>>,
    mut transform_query: Query<
        (
            &mut Transform,
            &mut common::Direction,
            &mut TextureAtlasSprite,
            &mut AnimationIndices,
        ),
        With<Player2>,
    >,
) {
    for (mut tank_transform, mut direction, mut sprite, mut indices) in &mut transform_query {
        let mut tank_x_position = tank_transform.translation.x;
        let mut tank_y_position = tank_transform.translation.y;

        let ori_direction = direction.clone();
        // 一次只能移动一个方向
        // 根据速度时间计算新坐标
        if keyboard_input.pressed(KeyCode::Left) {
            tank_x_position -= 1.0 * TANK_SPEED * TIME_STEP;
            *direction = common::Direction::Left;
        } else if keyboard_input.pressed(KeyCode::Right) {
            tank_x_position += 1.0 * TANK_SPEED * TIME_STEP;
            *direction = common::Direction::Right;
        } else if keyboard_input.pressed(KeyCode::Up) {
            tank_y_position += 1.0 * TANK_SPEED * TIME_STEP;
            *direction = common::Direction::Up;
        } else if keyboard_input.pressed(KeyCode::Down) {
            tank_y_position -= 1.0 * TANK_SPEED * TIME_STEP;
            *direction = common::Direction::Down;
        } else {
            return;
        }

        if direction.clone() != ori_direction {
            match *direction {
                common::Direction::Up => {
                    sprite.index = 0;
                    *indices = AnimationIndices { first: 0, last: 1 };
                }
                common::Direction::Right => {
                    sprite.index = 2;
                    *indices = AnimationIndices { first: 2, last: 3 };
                }
                common::Direction::Down => {
                    sprite.index = 4;
                    *indices = AnimationIndices { first: 4, last: 5 };
                }
                common::Direction::Left => {
                    sprite.index = 6;
                    *indices = AnimationIndices { first: 6, last: 7 };
                }
            }
        }

        tank_transform.translation.x = tank_x_position;
        tank_transform.translation.y = tank_y_position;
    }
}

// 坦克移动动画播放
pub fn animate_players(
    time: Res<Time>,
    mut query: Query<
        (
            &mut AnimationTimer,
            &AnimationIndices,
            &mut TextureAtlasSprite,
        ),
        Or<(With<Player1>, With<Player2>)>,
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

// 玩家攻击
pub fn players_attack(
    keyboard_input: Res<Input<KeyCode>>,
    mut q_player1: Query<
        (&Transform, &common::Direction, &mut TankRefreshBulletTimer),
        With<Player1>,
    >,
    mut q_player2: Query<
        (&Transform, &common::Direction, &mut TankRefreshBulletTimer),
        (With<Player2>, Without<Player1>),
    >,
    time: Res<Time>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    for (transform, direction, mut refresh_bullet_timer) in &mut q_player1 {
        refresh_bullet_timer.tick(time.delta());
        if keyboard_input.just_pressed(KeyCode::Space) {
            if refresh_bullet_timer.finished() {
                // TODO startup时加载texture
                spawn_bullet(
                    &mut commands,
                    &asset_server,
                    &mut texture_atlases,
                    Bullet::Player,
                    transform.translation,
                    direction.clone(),
                );
                refresh_bullet_timer.reset();
            }
        }
    }
    for (transform, direction, mut refresh_bullet_timer) in &mut q_player2 {
        refresh_bullet_timer.tick(time.delta());
        if keyboard_input.any_just_pressed([KeyCode::NumpadEnter, KeyCode::Return]) {
            if refresh_bullet_timer.finished() {
                spawn_bullet(
                    &mut commands,
                    &asset_server,
                    &mut texture_atlases,
                    Bullet::Player,
                    transform.translation,
                    direction.clone(),
                );
                refresh_bullet_timer.reset();
            }
        }
    }
}

// 保护盾动画播放
pub fn animate_shield(
    time: Res<Time>,
    mut query: Query<
        (
            &mut AnimationTimer,
            &AnimationIndices,
            &mut TextureAtlasSprite,
        ),
        With<Shield>,
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

// 移除保护盾
pub fn remove_shield(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut ShieldRemoveTimer), With<Shield>>,
) {
    for (entity, mut timer) in query.iter_mut() {
        timer.tick(time.delta());

        if timer.finished() {
            commands.entity(entity).despawn();
        }
    }
}

// 出生动画播放
pub fn animate_born(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<
        (
            Entity,
            &PlayerNo,
            &Transform,
            &mut AnimationTimer,
            &AnimationIndices,
            &mut TextureAtlasSprite,
        ),
        With<Born>,
    >,
    mut spawn_player_ew: EventWriter<SpawnPlayerEvent>,
) {
    for (entity, player_no, transform, mut timer, indices, mut sprite) in &mut query {
        timer.0.tick(time.delta());
        if timer.0.just_finished() {
            // 切换到下一个sprite
            sprite.index += 1;
            if sprite.index > indices.last {
                commands.entity(entity).despawn();
                spawn_player_ew.send(SpawnPlayerEvent {
                    pos: transform.translation.truncate(),
                    player_no: player_no.clone(),
                });
            }
        }
    }
}
