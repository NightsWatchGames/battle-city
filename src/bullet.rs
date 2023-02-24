use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::common::{self, Direction, *};
use crate::level::LevelItem;
use crate::wall::*;

pub const BULLET_SPEED: f32 = 300.0;

#[derive(Component)]
pub struct Bullet;

// 炮弹移动
// 撞墙消失
pub fn move_bullet(
    mut commands: Commands,
    mut transform_query: Query<(Entity, &mut Transform, &common::Direction), With<Bullet>>,
) {
    let bullet_movement = 1.0 * BULLET_SPEED * TIME_STEP;
    for (entity, mut bullet_transform, direction) in &mut transform_query {
        match direction {
            common::Direction::Left => {
                if bullet_transform.translation.x - bullet_movement < LEFT_WALL + WALL_THICKNESS {
                    bullet_transform.translation.x = LEFT_WALL + WALL_THICKNESS;
                    commands.entity(entity).despawn();
                } else {
                    bullet_transform.translation.x -= bullet_movement
                }
            }
            common::Direction::Right => {
                if bullet_transform.translation.x + bullet_movement > RIGHT_WALL - WALL_THICKNESS {
                    bullet_transform.translation.x = RIGHT_WALL - WALL_THICKNESS;
                    commands.entity(entity).despawn();
                } else {
                    bullet_transform.translation.x += bullet_movement
                }
            }
            common::Direction::Up => {
                if bullet_transform.translation.y + bullet_movement > TOP_WALL - WALL_THICKNESS {
                    bullet_transform.translation.y = TOP_WALL - WALL_THICKNESS;
                    commands.entity(entity).despawn();
                } else {
                    bullet_transform.translation.y += bullet_movement
                }
            }
            common::Direction::Down => {
                if bullet_transform.translation.y - bullet_movement < BOTTOM_WALL + WALL_THICKNESS {
                    bullet_transform.translation.y = BOTTOM_WALL + WALL_THICKNESS;
                    commands.entity(entity).despawn();
                } else {
                    bullet_transform.translation.y -= bullet_movement
                }
            }
        }
    }
}

// TODO 子弹撞墙 释放内存
pub fn check_bullet_collision(
    mut commands: Commands,
    mut collision_events: EventReader<CollisionEvent>,
    mut query: Query<Entity, With<Bullet>>,
    level_item_query: Query<&LevelItem>,
) {
    for entity in &mut query {
        for event in collision_events.iter() {
            match event {
                CollisionEvent::Started(entity1, entity2, _flags) => {
                    println!(
                        "bullet: {:?}, collision entity1: {:?}, entity2: {:?}",
                        entity, entity1, entity2
                    );
                    if entity == *entity1 || entity == *entity2 {
                        info!("bullet hit something");
                        // 另一个物体
                        let other_entity = if entity == *entity1 {
                            *entity2
                        } else {
                            *entity1
                        };
                        if level_item_query.contains(other_entity) {
                            let level_item = level_item_query
                                .get_component::<LevelItem>(other_entity)
                                .unwrap();
                            match level_item {
                                LevelItem::Home => {
                                    // Game Over
                                    println!("Game over");
                                }
                                LevelItem::StoneWall => {
                                    // 石墙消失
                                    commands.entity(other_entity).despawn();
                                }
                                LevelItem::IronWall => {
                                    // 子弹消失
                                    commands.entity(entity).despawn();
                                }
                                _ => {}
                            }
                        }
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
    translation: Vec3,
    direction: Direction,
) {
    let bullet_texture_handle = asset_server.load("textures/bullet.bmp");
    let bullet_texture_atlas =
        TextureAtlas::from_grid(bullet_texture_handle, Vec2::new(7.0, 8.0), 4, 1, None, None);
    let bullet_texture_atlas_handle = texture_atlases.add(bullet_texture_atlas);

    commands
        .spawn(Bullet)
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
