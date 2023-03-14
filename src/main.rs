mod area;
mod bullet;
mod common;
mod enemy;
mod level;
mod player;
mod ui;

use area::*;
use bullet::*;
use common::*;
use enemy::*;
use level::*;
use player::*;
use ui::*;

use bevy::{prelude::*, time::FixedTimestep};
use bevy_ecs_ldtk::prelude::*;
use bevy_inspector_egui::{prelude::*, quick::WorldInspectorPlugin};
use bevy_rapier2d::prelude::*;

const BACKGROUND_COLOR: Color = Color::BLACK;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
        .add_plugin(RapierDebugRenderPlugin::default())
        .add_plugin(LdtkPlugin)
        .add_plugin(WorldInspectorPlugin)
        .add_event::<ExplosionEvent>()
        .add_event::<SpawnPlayerEvent>()
        .add_state(AppState::StartMenu)
        .insert_resource(ClearColor(BACKGROUND_COLOR))
        .insert_resource(GameMode::SinglePlayer)
        .insert_resource(LevelSelection::Index(0))
        .insert_resource(LevelSpawnedEnemies(0))
        .register_ldtk_entity::<level::StoneWallBundle>("StoneWall")
        .register_ldtk_entity::<level::IronWallBundle>("IronWall")
        .register_ldtk_entity::<level::TreeBundle>("Tree")
        .register_ldtk_entity::<level::WaterBundle>("Water")
        .register_ldtk_entity::<level::HomeBundle>("Home")
        .register_ldtk_entity::<level::Player1MarkerBundle>("Player1")
        .register_ldtk_entity::<level::Player2MarkerBundle>("Player2")
        .register_ldtk_entity::<level::EnemiesMarkerBundle>("Enemies")
        .add_startup_system(setup_camera)
        .add_startup_system(setup_rapier)
        .add_startup_system(setup_wall)
        .add_startup_system(setup_explosion_assets)
        .add_system_set(SystemSet::on_enter(AppState::StartMenu).with_system(setup_start_menu))
        .add_system_set(
            SystemSet::on_update(AppState::StartMenu)
                .with_system(start_game)
                .with_system(switch_game_mode),
        )
        .add_system_set(
            SystemSet::on_exit(AppState::StartMenu)
                .with_system(despawn_screen::<OnStartMenuScreen>),
        )
        .add_system_set(SystemSet::on_enter(AppState::Playing).with_system(setup_levels))
        .add_system_set(
            SystemSet::on_update(AppState::Playing)
                .with_system(auto_spawn_player1)
                .with_system(auto_spawn_player2)
                .with_system(players_attack)
                .with_system(animate_players)
                .with_system(animate_shield)
                .with_system(animate_born)
                .with_system(remove_shield)
                .with_system(animate_water)
                .with_system(spawn_explosion)
                .with_system(animate_explosion)
                .with_system(handle_bullet_collision)
                .with_system(auto_switch_level)
                .with_system(auto_spawn_enemies)
                .with_system(animate_enemies)
        )
        .add_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::step(TIME_STEP as f64))
                .with_system(player1_move)
                .with_system(player2_move)
                .with_system(move_bullet),
        )
        .add_system_to_stage(CoreStage::PostUpdate, display_events)
        .add_system(bevy::window::close_on_esc)
        .run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn setup_rapier(mut rapier_config: ResMut<RapierConfiguration>) {
    rapier_config.gravity = Vec2::ZERO;
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
