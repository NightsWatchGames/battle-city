use bevy::{
    prelude::*,
    sprite::collide_aabb::{collide, Collision},
    time::FixedTimestep,
};

// 碰撞体
#[derive(Component)]
pub struct Collider;
