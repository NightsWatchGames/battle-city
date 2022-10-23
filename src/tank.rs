use crate::bullet::spawn_bullet;
use crate::common::*;
use crate::wall::*;
use crate::common;

use bevy::prelude::*;

pub const TANK_SIZE: Vec2 = Vec2::new(80.0, 80.0);
pub const TANK_SPEED: f32 = 200.0;
// 坦克离墙最近距离限制
pub const TANK_PADDING: f32 = 10.0;

// 坦克刷新子弹间隔
pub const TANK_REFRESH_BULLET_INTERVAL: f32 = 2.0;

// 坦克
#[derive(Component)]
pub struct Tank;

// 坦克刷新子弹计时器
#[derive(Component, Deref, DerefMut)]
pub struct TankRefreshBulletTimer(pub Timer);

// 出生保护盾
#[derive(Component)]
pub struct Shield;

// 出生保护盾计时
#[derive(Component, Deref, DerefMut)]
pub struct ShieldRemoveTimer(pub Timer);

// 移动坦克
pub fn tank_move_system(
    keyboard_input: Res<Input<KeyCode>>,
    mut transform_query: Query<(&mut Transform, &mut common::Direction), With<Tank>>,
) {
    let (mut tank_transform, mut direction) = transform_query.single_mut();

    let mut tank_x_position = tank_transform.translation.x;
    let mut tank_y_position = tank_transform.translation.y;

    // 一次只能移动一个方向
    // 根据速度时间计算新坐标
    if keyboard_input.pressed(KeyCode::Left) {
        tank_x_position -= 1.0 * TANK_SPEED * TIME_STEP;
        *direction = common::Direction::Left;
    } else if keyboard_input.pressed(KeyCode::Right) {
        tank_x_position += 1.0 * TANK_SPEED * TIME_STEP;
        *direction = common::Direction::Right;
    } else if keyboard_input.pressed(KeyCode::Up) {
        tank_y_position += 1.0 * TANK_SPEED * TIME_STEP;
        *direction = common::Direction::Up;
    } else if keyboard_input.pressed(KeyCode::Down) {
        tank_y_position -= 1.0 * TANK_SPEED * TIME_STEP;
        *direction = common::Direction::Down;
    } else {
        return;
    }

    // 区域边界，确保坦克不会超出边界
    let left_bound = LEFT_WALL + WALL_THICKNESS / 2.0 + TANK_SIZE.x / 2.0 + TANK_PADDING;
    let right_bound = RIGHT_WALL - WALL_THICKNESS / 2.0 - TANK_SIZE.x / 2.0 - TANK_PADDING;
    let bottom_bound = BOTTOM_WALL + WALL_THICKNESS / 2.0 + TANK_SIZE.x / 2.0 + TANK_PADDING;
    let top_bound = TOP_WALL - WALL_THICKNESS / 2.0 - TANK_SIZE.x / 2.0 - TANK_PADDING;
    tank_transform.translation.x = tank_x_position.clamp(left_bound, right_bound);
    tank_transform.translation.y = tank_y_position.clamp(bottom_bound, top_bound);
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
                    sprite.index = (sprite.index + 1) % sprites_each_direction + sprites_each_direction * 0;
                },
                common::Direction::Right => {
                    sprite.index = (sprite.index + 1) % sprites_each_direction + sprites_each_direction * 1;
                },
                common::Direction::Down => {
                    sprite.index = (sprite.index + 1) % sprites_each_direction + sprites_each_direction * 2;
                },
                common::Direction::Left => {
                    sprite.index = (sprite.index + 1) % sprites_each_direction + sprites_each_direction * 3;
                },
            }
        }
    }
}

// 坦克攻击
pub fn tank_attack_system(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&Transform, &common::Direction, &mut TankRefreshBulletTimer), With<Tank>>,
    time: Res<Time>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    for (transform, direction, mut refresh_bullet_timer) in &mut query {
        refresh_bullet_timer.tick(time.delta());
        if keyboard_input.pressed(KeyCode::Space) {
            if refresh_bullet_timer.finished() {
                spawn_bullet(&mut commands, &asset_server, &mut texture_atlases, direction.clone(), transform);
                refresh_bullet_timer.reset();
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
