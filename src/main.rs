use bevy::{
    prelude::*,
    sprite::collide_aabb::{collide, Collision},
    time::FixedTimestep,
};

// 物理帧间隔
const TIME_STEP: f32 = 1.0 / 60.0;

const TANK_SIZE: Vec3 = Vec3::new(80.0, 80.0, 0.0);
const TANK_SPEED: f32 = 500.0;
// 坦克离墙最近距离限制
const TANK_PADDING: f32 = 10.0;

const WALL_THICKNESS: f32 = 10.0;
// 墙壁x轴和y轴坐标
const LEFT_WALL: f32 = -450.;
const RIGHT_WALL: f32 = 450.;
const BOTTOM_WALL: f32 = -300.;
const TOP_WALL: f32 = 300.;

const BACKGROUND_COLOR: Color = Color::rgb(0.9, 0.9, 0.9);

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(ClearColor(BACKGROUND_COLOR))
        .add_startup_system(setup)
        .add_system(move_tank)
        .add_system(bevy::window::close_on_esc)
        .run();
}

// 坦克
#[derive(Component)]
struct Tank;

// 速度
#[derive(Component, Deref, DerefMut)]
struct Velocity(Vec2);

// 碰撞体
#[derive(Component)]
struct Collider;

// 墙壁bundle
#[derive(Bundle)]
struct WallBundle {
    #[bundle]
    sprite_bundle: SpriteBundle,  // 嵌套bundle
    collider: Collider,
}

// 墙壁位置
enum WallLocation {
    Left,
    Right,
    Bottom,
    Top,
}

impl WallLocation {
    fn position(&self) -> Vec2 {
        match self {
            WallLocation::Left => Vec2::new(LEFT_WALL, 0.),
            WallLocation::Right => Vec2::new(RIGHT_WALL, 0.),
            WallLocation::Bottom => Vec2::new(0., BOTTOM_WALL),
            WallLocation::Top => Vec2::new(0., TOP_WALL),
        }
    }

    fn size(&self) -> Vec2 {
        let arena_height = TOP_WALL - BOTTOM_WALL;
        let arena_width = RIGHT_WALL - LEFT_WALL;
        // Make sure we haven't messed up our constants
        assert!(arena_height > 0.0);
        assert!(arena_width > 0.0);

        match self {
            WallLocation::Left | WallLocation::Right => {
                Vec2::new(WALL_THICKNESS, arena_height + WALL_THICKNESS)
            }
            WallLocation::Bottom | WallLocation::Top => {
                Vec2::new(arena_width + WALL_THICKNESS, WALL_THICKNESS)
            }
        }
    }
}

impl WallBundle {
    fn new(location: WallLocation) -> WallBundle {
        WallBundle {
            sprite_bundle: SpriteBundle {
                transform: Transform {
                    translation: location.position().extend(0.0),  // z轴坐标0
                    scale: location.size().extend(1.0),  // https://github.com/bevyengine/bevy/issues/4149
                    ..default()
                },
                sprite: Sprite {
                    color: Color::rgb(0.8, 0.8, 0.8),
                    ..default()
                },
                ..default()
            },
            collider: Collider,
        }
    }
}

// setup系统 添加entities到世界
fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    // 相机
    commands.spawn_bundle(Camera2dBundle::default());

    // 坦克
    let tank_y = BOTTOM_WALL + 100.0;
    commands
        .spawn()
        .insert(Tank)
        .insert_bundle(SpriteBundle {
            transform: Transform {
                translation: Vec3::new(0.0, tank_y, 0.0),
                scale: TANK_SIZE,
                ..default()
            },
            sprite: Sprite {
                color: Color::rgb(0.3, 0.3, 0.7),
                ..default()
            },
            ..default()
        })
        .insert(Collider);

    // 墙壁
    commands.spawn_bundle(WallBundle::new(WallLocation::Left));
    commands.spawn_bundle(WallBundle::new(WallLocation::Right));
    commands.spawn_bundle(WallBundle::new(WallLocation::Bottom));
    commands.spawn_bundle(WallBundle::new(WallLocation::Top));
}

// 移动坦克
fn move_tank(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<&mut Transform, With<Tank>>,
) {
    let mut tank_transform = query.single_mut();
    // x轴移动
    let mut x_direction = 0.0;
    // y轴移动
    let mut y_direction = 0.0;

    // 一次只能移动一个方向
    if keyboard_input.pressed(KeyCode::Left) {
        x_direction -= 1.0;
    } else if keyboard_input.pressed(KeyCode::Right) {
        x_direction += 1.0;
    } else if keyboard_input.pressed(KeyCode::Up) {
        y_direction += 1.0;
    } else if keyboard_input.pressed(KeyCode::Down) {
        y_direction -= 1.0;
    }

    // 根据速度时间计算新坐标
    let new_tank_x_position = tank_transform.translation.x + x_direction * TANK_SPEED * TIME_STEP;
    let new_tank_y_position = tank_transform.translation.y + y_direction * TANK_SPEED * TIME_STEP;

    // 区域边界，确保坦克不会超出边界
    let left_bound = LEFT_WALL + WALL_THICKNESS / 2.0 + TANK_SIZE.x / 2.0 + TANK_PADDING;
    let right_bound = RIGHT_WALL - WALL_THICKNESS / 2.0 - TANK_SIZE.x / 2.0 - TANK_PADDING;
    let bottom_bound = BOTTOM_WALL + WALL_THICKNESS / 2.0 + TANK_SIZE.x / 2.0 + TANK_PADDING;
    let top_bound = TOP_WALL - WALL_THICKNESS / 2.0 - TANK_SIZE.x / 2.0 - TANK_PADDING;
    tank_transform.translation.x = new_tank_x_position.clamp(left_bound, right_bound);
    tank_transform.translation.y = new_tank_y_position.clamp(bottom_bound, top_bound);
}