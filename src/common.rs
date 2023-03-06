use bevy::prelude::*;

// 物理帧间隔
pub const TIME_STEP: f32 = 1.0 / 60.0;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum AppState {
    StartMenu,
    Playing,
    GameOver,
}

#[derive(Resource, Debug, PartialEq, Eq)]
pub enum GameMode {
    SinglePlayer,
    MultiPlayers,
}

// 方向
#[derive(Component, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    Left,
    Right,
    Up,
    Down,
}

#[derive(Component, Deref, DerefMut)]
pub struct AnimationTimer(pub Timer);

#[derive(Component)]
pub struct AnimationIndices {
    pub first: usize,
    pub last: usize,
}
