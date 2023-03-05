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
pub struct Explosion;

#[derive(Debug)]
pub struct ExplosionEvent {
    pos: Vec3,
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

pub fn explode(
    mut commands: Commands,
    mut explosion_er: EventReader<ExplosionEvent>,
    asset_server: Res<AssetServer>,
) {
    for explosion in explosion_er.iter() {
        commands.spawn((
            Explosion,
            SpriteBundle {
                texture: asset_server.load("textures/big_explosion_1.png"),
                transform: Transform::from_translation(explosion.pos),
                ..default()
            },
            AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating)),
        ));
    }
}

pub fn animate_explosion(mut q_explosion: Query<(&mut AnimationTimer, &mut Handle<Image>), With<Explosion>>, time: Res<Time>, asset_server: Res<AssetServer>,) {
    for (mut timer, mut texture) in &mut q_explosion {
        timer.tick(time.delta());
        // TODO 多个图片组成texturealtas
        if timer.just_finished() {
            *texture = asset_server.load("textures/big_explosion_2.png");
        }
    }
}
