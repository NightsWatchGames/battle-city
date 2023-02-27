use crate::common::AnimationTimer;
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

// 关卡项
#[derive(Component, Clone, PartialEq, Debug)]
pub enum LevelItem {
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
        commands
            .entity(level_item_entity)
            .insert(AnimationTimer(Timer::from_seconds(
                0.5,
                TimerMode::Repeating,
            )));
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
    mut query: Query<(&mut AnimationTimer, &mut TextureAtlasSprite, &LevelItem)>,
) {
    for (mut timer, mut sprite, level_item) in &mut query {
        timer.tick(time.delta());
        if timer.just_finished() {
            match level_item {
                LevelItem::Water => {
                    // 切换到下一个sprite
                    sprite.index = if sprite.index == 3 { 4 } else { 3 };
                }
                _ => {}
            }
        }
    }
}
