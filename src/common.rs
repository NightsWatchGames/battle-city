use bevy::prelude::*;

// 物理帧间隔
pub const TIME_STEP: f32 = 1.0 / 60.0;
// 关卡地图行数和列数
pub const LEVEL_ROWS: i32 = 18;
pub const LEVEL_COLUMNS: i32 = 27;
pub const TILE_SIZE: f32 = 32.0;
// 关卡数量
pub const MAX_LEVELS: i32 = 2;
// 同时共存的敌人最大数量
pub const MAX_LIVE_ENEMIES: i32 = 6;
// 每关敌人数量
pub const ENEMIES_PER_LEVEL: i32 = 12;
// 坦克刷新子弹间隔（秒）
pub const TANK_REFRESH_BULLET_INTERVAL: f32 = 0.5;

// sprite z轴顺序
pub const SPRITE_WATER_ORDER: f32 = 3.0;
pub const SPRITE_PLAYER_ORDER: f32 = 2.0;

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

// 坦克刷新子弹计时器
#[derive(Component, Deref, DerefMut)]
pub struct TankRefreshBulletTimer(pub Timer);
