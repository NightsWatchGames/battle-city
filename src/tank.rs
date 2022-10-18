use crate::common::*;
use crate::wall::*;
use crate::common;

use bevy::prelude::*;

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

// 出生保护盾计时
#[derive(Component, Deref, DerefMut)]
pub struct ShieldRemoveTimer(pub Timer);

// 移动坦克
pub fn move_tank_system(
    keyboard_input: Res<Input<KeyCode>>,
    mut transform_query: Query<(&mut Transform, &mut common::Direction), With<Tank>>,
) {
    let (mut tank_transform, mut direction) = transform_query.single_mut();

    // x轴移动
    let mut x_direction = 0.0;
    // y轴移动
    let mut y_direction = 0.0;

    // 一次只能移动一个方向
    if keyboard_input.pressed(KeyCode::Left) {
        x_direction -= 1.0;
        *direction = common::Direction::Left;
    } else if keyboard_input.pressed(KeyCode::Right) {
        x_direction += 1.0;
        *direction = common::Direction::Right;
    } else if keyboard_input.pressed(KeyCode::Up) {
        y_direction += 1.0;
        *direction = common::Direction::Up;
    } else if keyboard_input.pressed(KeyCode::Down) {
        y_direction -= 1.0;
        *direction = common::Direction::Down;
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

// 坦克移动动画播放
pub fn tank_animate_system(
    time: Res<Time>,
    texture_atlases: Res<Assets<TextureAtlas>>,
    mut query: Query<
        (
            &mut AnimationTimer,
            &mut TextureAtlasSprite,
            &Handle<TextureAtlas>,
            &common::Direction,
        ),
        With<Tank>,
    >,
) {
    for (mut timer, mut sprite, texture_atlas_handle, direction) in &mut query {
        timer.tick(time.delta());
        if timer.just_finished() {
            // 切换到下一个sprite
            let texture_atlas = texture_atlases.get(texture_atlas_handle).unwrap();
            // 每个方向上的sprite数量
            let sprites_each_direction = texture_atlas.len() / 4;
            match direction {
                common::Direction::Up => {
                    sprite.index = (sprite.index + 1) % 2 + sprites_each_direction * 0;
                },
                common::Direction::Right => {
                    sprite.index = (sprite.index + 1) % 2 + sprites_each_direction * 1;
                },
                common::Direction::Down => {
                    sprite.index = (sprite.index + 1) % 2 + sprites_each_direction * 2;
                },
                common::Direction::Left => {
                    sprite.index = (sprite.index + 1) % 2 + sprites_each_direction * 3;
                },
            }
        }
    }
}

// 保护盾动画播放
pub fn shield_animate_system(
    time: Res<Time>,
    texture_atlases: Res<Assets<TextureAtlas>>,
    mut query: Query<
        (
            &mut AnimationTimer,
            &mut TextureAtlasSprite,
            &Handle<TextureAtlas>,
        ),
        With<Shield>,
    >,
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

// 移除保护盾
pub fn shield_remove_system(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut ShieldRemoveTimer), With<Shield>>,
) {
    for (entity, mut timer) in query.iter_mut() {
        timer.tick(time.delta());

        if timer.just_finished() {
            commands.entity(entity).despawn();
        }
    }
}
