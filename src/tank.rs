use crate::wall::*;
use crate::common::*;

use bevy::{
    prelude::*,
    sprite::collide_aabb::{collide, Collision},
};

pub const TANK_SIZE: Vec2 = Vec2::new(80.0, 80.0);
pub const TANK_SPEED: f32 = 500.0;
// 坦克离墙最近距离限制
pub const TANK_PADDING: f32 = 10.0;

// 坦克
#[derive(Component)]
pub struct Tank;

// 出生保护盾
#[derive(Component)]
pub struct Shield;

// 移动坦克
pub fn move_tank_system(
    keyboard_input: Res<Input<KeyCode>>,
    mut transform_query: Query<&mut Transform, With<Tank>>,
) {
    let mut tank_transform = transform_query.single_mut();

    // x轴移动
    let mut x_direction = 0.0;
    // y轴移动
    let mut y_direction = 0.0;
    

    // 一次只能移动一个方向
    if keyboard_input.pressed(KeyCode::Left) {
        x_direction -= 1.0;
    } else if keyboard_input.pressed(KeyCode::Right) {
        x_direction += 1.0;
    } else if keyboard_input.pressed(KeyCode::Up) {
        y_direction += 1.0;
    } else if keyboard_input.pressed(KeyCode::Down) {
        y_direction -= 1.0;
    } else {
        return;
    }

    // 根据速度时间计算新坐标
    let new_tank_x_position = tank_transform.translation.x + x_direction * TANK_SPEED * TIME_STEP;
    let new_tank_y_position = tank_transform.translation.y + y_direction * TANK_SPEED * TIME_STEP;

    // 区域边界，确保坦克不会超出边界
    let left_bound = LEFT_WALL + WALL_THICKNESS / 2.0 + TANK_SIZE.x / 2.0 + TANK_PADDING;
    let right_bound = RIGHT_WALL - WALL_THICKNESS / 2.0 - TANK_SIZE.x / 2.0 - TANK_PADDING;
    let bottom_bound = BOTTOM_WALL + WALL_THICKNESS / 2.0 + TANK_SIZE.x / 2.0 + TANK_PADDING;
    let top_bound = TOP_WALL - WALL_THICKNESS / 2.0 - TANK_SIZE.x / 2.0 - TANK_PADDING;
    tank_transform.translation.x = new_tank_x_position.clamp(left_bound, right_bound);
    tank_transform.translation.y = new_tank_y_position.clamp(bottom_bound, top_bound);
}

// 保护盾动画播放
pub fn animate_shield_system(
    time: Res<Time>,
    texture_atlases: Res<Assets<TextureAtlas>>,
    mut query: Query<(
        &mut AnimationTimer,
        &mut TextureAtlasSprite,
        &Handle<TextureAtlas>,
    ), With<Shield>>,
) {
    for (mut timer, mut sprite, texture_atlas_handle) in &mut query {
        timer.tick(time.delta());
        if timer.just_finished() {
            let texture_atlas = texture_atlases.get(texture_atlas_handle).unwrap();
            // 切换到下一个sprite
            sprite.index = (sprite.index + 1) % texture_atlas.textures.len();
        }
    }
}