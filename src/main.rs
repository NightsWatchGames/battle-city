mod bullet;
mod common;
mod map;
mod tank;
mod wall;

use bullet::*;
use common::{AnimationTimer, TIME_STEP};
use map::*;
use tank::*;
use wall::*;

use bevy::{prelude::*, time::FixedTimestep};
use bevy_ecs_tilemap::prelude::*;
use bevy_rapier2d::prelude::*;
use bevy_inspector_egui::prelude::*;

const BACKGROUND_COLOR: Color = Color::BLACK;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(TilemapPlugin)
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
        .add_plugin(RapierDebugRenderPlugin::default())
        .add_plugin(WorldInspectorPlugin::new())
        .insert_resource(ClearColor(BACKGROUND_COLOR))
        .add_startup_system(setup)
        .add_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::step(TIME_STEP as f64))
                .with_system(move_tank)
                .with_system(move_bullet),
        )
        .add_system(tank_attack)
        .add_system(animate_tank)
        .add_system(animate_shield)
        .add_system(remove_shield)
        .add_system(animate_water)
        .add_system_to_stage(CoreStage::PostUpdate, display_events)
        .add_system_to_stage(CoreStage::PostUpdate, check_tank_collision)
        .add_system_to_stage(CoreStage::PostUpdate, check_bullet_collision)
        .add_system(bevy::window::close_on_esc)
        .run();
}

// setup系统 添加entities到世界
fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    mut rapier_config: ResMut<RapierConfiguration>,
) {
    rapier_config.gravity = Vec2::ZERO;
    // 相机
    commands.spawn(Camera2dBundle::default());

    let shield_texture_handle = asset_server.load("textures/shield.bmp");
    let shield_texture_atlas =
        TextureAtlas::from_grid(shield_texture_handle, Vec2::new(30.0, 30.0), 1, 2, None, None);
    let shield_texture_atlas_handle = texture_atlases.add(shield_texture_atlas);

    let tank_texture_handle = asset_server.load("textures/tank1.bmp");
    let tank_texture_atlas =
        TextureAtlas::from_grid(tank_texture_handle, Vec2::new(28.0, 28.0), 2, 4, None, None);
    let tank_texture_atlas_handle = texture_atlases.add(tank_texture_atlas);

    // 保护盾
    let shield = commands
        .spawn(Shield)
        .insert(SpriteSheetBundle {
            texture_atlas: shield_texture_atlas_handle,
            ..default()
        })
        .insert(AnimationTimer(Timer::from_seconds(0.2, TimerMode::Repeating)))
        .insert(ShieldRemoveTimer(Timer::from_seconds(5.0, TimerMode::Once)))
        .id();

    // 坦克
    let tank = commands
        .spawn(Tank)
        .insert(SpriteSheetBundle {
            texture_atlas: tank_texture_atlas_handle,
            transform: Transform {
                translation: Vec3::new(0.0, BOTTOM_WALL + 100.0, 0.0),
                ..default()
            },
            ..default()
        })
        .insert(AnimationTimer(Timer::from_seconds(0.2, TimerMode::Repeating)))
        .insert(TankRefreshBulletTimer(Timer::from_seconds(
            TANK_REFRESH_BULLET_INTERVAL,
            TimerMode::Once,
        )))
        .insert(common::Direction::Up)
        .insert(RigidBody::Dynamic)
        .insert(Collider::cuboid(18.0, 18.0))
        .insert(ActiveEvents::COLLISION_EVENTS)
        .insert(Sensor)
        .insert(Movable{can_up: true, can_down: true, can_left: true, can_right: true})
        .id();

    commands.entity(tank).add_child(shield);

    // 墙壁
    commands
        .spawn(WallBundle::new(WallLocation::Left));
    commands
        .spawn(WallBundle::new(WallLocation::Right));
    commands
        .spawn(WallBundle::new(WallLocation::Bottom));
    commands
        .spawn(WallBundle::new(WallLocation::Top));

    // 地图项
    spawn_map_item(
        &mut commands,
        &asset_server,
        &mut texture_atlases,
        Vec3::new(0.0, BOTTOM_WALL + 300.0, 0.0),
        MapItem::Home,
    );
    spawn_map_item(
        &mut commands,
        &asset_server,
        &mut texture_atlases,
        Vec3::new(0.0, BOTTOM_WALL + 350.0, 0.0),
        MapItem::Tree,
    );
    spawn_map_item(
        &mut commands,
        &asset_server,
        &mut texture_atlases,
        Vec3::new(0.0, BOTTOM_WALL + 400.0, 0.0),
        MapItem::Water,
    );
    spawn_map_item(
        &mut commands,
        &asset_server,
        &mut texture_atlases,
        Vec3::new(0.0, BOTTOM_WALL + 450.0, 0.0),
        MapItem::IronWall,
    );

    commands
        .spawn(TransformBundle::from(Transform::from_xyz(200.0, 100.0, 0.0)))
        .insert(Sensor)
        .insert(Collider::cuboid(80.0, 30.0));
}

fn display_events(
    mut collision_events: EventReader<CollisionEvent>,
    mut contact_force_events: EventReader<ContactForceEvent>,
) {
    for collision_event in collision_events.iter() {
        // println!("Received collision event: {:?}", collision_event);
    }

    for contact_force_event in contact_force_events.iter() {
        // println!("Received contact force event: {:?}", contact_force_event);
    }
}