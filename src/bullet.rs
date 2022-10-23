use crate::common;
use crate::common::*;
use crate::wall::*;

use bevy::prelude::*;

pub const BULLET_SPEED: f32 = 300.0;

#[derive(Component)]
pub struct Bullet;

pub fn spawn_bullet(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    texture_atlases: &mut ResMut<Assets<TextureAtlas>>,
    direction: common::Direction,
    transform: &Transform
) {
    let bullet_texture_handle = asset_server.load("textures/bullet.bmp");
    let bullet_texture_atlas =
        TextureAtlas::from_grid(bullet_texture_handle, Vec2::new(7.0, 8.0), 1, 4, None, None);
    let bullet_texture_atlas_handle = texture_atlases.add(bullet_texture_atlas);

    commands
        .spawn(Bullet)
        .insert(SpriteSheetBundle {
            texture_atlas: bullet_texture_atlas_handle,
            sprite: TextureAtlasSprite { 
                index: match direction {
                    common::Direction::Up => { 0 },
                    common::Direction::Right => { 1 },
                    common::Direction::Down => { 2 },
                    common::Direction::Left => { 3 },
                },
                ..default()
            },
            transform: Transform {
                translation: Vec3::new(transform.translation.x, transform.translation.y, transform.translation.z),
                ..default()
            },
            ..default()
        })
        .insert(direction);
}

// 炮弹移动
pub fn bullet_move_system(
    mut transform_query: Query<(&mut Transform, &common::Direction), With<Bullet>>,
) {
    for (mut bullet_transform, direction) in &mut transform_query {
        match direction {
            common::Direction::Left => {
                bullet_transform.translation.x -= 1.0 * BULLET_SPEED * TIME_STEP
            }
            common::Direction::Right => {
                bullet_transform.translation.x += 1.0 * BULLET_SPEED * TIME_STEP
            }
            common::Direction::Up => {
                bullet_transform.translation.y += 1.0 * BULLET_SPEED * TIME_STEP
            }
            common::Direction::Down => {
                bullet_transform.translation.y -= 1.0 * BULLET_SPEED * TIME_STEP
            }
        }
    }
}

// TODO 子弹撞墙 释放内存