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
use bevy_rapier2d::prelude::*;

const BACKGROUND_COLOR: Color = Color::BLACK;

// TODO 坦克碰撞导致被迫移动
fn main() {
    App::new()
        .register_type::<PlayerNo>()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
        // .add_plugin(RapierDebugRenderPlugin::default())
        .add_plugins(LdtkPlugin)
        .add_event::<ExplosionEvent>()
        .add_event::<SpawnPlayerEvent>()
        .add_event::<HomeDyingEvent>()
        .add_state::<AppState>()
        .insert_resource(ClearColor(BACKGROUND_COLOR))
        .insert_resource(MultiplayerMode::SinglePlayer)
        .insert_resource(LevelSelection::index(0))
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
        .add_systems(
            Startup,
            (
                setup_camera,
                setup_rapier,
                setup_wall,
                setup_explosion_assets,
                setup_game_sounds,
                setup_game_texture_atlas,
            ),
        )
        .add_systems(
            OnEnter(AppState::StartMenu),
            (
                setup_start_menu,
                cleanup_level_items,
                cleanup_ldtk_world,
                cleanup_players,
                cleanup_born,
                cleanup_bullets,
                cleanup_explosions,
                cleanup_enemies,
                reset_player_lives,
                reset_level_selection,
                reset_level_spawned_enemies,
                reset_multiplayer_mode,
            ),
        )
        .add_systems(
            Update,
            (start_game, switch_multiplayer_mode).run_if(in_state(AppState::StartMenu)),
        )
        .add_systems(
            OnExit(AppState::StartMenu),
            (despawn_screen::<OnStartMenuScreen>,),
        )
        .add_systems(OnEnter(AppState::Playing), (setup_levels,))
        .add_systems(
            Update,
            (
                spawn_ldtk_entity,
                auto_spawn_players,
                players_move,
                players_attack,
                animate_players,
                animate_shield,
                animate_born,
                remove_shield,
                animate_water,
                animate_home,
                spawn_explosion,
                animate_explosion,
                handle_bullet_collision,
                auto_switch_level,
                (
                    auto_spawn_enemies,
                    animate_enemies,
                    enemies_attack,
                    enemies_move,
                    handle_enemy_collision,
                    move_bullet,
                    pause_game,
                ),
            )
                .run_if(in_state(AppState::Playing)),
        )
        .add_systems(Update, (unpause_game,).run_if(in_state(AppState::Paused)))
        .add_systems(OnEnter(AppState::GameOver), (setup_game_over,))
        .add_systems(
            Update,
            (
                animate_game_over,
                animate_players,
                animate_shield,
                animate_water,
                animate_home,
                animate_explosion,
                animate_enemies,
            )
                .run_if(in_state(AppState::GameOver)),
        )
        .add_systems(
            OnExit(AppState::GameOver),
            (despawn_screen::<OnGameOverScreen>,),
        )
        .run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn setup_rapier(mut rapier_config: ResMut<RapierConfiguration>) {
    rapier_config.gravity = Vec2::ZERO;
}
