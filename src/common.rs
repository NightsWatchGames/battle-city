use bevy::prelude::*;

// 物理帧间隔
pub const TIME_STEP: f32 = 1.0 / 60.0;
// 关卡地图行数和列数
pub const LEVEL_ROWS: i32 = 18;
pub const LEVEL_COLUMNS: i32 = 27;
pub const TILE_SIZE: f32 = 32.0;
// 关卡数量
pub const MAX_LEVELS: i32 = 2;

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

#[derive(Component, Clone, Default, Debug)]
pub struct AnimationTimer(pub Timer);

#[derive(Component, Clone, Default, Debug)]
pub struct AnimationIndices {
    pub first: usize,
    pub last: usize,
}
