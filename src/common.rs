use bevy::prelude::*;

// 关卡地图行数和列数
pub const LEVEL_ROWS: i32 = 18;
pub const LEVEL_COLUMNS: i32 = 27;
pub const TILE_SIZE: f32 = 32.0;
// 关卡数量
pub const MAX_LEVELS: i32 = 2;
// 同时共存的敌人最大数量
pub const MAX_LIVE_ENEMIES: i32 = 5;
// 每关敌人数量
pub const ENEMIES_PER_LEVEL: i32 = 12;
// 坦克刷新子弹间隔（秒）
pub const PLAYER_REFRESH_BULLET_INTERVAL: f32 = 0.5;
pub const ENEMY_REFRESH_BULLET_INTERVAL: f32 = 2.0;
// 坦克速度、大小和缩放比例
pub const PLAYER_SPEED: f32 = 150.0;
pub const ENEMY_SPEED: f32 = 100.0;
pub const TANK_SIZE: f32 = 28.0;
pub const TANK_SCALE: f32 = 0.8;

// sprite z轴顺序
pub const SPRITE_GAME_OVER_ORDER: f32 = 4.0;
pub const SPRITE_TREE_ORDER: f32 = 3.0;
pub const SPRITE_PLAYER_ORDER: f32 = 2.0;

#[derive(Debug, Clone, Eq, PartialEq, Hash, States, Default)]
pub enum AppState {
    #[default]
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
#[derive(Component, Debug, Clone, Copy, PartialEq, Eq)]
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

#[derive(Default, Event)]
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

#[derive(Debug, Resource)]
pub struct GameTextureAtlasHandles {
    pub bullet: Handle<TextureAtlas>,
    pub shield: Handle<TextureAtlas>,
    pub born: Handle<TextureAtlas>,
    pub player1: Handle<TextureAtlas>,
    pub player2: Handle<TextureAtlas>,
    pub enemies: Handle<TextureAtlas>,
}

pub fn setup_game_sounds(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.insert_resource(GameSounds {
        start_menu: asset_server.load("sounds/start_menu.ogg"),
        mode_switch: asset_server.load("sounds/mode_switch.ogg"),
        bullet_explosion: asset_server.load("sounds/bullet_explosion.ogg"),
        big_explosion: asset_server.load("sounds/big_explosion.ogg"),
        player_fire: asset_server.load("sounds/player_fire.ogg"),
        game_over: asset_server.load("sounds/game_over.ogg"),
        game_pause: asset_server.load("sounds/game_pause.ogg"),
    });
}

pub fn setup_game_texture_atlas(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    // 炮弹
    let bullet_texture_handle = asset_server.load("textures/bullet.bmp");
    let bullet_texture_atlas =
        TextureAtlas::from_grid(bullet_texture_handle, Vec2::new(7.0, 8.0), 4, 1, None, None);
    let bullet_texture_atlas_handle = texture_atlases.add(bullet_texture_atlas);

    // 保护盾
    let shield_texture_handle = asset_server.load("textures/shield.bmp");
    let shield_texture_atlas = TextureAtlas::from_grid(
        shield_texture_handle,
        Vec2::new(31.0, 31.0),
        1,
        2,
        None,
        None,
    );
    let shield_texture_atlas_handle = texture_atlases.add(shield_texture_atlas);

    // 玩家1
    let player1_texture_handle = asset_server.load("textures/tank1.bmp");
    let player1_texture_atlas = TextureAtlas::from_grid(
        player1_texture_handle,
        Vec2::new(TANK_SIZE, TANK_SIZE),
        8,
        4,
        None,
        None,
    );
    let player1_texture_atlas_handle = texture_atlases.add(player1_texture_atlas);

    // 玩家2
    let player2_texture_handle = asset_server.load("textures/tank2.bmp");
    let player2_texture_atlas = TextureAtlas::from_grid(
        player2_texture_handle,
        Vec2::new(TANK_SIZE, TANK_SIZE),
        8,
        4,
        None,
        None,
    );
    let player2_texture_atlas_handle = texture_atlases.add(player2_texture_atlas);

    // 出生效果
    let born_texture_handle = asset_server.load("textures/born.bmp");
    let born_texture_atlas =
        TextureAtlas::from_grid(born_texture_handle, Vec2::new(32.0, 32.0), 4, 1, None, None);
    let born_texture_atlas_handle = texture_atlases.add(born_texture_atlas);

    let enemies_texture_handle = asset_server.load("textures/enemies.bmp");
    let enemies_texture_atlas = TextureAtlas::from_grid(
        enemies_texture_handle,
        Vec2::new(TANK_SIZE, TANK_SIZE),
        8,
        8,
        None,
        None,
    );
    let enemies_texture_atlas_handle = texture_atlases.add(enemies_texture_atlas);

    commands.insert_resource(GameTextureAtlasHandles {
        bullet: bullet_texture_atlas_handle,
        shield: shield_texture_atlas_handle,
        born: born_texture_atlas_handle,
        player1: player1_texture_atlas_handle,
        player2: player2_texture_atlas_handle,
        enemies: enemies_texture_atlas_handle,
    });
}
