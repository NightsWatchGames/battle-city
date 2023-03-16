use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::bullet::*;
use crate::common::{self, *};
use crate::level::Player2Marker;
use crate::level::{Player1Marker, LEVEL_TRANSLATION_OFFSET};

pub const TANK_SPEED: f32 = 200.0;
pub const TANK_SIZE: f32 = 28.0;

// 出生保护盾
#[derive(Component)]
pub struct Shield;

// 出生保护盾计时
#[derive(Component, Deref, DerefMut)]
pub struct ShieldRemoveTimer(pub Timer);

// 出生特效
#[derive(Component)]
pub struct Born;

#[derive(Debug, Clone, Copy, Component, Reflect, Default)]
#[reflect(Component)]
pub struct PlayerNo(pub u32);

#[derive(Debug)]
pub struct SpawnPlayerEvent {
    pos: Vec2,
    player_no: PlayerNo,
}

pub fn auto_spawn_players(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    q_players: Query<&PlayerNo>,
    q_player1_marker: Query<&Transform, With<Player1Marker>>,
    q_player2_marker: Query<&Transform, With<Player2Marker>>,
    mut spawn_player_er: EventReader<SpawnPlayerEvent>,
    mut spawning_player1: Local<bool>,
    mut spawning_player2: Local<bool>,
    multiplayer_mode: Res<MultiplayerMode>,
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
            if !*spawning_player1 {
                // 出生动画
                spawn_born(
                    player1_marker.translation + LEVEL_TRANSLATION_OFFSET,
                    PlayerNo(1),
                    &mut commands,
                    &asset_server,
                    &mut texture_atlases,
                );
                *spawning_player1 = true;
            }
        }
    }
    if !player2_exists && *multiplayer_mode == MultiplayerMode::TwoPlayers {
        for player2_marker in &q_player2_marker {
            if !*spawning_player2 {
                // 出生动画
                spawn_born(
                    player2_marker.translation + LEVEL_TRANSLATION_OFFSET,
                    PlayerNo(2),
                    &mut commands,
                    &asset_server,
                    &mut texture_atlases,
                );
                *spawning_player2 = true;
            }
        }
    }
    // 出生动画完毕后，进行player创建
    for spawn_player_event in spawn_player_er.iter() {
        dbg!(spawn_player_event);
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

        let tank_texture_handle = asset_server.load(if spawn_player_event.player_no.0 == 1 {
            "textures/tank1.bmp"
        } else {
            "textures/tank2.bmp"
        });
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
                spawn_player_event.player_no,
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

// 玩家移动坦克
pub fn players_move(
    time: Res<Time>,
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(
        &PlayerNo,
        &mut Transform,
        &mut common::Direction,
        &mut TextureAtlasSprite,
        &mut AnimationIndices,
    )>,
) {
    for (player_no, mut transform, mut direction, mut sprite, mut indices) in &mut query {
        // 一次只能移动一个方向
        if (player_no.0 == 1 && keyboard_input.pressed(KeyCode::W))
            || (player_no.0 == 2 && keyboard_input.pressed(KeyCode::Up))
        {
            transform.translation.y += time.delta_seconds() * TANK_SPEED;
            *direction = common::Direction::Up;
        } else if (player_no.0 == 1 && keyboard_input.pressed(KeyCode::S))
            || (player_no.0 == 2 && keyboard_input.pressed(KeyCode::Down))
        {
            transform.translation.y -= time.delta_seconds() * TANK_SPEED;
            *direction = common::Direction::Down;
        } else if (player_no.0 == 1 && keyboard_input.pressed(KeyCode::A))
            || (player_no.0 == 2 && keyboard_input.pressed(KeyCode::Left))
        {
            transform.translation.x -= time.delta_seconds() * TANK_SPEED;
            *direction = common::Direction::Left;
        } else if (player_no.0 == 1 && keyboard_input.pressed(KeyCode::D))
            || (player_no.0 == 2 && keyboard_input.pressed(KeyCode::Right))
        {
            transform.translation.x += time.delta_seconds() * TANK_SPEED;
            *direction = common::Direction::Right;
        } else {
            continue;
        }

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
    keyboard_input: Res<Input<KeyCode>>,
    mut q_players: Query<(
        &PlayerNo,
        &Transform,
        &common::Direction,
        &mut TankRefreshBulletTimer,
    )>,
    time: Res<Time>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    audio: Res<Audio>,
    game_sounds: Res<GameSounds>,
) {
    for (player_no, transform, direction, mut refresh_bullet_timer) in &mut q_players {
        refresh_bullet_timer.tick(time.delta());
        if (player_no.0 == 1 && keyboard_input.just_pressed(KeyCode::Space))
            || (player_no.0 == 2 && keyboard_input.just_pressed(KeyCode::Return))
        {
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
                audio.play(game_sounds.fire.clone());
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
