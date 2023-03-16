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
    Paused,
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
    pub start_menu: Handle<AudioSource>,
    pub mode_switch: Handle<AudioSource>,
    pub bullet_explosion: Handle<AudioSource>,
    pub big_explosion: Handle<AudioSource>,
    pub player_fire: Handle<AudioSource>,
    pub game_over: Handle<AudioSource>,
    pub game_pause: Handle<AudioSource>,
}

pub fn setup_game_sounds(mut commands: Commands, assert_server: Res<AssetServer>) {
    commands.insert_resource(GameSounds {
        start_menu: assert_server.load("sounds/start_menu.ogg"),
        mode_switch: assert_server.load("sounds/mode_switch.ogg"),
        bullet_explosion: assert_server.load("sounds/bullet_explosion.ogg"),
        big_explosion: assert_server.load("sounds/big_explosion.ogg"),
        player_fire: assert_server.load("sounds/player_fire.ogg"),
        game_over: assert_server.load("sounds/game_over.ogg"),
        game_pause: assert_server.load("sounds/game_pause.ogg"),
    });
}
