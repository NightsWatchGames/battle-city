use crate::{common::{AnimationIndices, AnimationTimer, LEVEL_COLUMNS, LEVEL_ROWS, TILE_SIZE, MAX_LEVELS}, enemy::LevelSpawnedEnemies};
use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use bevy_rapier2d::prelude::*;

// 关卡地图元素
#[derive(Component, Clone, PartialEq, Debug, Default)]
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

#[derive(Bundle, LdtkEntity)]
pub struct StoneWallBundle {
    #[from_entity_instance]
    level_item: LevelItem,
    #[from_entity_instance]
    #[bundle]
    pub collider_bundle: ColliderBundle,
    // #[sprite_sheet_bundle("path/to/asset.png", tile_width, tile_height, columns, rows, padding, offset, index)]
    #[sprite_sheet_bundle("textures/map.bmp", 32.0, 32.0, 7, 1, 0.0, 0.0, 0)]
    #[bundle]
    sprite_bundle: SpriteSheetBundle,
}
#[derive(Bundle, LdtkEntity)]
pub struct IronWallBundle {
    #[from_entity_instance]
    level_item: LevelItem,
    #[from_entity_instance]
    #[bundle]
    pub collider_bundle: ColliderBundle,
    #[sprite_sheet_bundle("textures/map.bmp", 32.0, 32.0, 7, 1, 0.0, 0.0, 1)]
    #[bundle]
    sprite_bundle: SpriteSheetBundle,
}
#[derive(Bundle, LdtkEntity)]
pub struct TreeBundle {
    #[from_entity_instance]
    level_item: LevelItem,
    #[sprite_sheet_bundle("textures/map.bmp", 32.0, 32.0, 7, 1, 0.0, 0.0, 2)]
    #[bundle]
    sprite_bundle: SpriteSheetBundle,
}
#[derive(Bundle, LdtkEntity)]
pub struct WaterBundle {
    #[from_entity_instance]
    level_item: LevelItem,
    #[from_entity_instance]
    #[bundle]
    pub collider_bundle: ColliderBundle,
    #[sprite_sheet_bundle("textures/map.bmp", 32.0, 32.0, 7, 1, 0.0, 0.0, 3)]
    #[bundle]
    sprite_bundle: SpriteSheetBundle,
    #[from_entity_instance]
    #[bundle]
    pub annimation_bundle: AnimationBundle,
}
#[derive(Bundle, LdtkEntity)]
pub struct HomeBundle {
    #[from_entity_instance]
    level_item: LevelItem,
    #[from_entity_instance]
    #[bundle]
    pub collider_bundle: ColliderBundle,
    #[sprite_sheet_bundle("textures/map.bmp", 32.0, 32.0, 7, 1, 0.0, 0.0, 5)]
    #[bundle]
    sprite_bundle: SpriteSheetBundle,
}

impl From<EntityInstance> for ColliderBundle {
    fn from(entity_instance: EntityInstance) -> ColliderBundle {
        match entity_instance.identifier.as_ref() {
            "StoneWall" | "IronWall" | "Water" | "Home" => ColliderBundle {
                collider: Collider::cuboid(18., 18.),
                rigid_body: RigidBody::Fixed,
            },
            _ => ColliderBundle::default(),
        }
    }
}
impl From<EntityInstance> for AnimationBundle {
    fn from(entity_instance: EntityInstance) -> AnimationBundle {
        match entity_instance.identifier.as_ref() {
            "Water" => AnimationBundle {
                timer: AnimationTimer(Timer::from_seconds(0.2, TimerMode::Repeating)),
                indices: AnimationIndices { first: 3, last: 4 },
            },
            _ => AnimationBundle::default(),
        }
    }
}
impl From<EntityInstance> for LevelItem {
    fn from(entity_instance: EntityInstance) -> LevelItem {
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

pub fn setup_levels(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(LdtkWorldBundle {
        ldtk_handle: asset_server.load("levels.ldtk"),
        transform: Transform::from_xyz(
            -LEVEL_COLUMNS as f32 / 2.0 * TILE_SIZE,
            -LEVEL_ROWS as f32 / 2. * TILE_SIZE,
            0.0,
        ),
        ..Default::default()
    });
}

pub fn spawn_level_item(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    texture_atlases: &mut ResMut<Assets<TextureAtlas>>,
    translation: Vec3,
    level_item: LevelItem,
) {
    let map_texture_handle = asset_server.load("textures/map.bmp");
    let map_texture_atlas =
        TextureAtlas::from_grid(map_texture_handle, Vec2::new(32.0, 32.0), 7, 1, None, None);
    let map_texture_atlas_handle = texture_atlases.add(map_texture_atlas);

    let level_item_entity = commands
        .spawn(SpriteSheetBundle {
            texture_atlas: map_texture_atlas_handle,
            sprite: TextureAtlasSprite {
                index: match level_item {
                    LevelItem::StoneWall => 0,
                    LevelItem::IronWall => 1,
                    LevelItem::Tree => 2,
                    LevelItem::Water => 3,
                    LevelItem::Home => 5,
                    LevelItem::None => 0,
                },
                ..default()
            },
            transform: Transform {
                translation: Vec3::new(translation.x, translation.y, translation.z),
                ..default()
            },
            ..default()
        })
        .insert(level_item.clone())
        .id();

    if level_item == LevelItem::Water {
        commands.entity(level_item_entity).insert((
            AnimationTimer(Timer::from_seconds(0.5, TimerMode::Repeating)),
            AnimationIndices { first: 3, last: 4 },
        ));
    }
    if level_item == LevelItem::IronWall
        || level_item == LevelItem::Home
        || level_item == LevelItem::StoneWall
    {
        commands
            .entity(level_item_entity)
            .insert(Collider::cuboid(18.0, 18.0))
            .insert(RigidBody::Fixed);
    }
}

// 水动画播放
pub fn animate_water(
    time: Res<Time>,
    mut query: Query<(
        &mut AnimationTimer,
        &AnimationIndices,
        &mut TextureAtlasSprite,
    )>,
) {
    for (mut timer, indices, mut sprite) in &mut query {
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

pub fn auto_switch_level(
    mut levels: ResMut<LevelSelection>,
    mut spawned_enmies: ResMut<LevelSpawnedEnemies>,
) {
    if spawned_enmies.0 > 5 {
        if let LevelSelection::Index(index) = *levels {
            if index as i32 == MAX_LEVELS - 1 {
                // 游戏胜利
                info!("win the game!");
            } else {
                // 下一关卡
                *levels = LevelSelection::Index(index + 1);
                spawned_enmies.0 = 0;
            }
        }
    }
}