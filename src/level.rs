use crate::{
    common::{
        AnimationIndices, AnimationTimer, AppState, HomeDyingEvent, ENEMIES_PER_LEVEL,
        LEVEL_COLUMNS, LEVEL_ROWS, MAX_LEVELS, SPRITE_TREE_ORDER, TILE_SIZE,
    },
    enemy::{Enemy, LevelSpawnedEnemies},
    player::PlayerNo,
};
use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use bevy_rapier2d::prelude::*;

pub const LEVEL_TRANSLATION_OFFSET: Vec3 = Vec3::new(
    -LEVEL_COLUMNS as f32 / 2.0 * TILE_SIZE,
    -LEVEL_ROWS as f32 / 2. * TILE_SIZE,
    0.0,
);

// 关卡地图元素
#[derive(Component, Clone, PartialEq, Eq, Debug, Default)]
pub enum LevelItem {
    #[default]
    None,
    // 石墙
    StoneWall,
    // 贴墙
    IronWall,
    // 树木
    Tree,
    // 水
    Water,
    // 家
    Home,
}

// 关卡player1位置标记
#[derive(Component, Default)]
pub struct Player1Marker;
// 关卡player2位置标记
#[derive(Component, Default)]
pub struct Player2Marker;
// 关卡敌人位置标记
#[derive(Component, Default)]
pub struct EnemiesMarker;

#[derive(Clone, Debug, Default, Bundle)]
pub struct ColliderBundle {
    pub collider: Collider,
    pub rigid_body: RigidBody,
}

#[derive(Clone, Debug, Default, Bundle)]
pub struct AnimationBundle {
    pub timer: AnimationTimer,
    pub indices: AnimationIndices,
}

#[derive(Bundle, LdtkEntity, Default)]
pub struct StoneWallBundle {
    #[from_entity_instance]
    level_item: LevelItem,
    #[from_entity_instance]
    pub collider_bundle: ColliderBundle,
    // #[sprite_sheet_bundle("path/to/asset.png", tile_width, tile_height, columns, rows, padding, offset, index)]
    #[sprite_sheet_bundle("textures/map.bmp", 32.0, 32.0, 7, 1, 0.0, 0.0, 0)]
    sprite_bundle: SpriteSheetBundle,
}
#[derive(Bundle, LdtkEntity, Default)]
pub struct IronWallBundle {
    #[from_entity_instance]
    level_item: LevelItem,
    #[from_entity_instance]
    pub collider_bundle: ColliderBundle,
    #[sprite_sheet_bundle("textures/map.bmp", 32.0, 32.0, 7, 1, 0.0, 0.0, 1)]
    sprite_bundle: SpriteSheetBundle,
}
#[derive(Bundle, LdtkEntity, Default)]
pub struct TreeBundle {
    #[from_entity_instance]
    level_item: LevelItem,
    #[sprite_sheet_bundle("textures/map.bmp", 32.0, 32.0, 7, 1, 0.0, 0.0, 2)]
    sprite_bundle: SpriteSheetBundle,
}
#[derive(Bundle, LdtkEntity, Default)]
pub struct WaterBundle {
    #[from_entity_instance]
    level_item: LevelItem,
    #[from_entity_instance]
    pub collider_bundle: ColliderBundle,
    #[sprite_sheet_bundle("textures/map.bmp", 32.0, 32.0, 7, 1, 0.0, 0.0, 3)]
    sprite_bundle: SpriteSheetBundle,
    #[from_entity_instance]
    pub annimation_bundle: AnimationBundle,
}
#[derive(Bundle, LdtkEntity, Default)]
pub struct HomeBundle {
    #[from_entity_instance]
    level_item: LevelItem,
    #[from_entity_instance]
    pub collider_bundle: ColliderBundle,
    #[sprite_sheet_bundle("textures/map.bmp", 32.0, 32.0, 7, 1, 0.0, 0.0, 5)]
    sprite_bundle: SpriteSheetBundle,
}

#[derive(Bundle, LdtkEntity, Default)]
pub struct Player1MarkerBundle {
    marker: Player1Marker,
}
#[derive(Bundle, LdtkEntity, Default)]
pub struct Player2MarkerBundle {
    marker: Player2Marker,
    #[sprite_sheet_bundle]
    sprite_bundle: SpriteSheetBundle,
}
#[derive(Bundle, LdtkEntity, Default)]
pub struct EnemiesMarkerBundle {
    marker: EnemiesMarker,
    #[sprite_sheet_bundle]
    sprite_bundle: SpriteSheetBundle,
}

impl From<&EntityInstance> for ColliderBundle {
    fn from(entity_instance: &EntityInstance) -> ColliderBundle {
        match entity_instance.identifier.as_ref() {
            "StoneWall" | "IronWall" | "Water" | "Home" => ColliderBundle {
                collider: Collider::cuboid(TILE_SIZE / 2., TILE_SIZE / 2.),
                rigid_body: RigidBody::Fixed,
            },
            _ => ColliderBundle::default(),
        }
    }
}
impl From<&EntityInstance> for AnimationBundle {
    fn from(entity_instance: &EntityInstance) -> AnimationBundle {
        match entity_instance.identifier.as_ref() {
            "Water" => AnimationBundle {
                timer: AnimationTimer(Timer::from_seconds(0.2, TimerMode::Repeating)),
                indices: AnimationIndices { first: 3, last: 4 },
            },
            _ => AnimationBundle::default(),
        }
    }
}
impl From<&EntityInstance> for LevelItem {
    fn from(entity_instance: &EntityInstance) -> LevelItem {
        match entity_instance.identifier.as_ref() {
            "StoneWall" => LevelItem::StoneWall,
            "IronWall" => LevelItem::IronWall,
            "Tree" => LevelItem::Tree,
            "Water" => LevelItem::Water,
            "Home" => LevelItem::Home,
            _ => LevelItem::None,
        }
    }
}

pub fn setup_levels(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    q_ldtk_world: Query<(), With<Handle<LdtkProject>>>,
) {
    if q_ldtk_world.iter().len() > 0 {
        // 从Paused状态进入时无需再load ldtk
        return;
    }
    commands.spawn(LdtkWorldBundle {
        ldtk_handle: asset_server.load("levels.ldtk"),
        transform: Transform::from_translation(Vec3::ZERO + LEVEL_TRANSLATION_OFFSET),
        ..Default::default()
    });
}

pub fn spawn_ldtk_entity(
    mut commands: Commands,
    entity_query: Query<(Entity, &Transform, &EntityInstance), Added<EntityInstance>>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    asset_server: Res<AssetServer>,
) {
    for (_entity, transform, entity_instance) in entity_query.iter() {
        if entity_instance.identifier == *"Tree" {
            let map_texture_handle = asset_server.load("textures/map.bmp");
            let map_texture_atlas = TextureAtlas::from_grid(
                map_texture_handle,
                Vec2::new(32.0, 32.0),
                7,
                1,
                None,
                None,
            );
            let map_texture_atlas_handle = texture_atlases.add(map_texture_atlas);

            let mut translation = transform.translation + LEVEL_TRANSLATION_OFFSET;
            translation.z = SPRITE_TREE_ORDER;
            commands.spawn((
                LevelItem::Tree,
                SpriteSheetBundle {
                    texture_atlas: map_texture_atlas_handle,
                    sprite: TextureAtlasSprite {
                        index: 2,
                        ..default()
                    },
                    transform: Transform::from_translation(translation),
                    ..default()
                },
            ));
        }
    }
}

// 水动画播放
pub fn animate_water(
    time: Res<Time>,
    mut query: Query<(
        &LevelItem,
        &mut AnimationTimer,
        &AnimationIndices,
        &mut TextureAtlasSprite,
    )>,
) {
    for (level_item, mut timer, indices, mut sprite) in &mut query {
        if *level_item == LevelItem::Water {
            timer.0.tick(time.delta());
            if timer.0.just_finished() {
                // 切换到下一个sprite
                sprite.index = if sprite.index == indices.last {
                    indices.first
                } else {
                    sprite.index + 1
                };
            }
        }
    }
}

pub fn auto_switch_level(
    mut commands: Commands,
    q_enemies: Query<(), With<Enemy>>,
    q_players: Query<Entity, With<PlayerNo>>,
    q_level_items: Query<Entity, With<LevelItem>>,
    mut level_selection: ResMut<LevelSelection>,
    mut level_spawned_enemies: ResMut<LevelSpawnedEnemies>,
    mut app_state: ResMut<NextState<AppState>>,
) {
    // 已生成的敌人数量达到最大值 并且 敌人全部阵亡，切换到下一关卡
    if level_spawned_enemies.0 == ENEMIES_PER_LEVEL && q_enemies.iter().len() == 0 {
        if let LevelSelection::Indices(LevelIndices { level, .. }) = *level_selection {
            if level as i32 == MAX_LEVELS - 1 {
                // TODO 游戏胜利
                info!("win the game!");
                app_state.set(AppState::StartMenu);
            } else {
                // 下一关卡
                info!("Switch to next level, index={}", level + 1);
                *level_selection = LevelSelection::index(level + 1);
                level_spawned_enemies.0 = 0;

                // 重新生成玩家
                for player in &q_players {
                    commands.entity(player).despawn_recursive();
                }
                for level_item in &q_level_items {
                    commands.entity(level_item).despawn_recursive();
                }
            }
        }
    }
}

pub fn animate_home(
    mut home_dying_er: EventReader<HomeDyingEvent>,
    mut q_level_items: Query<(&LevelItem, &mut TextureAtlasSprite)>,
    mut app_state: ResMut<NextState<AppState>>,
) {
    for _ in home_dying_er.read() {
        for (level_item, mut sprite) in &mut q_level_items {
            if *level_item == LevelItem::Home {
                sprite.index = 6;
                app_state.set(AppState::GameOver);
            }
        }
    }
}

pub fn cleanup_level_items(mut commands: Commands, q_level_items: Query<Entity, With<LevelItem>>) {
    for entity in &q_level_items {
        commands.entity(entity).despawn_recursive();
    }
}

pub fn cleanup_ldtk_world(
    mut commands: Commands,
    q_ldtk_world: Query<Entity, With<Handle<LdtkProject>>>,
) {
    for entity in &q_ldtk_world {
        commands.entity(entity).despawn_recursive();
    }
}

pub fn reset_level_selection(mut level_selection: ResMut<LevelSelection>) {
    *level_selection = LevelSelection::index(0);
}
