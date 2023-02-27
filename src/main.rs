mod bullet;
mod common;
mod level;
mod player;
mod wall;
mod enemy;
mod ui;

use bullet::*;
use common::*;
use level::*;
use player::*;
use wall::*;
use enemy::*;
use ui::*;

use bevy::{prelude::*, time::FixedTimestep};
use bevy_ecs_tilemap::prelude::*;
use bevy_inspector_egui::{prelude::*, quick::WorldInspectorPlugin};
use bevy_rapier2d::prelude::*;

const BACKGROUND_COLOR: Color = Color::BLACK;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(TilemapPlugin)
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
        .add_plugin(RapierDebugRenderPlugin::default())
        .add_plugin(WorldInspectorPlugin)
        .insert_resource(ClearColor(BACKGROUND_COLOR))
        .add_startup_system(setup_camera)
        .add_startup_system(setup_rapier)
        .add_startup_system(setup_wall)
        .add_startup_system(setup_player1)
        .add_startup_system(setup_player2)
        .add_startup_system(setup_enemies)
        .add_startup_system(setup)
        .add_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::step(TIME_STEP as f64))
                .with_system(player1_move)
                .with_system(player2_move)
                .with_system(move_bullet),
        )
        .add_system(players_attack)
        .add_system(animate_players)
        .add_system(animate_shield)
        .add_system(remove_shield)
        .add_system(animate_water)
        .add_system_to_stage(CoreStage::PostUpdate, display_events)
        .add_system_to_stage(CoreStage::PostUpdate, check_bullet_collision)
        .add_system(bevy::window::close_on_esc)
        .run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn setup_rapier(mut rapier_config: ResMut<RapierConfiguration>) {
    rapier_config.gravity = Vec2::ZERO;
}

// setup系统 添加entities到世界
fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    // 地图项
    spawn_level_item(
        &mut commands,
        &asset_server,
        &mut texture_atlases,
        Vec3::new(0.0, BOTTOM_WALL + 300.0, 0.0),
        LevelItem::Home,
    );
    spawn_level_item(
        &mut commands,
        &asset_server,
        &mut texture_atlases,
        Vec3::new(0.0, BOTTOM_WALL + 350.0, 0.0),
        LevelItem::Tree,
    );
    spawn_level_item(
        &mut commands,
        &asset_server,
        &mut texture_atlases,
        Vec3::new(0.0, BOTTOM_WALL + 400.0, 0.0),
        LevelItem::Water,
    );
    spawn_level_item(
        &mut commands,
        &asset_server,
        &mut texture_atlases,
        Vec3::new(0.0, BOTTOM_WALL + 450.0, 0.0),
        LevelItem::IronWall,
    );

    commands
        .spawn(TransformBundle::from(Transform::from_xyz(
            200.0, 100.0, 0.0,
        )))
        .insert(RigidBody::Fixed)
        .insert(Collider::cuboid(80.0, 30.0));
}

fn display_events(
    mut collision_events: EventReader<CollisionEvent>,
    mut contact_force_events: EventReader<ContactForceEvent>,
) {
    for _collision_event in collision_events.iter() {
        // println!("Received collision event: {:?}", collision_event);
    }

    for _contact_force_event in contact_force_events.iter() {
        // println!("Received contact force event: {:?}", contact_force_event);
    }
}
