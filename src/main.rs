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

use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_rapier2d::prelude::*;

const BACKGROUND_COLOR: Color = Color::BLACK;

// TODO 坦克碰撞导致被迫移动
fn main() {
    App::new()
        .register_type::<PlayerNo>()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
        // .add_plugin(RapierDebugRenderPlugin::default())
        .add_plugin(LdtkPlugin)
        // .add_plugin(WorldInspectorPlugin)
        .add_event::<ExplosionEvent>()
        .add_event::<SpawnPlayerEvent>()
        .add_event::<HomeDyingEvent>()
        .add_state(AppState::StartMenu)
        .insert_resource(ClearColor(BACKGROUND_COLOR))
        .insert_resource(MultiplayerMode::SinglePlayer)
        .insert_resource(LevelSelection::Index(0))
        .insert_resource(LevelSpawnedEnemies(0))
        .insert_resource(PlayerLives {
            player1: 3,
            player2: 3,
        })
        .register_ldtk_entity::<level::StoneWallBundle>("StoneWall")
        .register_ldtk_entity::<level::IronWallBundle>("IronWall")
        .register_ldtk_entity::<level::WaterBundle>("Water")
        .register_ldtk_entity::<level::HomeBundle>("Home")
        .register_ldtk_entity::<level::Player1MarkerBundle>("Player1")
        .register_ldtk_entity::<level::Player2MarkerBundle>("Player2")
        .register_ldtk_entity::<level::EnemiesMarkerBundle>("Enemies")
        .add_startup_system(setup_camera)
        .add_startup_system(setup_rapier)
        .add_startup_system(setup_wall)
        .add_startup_system(setup_explosion_assets)
        .add_startup_system(setup_game_sounds)
        .add_startup_system(setup_game_texture_atlas)
        .add_system_set(
            SystemSet::on_enter(AppState::StartMenu)
                .with_system(setup_start_menu)
                .with_system(cleanup_level_items)
                .with_system(cleanup_ldtk_world)
                .with_system(cleanup_players)
                .with_system(cleanup_born)
                .with_system(cleanup_bullets)
                .with_system(cleanup_explosions)
                .with_system(cleanup_enemies)
                .with_system(reset_player_lives)
                .with_system(reset_level_selection)
                .with_system(reset_level_spawned_enemies)
                .with_system(reset_multiplayer_mode),
        )
        .add_system_set(
            SystemSet::on_update(AppState::StartMenu)
                .with_system(start_game)
                .with_system(switch_multiplayer_mode),
        )
        .add_system_set(
            SystemSet::on_exit(AppState::StartMenu)
                .with_system(despawn_screen::<OnStartMenuScreen>),
        )
        .add_system_set(SystemSet::on_enter(AppState::Playing).with_system(setup_levels))
        .add_system_set(
            SystemSet::on_update(AppState::Playing)
                .with_system(spawn_ldtk_entity)
                .with_system(auto_spawn_players)
                .with_system(players_move)
                .with_system(players_attack)
                .with_system(animate_players)
                .with_system(animate_shield)
                .with_system(animate_born)
                .with_system(remove_shield)
                .with_system(animate_water)
                .with_system(animate_home)
                .with_system(spawn_explosion)
                .with_system(animate_explosion)
                .with_system(handle_bullet_collision)
                .with_system(auto_switch_level)
                .with_system(auto_spawn_enemies)
                .with_system(animate_enemies)
                .with_system(enemies_attack)
                .with_system(enemies_move)
                .with_system(handle_enemy_collision)
                .with_system(move_bullet)
                .with_system(pause_game),
        )
        .add_system_set(SystemSet::on_update(AppState::Paused).with_system(unpause_game))
        .add_system_set(SystemSet::on_enter(AppState::GameOver).with_system(setup_game_over))
        .add_system_set(
            SystemSet::on_update(AppState::GameOver)
                .with_system(animate_game_over)
                .with_system(animate_players)
                .with_system(animate_shield)
                .with_system(animate_water)
                .with_system(animate_home)
                .with_system(animate_explosion)
                .with_system(animate_enemies),
        )
        .add_system_set(
            SystemSet::on_exit(AppState::GameOver).with_system(despawn_screen::<OnGameOverScreen>),
        )
        .run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn setup_rapier(mut rapier_config: ResMut<RapierConfiguration>) {
    rapier_config.gravity = Vec2::ZERO;
}
