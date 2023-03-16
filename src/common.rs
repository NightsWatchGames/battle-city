use bevy::prelude::*;

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
pub const SPRITE_GAME_OVER_ORDER: f32 = 4.0;
pub const SPRITE_TREE_ORDER: f32 = 3.0;
pub const SPRITE_PLAYER_ORDER: f32 = 2.0;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum AppState {
    StartMenu,
    Playing,
    GameOver,
}

#[derive(Resource, Debug, PartialEq, Eq)]
pub enum MultiplayerMode {
    SinglePlayer,
    TwoPlayers,
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

#[derive(Default)]
pub struct HomeDyingEvent;

#[derive(Debug, Resource)]
pub struct GameSounds {
    pub start: Handle<AudioSource>,
    pub explosion: Handle<AudioSource>,
    pub fire: Handle<AudioSource>,
}

pub fn setup_game_sounds(mut commands: Commands, assert_server: Res<AssetServer>) {
    commands.insert_resource(GameSounds {
        start: assert_server.load("sounds/start.wav"),
        explosion: assert_server.load("sounds/explosion.wav"),
        fire: assert_server.load("sounds/fire.wav"),
    });
}
