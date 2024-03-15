use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::area::*;
use crate::common::{self, Direction, *};
use crate::enemy::Enemy;
use crate::level::LevelItem;
use crate::player::{PlayerLives, PlayerNo, Shield};

pub const BULLET_SPEED: f32 = 300.0;

#[derive(Component, PartialEq, Eq)]
pub enum Bullet {
    Player,
    Enemy,
}

#[derive(Debug, Component)]
pub struct Explosion;

#[derive(Debug, Event)]
pub struct ExplosionEvent {
    pos: Vec3,
    explosion_type: ExplosionType,
}

#[derive(Debug, PartialEq, Eq)]
pub enum ExplosionType {
    BigExplosion,
    BulletExplosion,
}

#[derive(Debug, Resource)]
pub struct ExplosionAssets {
    pub big_explosion: Vec<Handle<Image>>,
    pub bullet_explosion: Vec<Handle<Image>>,
}

pub fn setup_explosion_assets(mut commands: Commands, asset_server: Res<AssetServer>) {
    let mut big_explosion: Vec<Handle<Image>> = Vec::new();
    big_explosion.push(asset_server.load("textures/big_explosion_1.png"));
    big_explosion.push(asset_server.load("textures/big_explosion_2.png"));
    big_explosion.push(asset_server.load("textures/big_explosion_3.png"));
    big_explosion.push(asset_server.load("textures/big_explosion_4.png"));
    big_explosion.push(asset_server.load("textures/big_explosion_5.png"));

    let mut bullet_explosion: Vec<Handle<Image>> = Vec::new();
    bullet_explosion.push(asset_server.load("textures/bullet_explosion_1.png"));
    bullet_explosion.push(asset_server.load("textures/bullet_explosion_2.png"));
    bullet_explosion.push(asset_server.load("textures/bullet_explosion_3.png"));

    commands.insert_resource(ExplosionAssets {
        big_explosion,
        bullet_explosion,
    });
}

// 炮弹移动 // Bullet movement
pub fn move_bullet(
    mut q_bullet: Query<(&mut Transform, &common::Direction), With<Bullet>>,
    time: Res<Time>,
) {
    for (mut bullet_transform, direction) in &mut q_bullet {
        match direction {
            common::Direction::Left => {
                bullet_transform.translation.x -= BULLET_SPEED * time.delta_seconds()
            }
            common::Direction::Right => {
                bullet_transform.translation.x += BULLET_SPEED * time.delta_seconds()
            }
            common::Direction::Up => {
                bullet_transform.translation.y += BULLET_SPEED * time.delta_seconds()
            }
            common::Direction::Down => {
                bullet_transform.translation.y -= BULLET_SPEED * time.delta_seconds()
            }
        }
    }
}

pub fn handle_bullet_collision(
    mut commands: Commands,
    q_bullets: Query<(Entity, &Bullet, &Transform)>,
    q_level_items: Query<(&LevelItem, &GlobalTransform, &mut TextureAtlasSprite)>,
    q_area_wall: Query<(), With<AreaWall>>,
    q_players: Query<(&Transform, &Children), With<PlayerNo>>,
    q_shields: Query<Entity, With<Shield>>,
    q_enemies: Query<&Transform, With<Enemy>>,
    mut collision_er: EventReader<CollisionEvent>,
    mut explosion_ew: EventWriter<ExplosionEvent>,
    mut home_dying_ew: EventWriter<HomeDyingEvent>,
    player_lives: Res<PlayerLives>,
    multiplayer_mode: Res<MultiplayerMode>,
    mut app_state: ResMut<NextState<AppState>>,
) {
    for event in collision_er.read() {
        match event {
            CollisionEvent::Started(entity1, entity2, _flags)
            | CollisionEvent::Stopped(entity1, entity2, _flags) => {
                let bullet_entity = if q_bullets.contains(*entity1) {
                    *entity1
                } else if q_bullets.contains(*entity2) {
                    *entity2
                } else {
                    continue;
                };
                let other_entity = if bullet_entity == *entity1 {
                    *entity2
                } else {
                    *entity1
                };

                trace!(
                    "Bullet: {:?}, collision entity1: {:?}, entity2: {:?}",
                    bullet_entity,
                    entity1,
                    entity2
                );

                let bullet = q_bullets.get_component::<Bullet>(bullet_entity).unwrap();
                let bullet_transform = q_bullets.get_component::<Transform>(bullet_entity).unwrap();

                // 另一个物体 // Another object
                if q_level_items.contains(other_entity) {
                    let level_item = q_level_items
                        .get_component::<LevelItem>(other_entity)
                        .unwrap();
                    let level_item_transform = q_level_items
                        .get_component::<GlobalTransform>(other_entity)
                        .unwrap();
                    trace!("Bullet hit {:?}", level_item);
                    // dbg!(level_item);
                    // dbg!(bullet_transform);
                    // dbg!(level_item_transform);
                    match level_item {
                        LevelItem::Home => {
                            // Game Over
                            info!("Game over");
                            commands.entity(bullet_entity).despawn();
                            explosion_ew.send(ExplosionEvent {
                                pos: Vec3::new(
                                    level_item_transform.translation().x,
                                    level_item_transform.translation().y,
                                    level_item_transform.translation().z,
                                ),
                                explosion_type: ExplosionType::BigExplosion,
                            });
                            home_dying_ew.send_default();
                        }
                        LevelItem::BrickWall => {
                            commands.entity(bullet_entity).despawn();
                            commands.entity(other_entity).despawn();
                            explosion_ew.send(ExplosionEvent {
                                pos: Vec3::new(
                                    bullet_transform.translation.x,
                                    bullet_transform.translation.y,
                                    bullet_transform.translation.z,
                                ),
                                explosion_type: ExplosionType::BulletExplosion,
                            });
                        }
                        LevelItem::IronWall => {
                            commands.entity(bullet_entity).despawn();
                            explosion_ew.send(ExplosionEvent {
                                pos: Vec3::new(
                                    bullet_transform.translation.x,
                                    bullet_transform.translation.y,
                                    bullet_transform.translation.z,
                                ),
                                explosion_type: ExplosionType::BulletExplosion,
                            });
                        }
                        _ => {}
                    }
                }

                if q_area_wall.contains(other_entity) {
                    trace!("Bullet hit area wall");
                    commands.entity(bullet_entity).despawn();
                    explosion_ew.send(ExplosionEvent {
                        pos: Vec3::new(
                            bullet_transform.translation.x,
                            bullet_transform.translation.y,
                            bullet_transform.translation.z,
                        ),
                        explosion_type: ExplosionType::BulletExplosion,
                    });
                }

                if *bullet == Bullet::Player && q_enemies.contains(other_entity) {
                    debug!("Player bullet hit enemy");
                    let enemy_transform =
                        q_enemies.get_component::<Transform>(other_entity).unwrap();
                    commands.entity(bullet_entity).despawn();
                    commands.entity(other_entity).despawn();
                    explosion_ew.send(ExplosionEvent {
                        pos: Vec3::new(
                            enemy_transform.translation.x,
                            enemy_transform.translation.y,
                            enemy_transform.translation.z,
                        ),
                        explosion_type: ExplosionType::BigExplosion,
                    });
                }

                if *bullet == Bullet::Enemy && q_players.contains(other_entity) {
                    debug!("Enemy bullet hit player");
                    let player_children =
                        q_players.get_component::<Children>(other_entity).unwrap();
                    let mut player_has_shield = false;
                    for child in player_children.iter() {
                        if q_shields.contains(*child) {
                            player_has_shield = true;
                            break;
                        }
                    }

                    let player_transform =
                        q_players.get_component::<Transform>(other_entity).unwrap();
                    commands.entity(bullet_entity).despawn();

                    if player_has_shield {
                        trace!("Player has shield");
                        explosion_ew.send(ExplosionEvent {
                            pos: Vec3::new(
                                player_transform.translation.x,
                                player_transform.translation.y,
                                player_transform.translation.z,
                            ),
                            explosion_type: ExplosionType::BulletExplosion,
                        });
                    } else {
                        commands.entity(other_entity).despawn_recursive();
                        explosion_ew.send(ExplosionEvent {
                            pos: Vec3::new(
                                player_transform.translation.x,
                                player_transform.translation.y,
                                player_transform.translation.z,
                            ),
                            explosion_type: ExplosionType::BigExplosion,
                        });
                        if player_lives.player1 <= 0 && player_lives.player2 <= 0 {
                            app_state.set(AppState::GameOver);
                        }
                        if player_lives.player1 <= 0
                            && *multiplayer_mode == MultiplayerMode::SinglePlayer
                        {
                            app_state.set(AppState::GameOver);
                        }
                    }
                }
            }
        }
    }
}

pub fn spawn_bullet(
    commands: &mut Commands,
    game_texture_atlas: &Res<GameTextureAtlasHandles>,
    bullet: Bullet,
    translation: Vec3,
    direction: Direction,
) {
    commands
        .spawn(bullet)
        .insert(SpriteSheetBundle {
            texture_atlas: game_texture_atlas.bullet.clone(),
            sprite: TextureAtlasSprite {
                index: match direction {
                    common::Direction::Up => 0,
                    common::Direction::Right => 1,
                    common::Direction::Down => 2,
                    common::Direction::Left => 3,
                },
                ..default()
            },
            transform: Transform {
                translation: Vec3::new(translation.x, translation.y, translation.z),
                ..default()
            },
            ..default()
        })
        .insert((
            Collider::cuboid(2.0, 2.0),
            Sensor,
            RigidBody::Dynamic,
            ActiveEvents::COLLISION_EVENTS,
        ))
        .insert(direction);
}

pub fn spawn_explosion(
    mut commands: Commands,
    mut explosion_er: EventReader<ExplosionEvent>,
    explosion_assets: Res<ExplosionAssets>,
    asset_server: Res<AssetServer>,
    mut textures: ResMut<Assets<Image>>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    game_sounds: Res<GameSounds>,
) {
    let mut big_explosion_texture_atlas_builder = TextureAtlasBuilder::default();
    for handle in &explosion_assets.big_explosion {
        let Some(texture) = textures.get(handle.id()) else {
            warn!(
                "{:?} did not resolve to an `Image` asset.",
                asset_server.get_path(handle.id())
            );
            continue;
        };
        big_explosion_texture_atlas_builder.add_texture(handle.id(), texture);
    }
    let big_explosion_texture_atlas = big_explosion_texture_atlas_builder
        .finish(&mut textures)
        .unwrap();
    let big_explosion_texture_atlas_handle = texture_atlases.add(big_explosion_texture_atlas);

    let mut bullet_explosion_texture_atlas_builder = TextureAtlasBuilder::default();
    for handle in &explosion_assets.bullet_explosion {
        let Some(texture) = textures.get(handle.id()) else {
            warn!(
                "{:?} did not resolve to an `Image` asset.",
                asset_server.get_path(handle.id())
            );
            continue;
        };
        bullet_explosion_texture_atlas_builder.add_texture(handle.id(), texture);
    }
    let bullet_explosion_texture_atlas = bullet_explosion_texture_atlas_builder
        .finish(&mut textures)
        .unwrap();
    let bullet_explosion_texture_atlas_handle = texture_atlases.add(bullet_explosion_texture_atlas);

    for explosion in explosion_er.read() {
        commands.spawn((
            Explosion,
            SpriteSheetBundle {
                sprite: TextureAtlasSprite::new(0),
                texture_atlas: if explosion.explosion_type == ExplosionType::BigExplosion {
                    big_explosion_texture_atlas_handle.clone()
                } else {
                    bullet_explosion_texture_atlas_handle.clone()
                },
                transform: Transform::from_translation(explosion.pos),
                ..default()
            },
            AnimationTimer(Timer::from_seconds(0.05, TimerMode::Repeating)),
            AnimationIndices {
                first: 0,
                last: if explosion.explosion_type == ExplosionType::BigExplosion {
                    4
                } else {
                    2
                },
            },
        ));
        if explosion.explosion_type == ExplosionType::BigExplosion {
            commands.spawn(AudioBundle {
                source: game_sounds.big_explosion.clone(),
                settings: PlaybackSettings::DESPAWN,
            });
        } else if explosion.explosion_type == ExplosionType::BulletExplosion {
            commands.spawn(AudioBundle {
                source: game_sounds.bullet_explosion.clone(),
                settings: PlaybackSettings::DESPAWN,
            });
        }
    }
}

pub fn animate_explosion(
    mut commands: Commands,
    mut q_explosion: Query<
        (
            Entity,
            &mut AnimationTimer,
            &AnimationIndices,
            &mut TextureAtlasSprite,
        ),
        With<Explosion>,
    >,
    time: Res<Time>,
) {
    for (entity, mut timer, indices, mut sprite) in &mut q_explosion {
        timer.0.tick(time.delta());
        if timer.0.just_finished() {
            sprite.index += 1;
            if sprite.index > indices.last {
                commands.entity(entity).despawn();
            }
        }
    }
}

pub fn cleanup_bullets(mut commands: Commands, q_bullets: Query<Entity, With<Bullet>>) {
    for entity in &q_bullets {
        commands.entity(entity).despawn_recursive();
    }
}

pub fn cleanup_explosions(mut commands: Commands, q_explosions: Query<Entity, With<Explosion>>) {
    for entity in &q_explosions {
        commands.entity(entity).despawn_recursive();
    }
}
