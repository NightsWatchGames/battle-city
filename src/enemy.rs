use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::{
    common::{self, AnimationTimer},
    player::{TankRefreshBulletTimer, TANK_REFRESH_BULLET_INTERVAL},
    wall::BOTTOM_WALL,
};

#[derive(Component)]
pub struct Enemy;

pub fn setup_enemies(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let tank_texture_handle = asset_server.load("textures/enemies.bmp");
    let tank_texture_atlas =
        TextureAtlas::from_grid(tank_texture_handle, Vec2::new(28.0, 28.0), 2, 4, None, None);
    let tank_texture_atlas_handle = texture_atlases.add(tank_texture_atlas);

    commands
        .spawn(Enemy)
        .insert(SpriteSheetBundle {
            texture_atlas: tank_texture_atlas_handle,
            transform: Transform {
                translation: Vec3::new(0.0, BOTTOM_WALL + 150.0, 0.0),
                ..default()
            },
            ..default()
        })
        .insert(AnimationTimer(Timer::from_seconds(
            0.2,
            TimerMode::Repeating,
        )))
        .insert(TankRefreshBulletTimer(Timer::from_seconds(
            TANK_REFRESH_BULLET_INTERVAL,
            TimerMode::Once,
        )))
        .insert(common::Direction::Up)
        .insert(RigidBody::Dynamic)
        .insert(Collider::cuboid(18.0, 18.0))
        .insert(ActiveEvents::COLLISION_EVENTS)
        .insert(LockedAxes::ROTATION_LOCKED);
}
