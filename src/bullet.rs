use crate::common::*;
use crate::common::{self, *};
use crate::wall::*;

use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

pub const BULLET_SPEED: f32 = 300.0;

#[derive(Component)]
pub struct Bullet;

// 炮弹移动
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
                        
                    }
                }
                _ => {}
            }
        }
    }
}
