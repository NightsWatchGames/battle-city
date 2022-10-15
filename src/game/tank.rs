use bevy::{
    prelude::*,
    sprite::collide_aabb::{collide, Collision},
    time::FixedTimestep,
};

// 坦克
#[derive(Component)]
struct Tank;
