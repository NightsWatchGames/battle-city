use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::bullet::*;
use crate::common::{self, *};
use crate::level::Player2Marker;
use crate::level::{Player1Marker, LEVEL_TRANSLATION_OFFSET};

// 出生保护盾
#[derive(Component)]
pub struct Shield;

// 出生保护盾计时
#[derive(Component)]
pub struct ShieldRemoveTimer(pub Timer);

// 出生特效
#[derive(Component)]
pub struct Born;
#[derive(Component)]
pub struct BornRemoveTimer(pub Timer);

#[derive(Debug, Clone, Copy, Component, Reflect, Default)]
#[reflect(Component)]
pub struct PlayerNo(pub u32);

#[derive(Debug, Event)]
pub struct SpawnPlayerEvent {
    pos: Vec2,
    player_no: PlayerNo,
}

#[derive(Debug, Resource)]
pub struct PlayerLives {
    pub player1: i8,
    pub player2: i8,
}

pub fn auto_spawn_players(
    mut commands: Commands,
    q_players: Query<&PlayerNo>,
    q_player1_marker: Query<&Transform, With<Player1Marker>>,
    q_player2_marker: Query<&Transform, With<Player2Marker>>,
    mut spawn_player_er: EventReader<SpawnPlayerEvent>,
    mut spawning_player1: Local<bool>,
    mut spawning_player2: Local<bool>,
    multiplayer_mode: Res<MultiplayerMode>,
    mut player_lives: ResMut<PlayerLives>,
    game_texture_atlas: Res<GameTextureAtlasHandles>,
) {
    let mut player1_exists = false;
    let mut player2_exists = false;
    for player in &q_players {
        if player.0 == 1 {
            player1_exists = true;
        }
        if player.0 == 2 {
            player2_exists = true;
        }
    }
    if !player1_exists {
        for player1_marker in &q_player1_marker {
            if !*spawning_player1 && player_lives.player1 > 0 {
                // 出生动画
                spawn_born(
                    player1_marker.translation + LEVEL_TRANSLATION_OFFSET,
                    PlayerNo(1),
                    &mut commands,
                    &game_texture_atlas,
                );
                *spawning_player1 = true;
            }
        }
    }
    if !player2_exists && *multiplayer_mode == MultiplayerMode::TwoPlayers {
        for player2_marker in &q_player2_marker {
            if !*spawning_player2 && player_lives.player2 > 0 {
                // 出生动画
                spawn_born(
                    player2_marker.translation + LEVEL_TRANSLATION_OFFSET,
                    PlayerNo(2),
                    &mut commands,
                    &game_texture_atlas,
                );
                *spawning_player2 = true;
            }
        }
    }
    // 出生动画完毕后，进行player创建
    for spawn_player_event in spawn_player_er.read() {
        dbg!(spawn_player_event);
        // 保护盾
        let shield = commands
            .spawn((
                Shield,
                SpriteSheetBundle {
                    texture_atlas: game_texture_atlas.shield.clone(),
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
                spawn_player_event.player_no,
                SpriteSheetBundle {
                    texture_atlas: if spawn_player_event.player_no.0 == 1 {
                        game_texture_atlas.player1.clone()
                    } else {
                        game_texture_atlas.player2.clone()
                    },
                    transform: Transform {
                        translation: spawn_player_event.pos.extend(SPRITE_PLAYER_ORDER),
                        scale: Vec3::splat(TANK_SCALE),
                        ..default()
                    },
                    ..default()
                },
                TankRefreshBulletTimer(Timer::from_seconds(
                    PLAYER_REFRESH_BULLET_INTERVAL,
                    TimerMode::Once,
                )),
                common::Direction::Up,
                AnimationTimer(Timer::from_seconds(0.2, TimerMode::Repeating)),
                AnimationIndices { first: 0, last: 1 },
                RigidBody::Dynamic,
                Velocity::zero(),
                // 圆形碰撞体防止因ROTATION_LOCKED被地形卡住
                Collider::ball(TANK_SIZE * TANK_SCALE / 2.0 + 2.0),
                ActiveEvents::COLLISION_EVENTS,
                LockedAxes::ROTATION_LOCKED,
            ))
            .id();

        commands.entity(tank).add_child(shield);

        // 生命条数减少
        if spawn_player_event.player_no.0 == 1 {
            player_lives.player1 -= 1;
        } else if spawn_player_event.player_no.0 == 2 {
            player_lives.player2 -= 1;
        }

        // 重置状态
        if spawn_player_event.player_no.0 == 1 {
            *spawning_player1 = false;
        } else if spawn_player_event.player_no.0 == 2 {
            *spawning_player2 = false;
        }
    }
}

pub fn spawn_born(
    pos: Vec3,
    player_no: PlayerNo,
    commands: &mut Commands,
    game_texture_atlas: &Res<GameTextureAtlasHandles>,
) {
    // 出生特效
    println!("spawn born once");
    commands.spawn((
        Born,
        player_no,
        SpriteSheetBundle {
            texture_atlas: game_texture_atlas.born.clone(),
            transform: Transform::from_translation(pos),
            ..default()
        },
        AnimationTimer(Timer::from_seconds(0.2, TimerMode::Repeating)),
        AnimationIndices { first: 0, last: 3 },
        BornRemoveTimer(Timer::from_seconds(2.0, TimerMode::Once)),
    ));
}

// 玩家移动坦克
pub fn players_move(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(
        &PlayerNo,
        &mut Velocity,
        &mut common::Direction,
        &mut TextureAtlasSprite,
        &mut AnimationIndices,
    )>,
) {
    for (player_no, mut velocity, mut direction, mut sprite, mut indices) in &mut query {
        if player_no.0 == 1
            && keyboard_input.any_just_released([KeyCode::W, KeyCode::S, KeyCode::A, KeyCode::D])
        {
            velocity.linvel = Vec2::ZERO;
            continue;
        }
        if player_no.0 == 2
            && keyboard_input.any_just_released([
                KeyCode::Up,
                KeyCode::Down,
                KeyCode::Left,
                KeyCode::Right,
            ])
        {
            velocity.linvel = Vec2::ZERO;
            continue;
        }
        // 一次只能移动一个方向
        if (player_no.0 == 1 && keyboard_input.pressed(KeyCode::W))
            || (player_no.0 == 2 && keyboard_input.pressed(KeyCode::Up))
        {
            velocity.linvel = Vec2::new(0.0, PLAYER_SPEED);
            *direction = common::Direction::Up;
        } else if (player_no.0 == 1 && keyboard_input.pressed(KeyCode::S))
            || (player_no.0 == 2 && keyboard_input.pressed(KeyCode::Down))
        {
            velocity.linvel = Vec2::new(0.0, -PLAYER_SPEED);
            *direction = common::Direction::Down;
        } else if (player_no.0 == 1 && keyboard_input.pressed(KeyCode::A))
            || (player_no.0 == 2 && keyboard_input.pressed(KeyCode::Left))
        {
            velocity.linvel = Vec2::new(-PLAYER_SPEED, 0.0);
            *direction = common::Direction::Left;
        } else if (player_no.0 == 1 && keyboard_input.pressed(KeyCode::D))
            || (player_no.0 == 2 && keyboard_input.pressed(KeyCode::Right))
        {
            velocity.linvel = Vec2::new(PLAYER_SPEED, 0.0);
            *direction = common::Direction::Right;
        } else {
            continue;
        }

        match *direction {
            common::Direction::Up => {
                *indices = AnimationIndices { first: 0, last: 1 };
            }
            common::Direction::Right => {
                *indices = AnimationIndices { first: 8, last: 9 };
            }
            common::Direction::Down => {
                *indices = AnimationIndices {
                    first: 16,
                    last: 17,
                };
            }
            common::Direction::Left => {
                *indices = AnimationIndices {
                    first: 24,
                    last: 25,
                };
            }
        }
        sprite.index = indices.first;
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
        With<PlayerNo>,
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
    mut commands: Commands,
    keyboard_input: Res<Input<KeyCode>>,
    mut q_players: Query<(
        &PlayerNo,
        &Transform,
        &common::Direction,
        &mut TankRefreshBulletTimer,
    )>,
    time: Res<Time>,
    game_sounds: Res<GameSounds>,
    game_texture_atlas: Res<GameTextureAtlasHandles>,
) {
    for (player_no, transform, direction, mut refresh_bullet_timer) in &mut q_players {
        refresh_bullet_timer.tick(time.delta());
        if (player_no.0 == 1 && keyboard_input.just_pressed(KeyCode::Space))
            || (player_no.0 == 2 && keyboard_input.just_pressed(KeyCode::Return))
        {
            if refresh_bullet_timer.finished() {
                spawn_bullet(
                    &mut commands,
                    &game_texture_atlas,
                    Bullet::Player,
                    transform.translation,
                    direction.clone(),
                );
                commands.spawn(AudioBundle {
                    source: game_sounds.player_fire.clone(),
                    settings: PlaybackSettings::DESPAWN,
                });
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
        timer.0.tick(time.delta());

        if timer.0.finished() {
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
            &mut BornRemoveTimer,
        ),
        With<Born>,
    >,
    mut spawn_player_ew: EventWriter<SpawnPlayerEvent>,
) {
    for (entity, player_no, transform, mut timer, indices, mut sprite, mut born_remove_timer) in
        &mut query
    {
        timer.0.tick(time.delta());
        born_remove_timer.0.tick(time.delta());
        if timer.0.just_finished() {
            // 切换到下一个sprite
            sprite.index = if sprite.index == indices.last {
                indices.first
            } else {
                sprite.index + 1
            };
        }
        if born_remove_timer.0.finished() {
            commands.entity(entity).despawn();
            spawn_player_ew.send(SpawnPlayerEvent {
                pos: transform.translation.truncate(),
                player_no: player_no.clone(),
            });
        }
    }
}

pub fn cleanup_players(mut commands: Commands, q_players: Query<Entity, With<PlayerNo>>) {
    for entity in &q_players {
        commands.entity(entity).despawn_recursive();
    }
}

pub fn cleanup_born(mut commands: Commands, q_born: Query<Entity, With<Born>>) {
    for entity in &q_born {
        commands.entity(entity).despawn_recursive();
    }
}

pub fn reset_player_lives(mut player_lives: ResMut<PlayerLives>) {
    player_lives.player1 = 3;
    player_lives.player2 = 3;
}
