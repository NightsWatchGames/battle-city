use crate::common::AnimationTimer;
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

// 地图项
#[derive(Component, Clone, PartialEq)]
pub enum MapItem {
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

pub fn spawn_map_item(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    texture_atlases: &mut ResMut<Assets<TextureAtlas>>,
    translation: Vec3,
    map_item: MapItem,
) {
    let map_texture_handle = asset_server.load("textures/map.bmp");
    let map_texture_atlas =
        TextureAtlas::from_grid(map_texture_handle, Vec2::new(32.0, 32.0), 7, 1, None, None);
    let map_texture_atlas_handle = texture_atlases.add(map_texture_atlas);

    let map_item_entity = commands
        .spawn(SpriteSheetBundle {
            texture_atlas: map_texture_atlas_handle,
            sprite: TextureAtlasSprite {
                index: match map_item {
                    MapItem::StoneWall => 0,
                    MapItem::IronWall => 1,
                    MapItem::Tree => 2,
                    MapItem::Water => 3,
                    MapItem::Home => 5,
                },
                ..default()
            },
            transform: Transform {
                translation: Vec3::new(translation.x, translation.y, translation.z),
                ..default()
            },
            ..default()
        })
        .insert(map_item.clone())
        .id();

    if map_item == MapItem::Water {
        commands
            .entity(map_item_entity)
            .insert(AnimationTimer(Timer::from_seconds(
                0.5,
                TimerMode::Repeating,
            )));
    }
    if map_item == MapItem::IronWall || map_item == MapItem::Home {
        commands
            .entity(map_item_entity)
            .insert(Collider::cuboid(18.0, 18.0))
            .insert(RigidBody::Fixed);
    }
}

// 水动画播放
pub fn animate_water(
    time: Res<Time>,
    mut query: Query<(&mut AnimationTimer, &mut TextureAtlasSprite, &MapItem)>,
) {
    for (mut timer, mut sprite, map_item) in &mut query {
        timer.tick(time.delta());
        if timer.just_finished() {
            match map_item {
                MapItem::Water => {
                    // 切换到下一个sprite
                    sprite.index = if sprite.index == 3 { 4 } else { 3 };
                }
                _ => {}
            }
        }
    }
}
