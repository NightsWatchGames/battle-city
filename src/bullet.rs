use crate::common::*;
use crate::common::{self, *};
use crate::map::MapItem;
use crate::wall::*;

use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

pub const BULLET_SPEED: f32 = 300.0;

#[derive(Component)]
pub struct Bullet;

// 炮弹移动
// TODO 撞墙消失
pub fn move_bullet(mut transform_query: Query<(&mut Transform, &common::Direction), With<Bullet>>) {
    for (mut bullet_transform, direction) in &mut transform_query {
        match direction {
            common::Direction::Left => {
                bullet_transform.translation.x -= 1.0 * BULLET_SPEED * TIME_STEP
            }
            common::Direction::Right => {
                bullet_transform.translation.x += 1.0 * BULLET_SPEED * TIME_STEP
            }
            common::Direction::Up => {
                bullet_transform.translation.y += 1.0 * BULLET_SPEED * TIME_STEP
            }
            common::Direction::Down => {
                bullet_transform.translation.y -= 1.0 * BULLET_SPEED * TIME_STEP
            }
        }
    }
}

// TODO 子弹撞墙 释放内存
pub fn check_bullet_collision(
    mut commands: Commands,
    mut collision_events: EventReader<CollisionEvent>,
    mut query: Query<Entity, With<Bullet>>,
    map_item_query: Query<&MapItem>,
) {
    for entity in &mut query {
        for event in collision_events.iter() {
            match event {
                CollisionEvent::Started(entity1, entity2, flags) => {
                    println!(
                        "bullet: {:?}, collision entity1: {:?}, entity2: {:?}",
                        entity, entity1, entity2
                    );
                    if entity == *entity1 || entity == *entity2 {
                        info!("bullet hitted something");
                        // 另一个物体
                        let other_entity = if entity == *entity1 { *entity2 } else { *entity1 };
                        if map_item_query.contains(other_entity) {
                            let map_item = map_item_query.get_component::<MapItem>(other_entity).unwrap();
                            match map_item {
                                MapItem::Home => {
                                    // Game Over
                                    println!("Game over");
                                },
                                MapItem::StoneWall => {
                                    // 石墙消失
                                    commands.entity(other_entity).despawn();
                                },
                                MapItem::IronWall => {
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
