use bevy::prelude::*;

// 物理帧间隔
pub const TIME_STEP: f32 = 1.0 / 60.0;

// 方向
#[derive(Component, Clone, PartialEq, Eq)]
pub enum Direction {
    Left,
    Right,
    Up,
    Down
}

// 速度
#[derive(Component, Deref, DerefMut)]
pub struct Velocity(Vec2);

#[derive(Component, Deref, DerefMut)]
pub struct AnimationTimer(pub Timer);