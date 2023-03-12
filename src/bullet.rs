use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::common::{self, Direction, *};
use crate::enemy::Enemy;
use crate::level::LevelItem;
use crate::wall::*;

pub const BULLET_SPEED: f32 = 300.0;

#[derive(Component)]
pub enum Bullet {
    Player,
    Enemy,
}

#[derive(Debug, Component)]
pub struct ExplosionEffect;

#[derive(Debug)]
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

// 炮弹移动
pub fn move_bullet(mut transform_query: Query<(&mut Transform, &common::Direction), With<Bullet>>) {
    let bullet_movement = 1.0 * BULLET_SPEED * TIME_STEP;
    for (mut bullet_transform, direction) in &mut transform_query {
        match direction {
            common::Direction::Left => bullet_transform.translation.x -= bullet_movement,
            common::Direction::Right => bullet_transform.translation.x += bullet_movement,
            common::Direction::Up => bullet_transform.translation.y += bullet_movement,
            common::Direction::Down => bullet_transform.translation.y -= bullet_movement,
        }
    }
}

pub fn check_bullet_collision(
    mut commands: Commands,
    mut collision_events: EventReader<CollisionEvent>,
    mut q_bullet: Query<(Entity, &Bullet, &Transform)>,
    q_level_item: Query<&LevelItem>,
    q_wall: Query<&Wall>,
    q_enemy: Query<&Enemy>,
    mut explosion_ew: EventWriter<ExplosionEvent>,
) {
    for (entity, bullet, transform) in &mut q_bullet {
        for event in collision_events.iter() {
            match event {
                CollisionEvent::Started(entity1, entity2, _flags) => {
                    println!(
                        "bullet: {:?}, collision entity1: {:?}, entity2: {:?}",
                        entity, entity1, entity2
                    );
                    if *entity1 != entity && *entity2 != entity {
                        continue;
                    }
                    info!("bullet hit something");
                    // 另一个物体
                    let other_entity = if entity == *entity1 {
                        *entity2
                    } else {
                        *entity1
                    };
                    if q_level_item.contains(other_entity) {
                        info!("Bullet hit level item");
                        let level_item = q_level_item
                            .get_component::<LevelItem>(other_entity)
                            .unwrap();
                        dbg!(level_item);
                        match level_item {
                            LevelItem::Home => {
                                // Game Over
                                println!("Game over");
                                explosion_ew.send(ExplosionEvent {
                                    pos: Vec3::new(
                                        transform.translation.x,
                                        transform.translation.y,
                                        transform.translation.z,
                                    ),
                                    explosion_type: ExplosionType::BigExplosion,
                                });
                            }
                            LevelItem::StoneWall => {
                                commands.entity(entity).despawn();
                                commands.entity(other_entity).despawn();
                                explosion_ew.send(ExplosionEvent {
                                    pos: Vec3::new(
                                        transform.translation.x,
                                        transform.translation.y,
                                        transform.translation.z,
                                    ),
                                    explosion_type: ExplosionType::BulletExplosion,
                                });
                            }
                            LevelItem::IronWall => {
                                commands.entity(entity).despawn();
                                explosion_ew.send(ExplosionEvent {
                                    pos: Vec3::new(
                                        transform.translation.x,
                                        transform.translation.y,
                                        transform.translation.z,
                                    ),
                                    explosion_type: ExplosionType::BulletExplosion,
                                });
                            }
                            _ => {}
                        }
                    }
                    if q_wall.contains(other_entity) {
                        info!("Bullet hit wall");
                        commands.entity(entity).despawn();
                        explosion_ew.send(ExplosionEvent {
                            pos: Vec3::new(
                                transform.translation.x,
                                transform.translation.y,
                                transform.translation.z,
                            ),
                            explosion_type: ExplosionType::BulletExplosion,
                        });
                    }
                    if q_enemy.contains(other_entity) {
                        info!("Bullet hit enemy");
                        commands.entity(entity).despawn();
                        commands.entity(other_entity).despawn();
                        explosion_ew.send(ExplosionEvent {
                            pos: Vec3::new(
                                transform.translation.x,
                                transform.translation.y,
                                transform.translation.z,
                            ),
                            explosion_type: ExplosionType::BigExplosion,
                        });
                    }
                }
                _ => {}
            }
        }
    }
}

pub fn spawn_bullet(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    texture_atlases: &mut ResMut<Assets<TextureAtlas>>,
    bullet: Bullet,
    translation: Vec3,
    direction: Direction,
) {
    let bullet_texture_handle = asset_server.load("textures/bullet.bmp");
    let bullet_texture_atlas =
        TextureAtlas::from_grid(bullet_texture_handle, Vec2::new(7.0, 8.0), 4, 1, None, None);
    let bullet_texture_atlas_handle = texture_atlases.add(bullet_texture_atlas);

    commands
        .spawn(bullet)
        .insert(SpriteSheetBundle {
            texture_atlas: bullet_texture_atlas_handle,
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
) {
    let mut big_explosion_texture_atlas_builder = TextureAtlasBuilder::default();
    for handle in &explosion_assets.big_explosion {
        let Some(texture) = textures.get(&handle) else {
            warn!("{:?} did not resolve to an `Image` asset.", asset_server.get_handle_path(handle));
            continue;
        };
        big_explosion_texture_atlas_builder.add_texture(handle.clone(), texture);
    }
    let big_explosion_texture_atlas = big_explosion_texture_atlas_builder
        .finish(&mut textures)
        .unwrap();
    let big_explosion_texture_atlas_handle = texture_atlases.add(big_explosion_texture_atlas);

    let mut bullet_explosion_texture_atlas_builder = TextureAtlasBuilder::default();
    for handle in &explosion_assets.bullet_explosion {
        let Some(texture) = textures.get(&handle) else {
            warn!("{:?} did not resolve to an `Image` asset.", asset_server.get_handle_path(handle));
            continue;
        };
        bullet_explosion_texture_atlas_builder.add_texture(handle.clone(), texture);
    }
    let bullet_explosion_texture_atlas = bullet_explosion_texture_atlas_builder
        .finish(&mut textures)
        .unwrap();
    let bullet_explosion_texture_atlas_handle = texture_atlases.add(bullet_explosion_texture_atlas);

    for explosion in explosion_er.iter() {
        commands.spawn((
            ExplosionEffect,
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
        With<ExplosionEffect>,
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
