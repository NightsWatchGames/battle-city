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
        .add_state::<AppState>()
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
        .add_system(setup_start_menu.in_schedule(OnEnter(AppState::StartMenu)))
        .add_system(cleanup_level_items.in_schedule(OnEnter(AppState::StartMenu)))
        .add_system(cleanup_ldtk_world.in_schedule(OnEnter(AppState::StartMenu)))
        .add_system(cleanup_players.in_schedule(OnEnter(AppState::StartMenu)))
        .add_system(cleanup_born.in_schedule(OnEnter(AppState::StartMenu)))
        .add_system(cleanup_bullets.in_schedule(OnEnter(AppState::StartMenu)))
        .add_system(cleanup_explosions.in_schedule(OnEnter(AppState::StartMenu)))
        .add_system(cleanup_enemies.in_schedule(OnEnter(AppState::StartMenu)))
        .add_system(reset_player_lives.in_schedule(OnEnter(AppState::StartMenu)))
        .add_system(reset_level_selection.in_schedule(OnEnter(AppState::StartMenu)))
        .add_system(reset_level_spawned_enemies.in_schedule(OnEnter(AppState::StartMenu)))
        .add_system(reset_multiplayer_mode.in_schedule(OnEnter(AppState::StartMenu)))
        .add_system(start_game.in_set(OnUpdate(AppState::StartMenu)))
        .add_system(switch_multiplayer_mode.in_set(OnUpdate(AppState::StartMenu)))
        .add_system(despawn_screen::<OnStartMenuScreen>.in_schedule(OnExit(AppState::StartMenu)))
        .add_system(setup_levels.in_schedule(OnEnter(AppState::Playing)))
        .add_system(spawn_ldtk_entity.in_set(OnUpdate(AppState::Playing)))
        .add_system(auto_spawn_players.in_set(OnUpdate(AppState::Playing)))
        .add_system(players_move.in_set(OnUpdate(AppState::Playing)))
        .add_system(players_attack.in_set(OnUpdate(AppState::Playing)))
        .add_system(animate_players.in_set(OnUpdate(AppState::Playing)))
        .add_system(animate_shield.in_set(OnUpdate(AppState::Playing)))
        .add_system(animate_born.in_set(OnUpdate(AppState::Playing)))
        .add_system(remove_shield.in_set(OnUpdate(AppState::Playing)))
        .add_system(animate_water.in_set(OnUpdate(AppState::Playing)))
        .add_system(animate_home.in_set(OnUpdate(AppState::Playing)))
        .add_system(spawn_explosion.in_set(OnUpdate(AppState::Playing)))
        .add_system(animate_explosion.in_set(OnUpdate(AppState::Playing)))
        .add_system(handle_bullet_collision.in_set(OnUpdate(AppState::Playing)))
        .add_system(auto_switch_level.in_set(OnUpdate(AppState::Playing)))
        .add_system(auto_spawn_enemies.in_set(OnUpdate(AppState::Playing)))
        .add_system(animate_enemies.in_set(OnUpdate(AppState::Playing)))
        .add_system(enemies_attack.in_set(OnUpdate(AppState::Playing)))
        .add_system(enemies_move.in_set(OnUpdate(AppState::Playing)))
        .add_system(handle_enemy_collision.in_set(OnUpdate(AppState::Playing)))
        .add_system(move_bullet.in_set(OnUpdate(AppState::Playing)))
        .add_system(pause_game.in_set(OnUpdate(AppState::Playing)))
        .add_system(unpause_game.in_set(OnUpdate(AppState::Paused)))
        .add_system(setup_game_over.in_schedule(OnEnter(AppState::GameOver)))
        .add_system(animate_game_over.in_set(OnUpdate(AppState::GameOver)))
        .add_system(animate_players.in_set(OnUpdate(AppState::GameOver)))
        .add_system(animate_shield.in_set(OnUpdate(AppState::GameOver)))
        .add_system(animate_water.in_set(OnUpdate(AppState::GameOver)))
        .add_system(animate_home.in_set(OnUpdate(AppState::GameOver)))
        .add_system(animate_explosion.in_set(OnUpdate(AppState::GameOver)))
        .add_system(animate_enemies.in_set(OnUpdate(AppState::GameOver)))
        .add_system(despawn_screen::<OnGameOverScreen>.in_schedule(OnExit(AppState::GameOver)))
        .run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn setup_rapier(mut rapier_config: ResMut<RapierConfiguration>) {
    rapier_config.gravity = Vec2::ZERO;
}
