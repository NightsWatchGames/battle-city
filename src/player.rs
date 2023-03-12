use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::bullet::*;
use crate::common::{self, *};
use crate::area::*;

pub const TANK_SPEED: f32 = 200.0;

// 坦克刷新子弹间隔
pub const TANK_REFRESH_BULLET_INTERVAL: f32 = 2.0;

// 玩家1
#[derive(Component)]
pub struct Player1;

// 玩家2
#[derive(Component)]
pub struct Player2;

// 坦克刷新子弹计时器
#[derive(Component, Deref, DerefMut)]
pub struct TankRefreshBulletTimer(pub Timer);

// 出生保护盾
#[derive(Component)]
pub struct Shield;

// 出生保护盾计时
#[derive(Component, Deref, DerefMut)]
pub struct ShieldRemoveTimer(pub Timer);

pub fn setup_player1(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
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
        .spawn(Player1)
        .insert(SpriteSheetBundle {
            texture_atlas: tank_texture_atlas_handle,
            transform: Transform {
                translation: Vec3::new(-50.0, BOTTOM_WALL + 100.0, 2.0),
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
        .insert(common::Direction::Up)
        .insert(RigidBody::Dynamic)
        .insert(Collider::cuboid(18.0, 18.0))
        .insert(ActiveEvents::COLLISION_EVENTS)
        .insert(LockedAxes::ROTATION_LOCKED)
        .id();

    commands.entity(tank).add_child(shield);
}

pub fn setup_player2(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    game_mode: Res<GameMode>,
) {
    if *game_mode == GameMode::SinglePlayer {
        return;
    }
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

    let tank_texture_handle = asset_server.load("textures/tank2.bmp");
    let tank_texture_atlas =
        TextureAtlas::from_grid(tank_texture_handle, Vec2::new(28.0, 28.0), 2, 4, None, None);
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
        .spawn(Player2)
        .insert(SpriteSheetBundle {
            texture_atlas: tank_texture_atlas_handle,
            transform: Transform {
                translation: Vec3::new(50.0, BOTTOM_WALL + 100.0, 2.0),
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
        .insert(common::Direction::Up)
        .insert(RigidBody::Dynamic)
        .insert(Collider::cuboid(18.0, 18.0))
        .insert(ActiveEvents::COLLISION_EVENTS)
        .insert(LockedAxes::ROTATION_LOCKED)
        .id();

    commands.entity(tank).add_child(shield);
}

// 玩家1移动坦克
pub fn player1_move(
    keyboard_input: Res<Input<KeyCode>>,
    mut transform_query: Query<
        (
            &mut Transform,
            &mut common::Direction,
            &mut TextureAtlasSprite,
        ),
        With<Player1>,
    >,
) {
    for (mut tank_transform, mut direction, mut sprite) in &mut transform_query {
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
                }
                common::Direction::Right => {
                    sprite.index = 2;
                }
                common::Direction::Down => {
                    sprite.index = 4;
                }
                common::Direction::Left => {
                    sprite.index = 6;
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
        ),
        With<Player2>,
    >,
) {
    for (mut tank_transform, mut direction, mut sprite) in &mut transform_query {
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
                }
                common::Direction::Right => {
                    sprite.index = 2;
                }
                common::Direction::Down => {
                    sprite.index = 4;
                }
                common::Direction::Left => {
                    sprite.index = 6;
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
    texture_atlases: Res<Assets<TextureAtlas>>,
    mut query: Query<
        (
            &mut AnimationTimer,
            &mut TextureAtlasSprite,
            &Handle<TextureAtlas>,
            &common::Direction,
        ),
        Or<(With<Player1>, With<Player2>)>,
    >,
) {
    for (mut timer, mut sprite, texture_atlas_handle, direction) in &mut query {
        timer.0.tick(time.delta());
        if timer.0.just_finished() {
            // 切换到下一个sprite
            let texture_atlas = texture_atlases.get(texture_atlas_handle).unwrap();
            // 每个方向上的sprite数量
            let sprites_each_direction = texture_atlas.len() / 4;
            match direction {
                common::Direction::Up => {
                    sprite.index =
                        (sprite.index + 1) % sprites_each_direction + sprites_each_direction * 0;
                }
                common::Direction::Right => {
                    sprite.index =
                        (sprite.index + 1) % sprites_each_direction + sprites_each_direction * 1;
                }
                common::Direction::Down => {
                    sprite.index =
                        (sprite.index + 1) % sprites_each_direction + sprites_each_direction * 2;
                }
                common::Direction::Left => {
                    sprite.index =
                        (sprite.index + 1) % sprites_each_direction + sprites_each_direction * 3;
                }
            }
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
