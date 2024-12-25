use bevy::prelude::*;
use bevy_rapier2d::prelude::{Collider, RigidBody};

use crate::common::{LEVEL_COLUMNS, LEVEL_ROWS, TILE_SIZE};

pub const WALL_THICKNESS: f32 = 10.0;

#[derive(Debug, Component)]
pub struct AreaWall;

pub fn setup_wall(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let left_wall = -LEVEL_COLUMNS as f32 / 2.0 * TILE_SIZE - WALL_THICKNESS / 2.0;
    let right_wall = LEVEL_COLUMNS as f32 / 2.0 * TILE_SIZE + WALL_THICKNESS / 2.0;
    let top_wall = LEVEL_ROWS as f32 / 2.0 * TILE_SIZE + WALL_THICKNESS / 2.0;
    let bottom_wall = -LEVEL_ROWS as f32 / 2.0 * TILE_SIZE - WALL_THICKNESS / 2.0;
    let arena_height = top_wall - bottom_wall;
    let arena_width = right_wall - left_wall;
    let wall_color = Color::srgb(0.8, 0.8, 0.8);
    let material_handle = materials.add(ColorMaterial::from_color(wall_color));

    // left wall
    commands.spawn((
        AreaWall,
        Mesh2d(meshes.add(Rectangle::new(WALL_THICKNESS, arena_height + WALL_THICKNESS).mesh())),
        MeshMaterial2d(material_handle.clone()),
        Transform::from_translation(Vec3::new(left_wall, 0., 0.)),
        RigidBody::Fixed,
        Collider::cuboid(WALL_THICKNESS / 2.0, (arena_height + WALL_THICKNESS) / 2.0),
    ));

    // right wall
    commands.spawn((
        AreaWall,
        Mesh2d(meshes.add(Rectangle::new(WALL_THICKNESS, arena_height + WALL_THICKNESS).mesh())),
        MeshMaterial2d(material_handle.clone()),
        Transform::from_translation(Vec3::new(right_wall, 0., 0.)),
        RigidBody::Fixed,
        Collider::cuboid(WALL_THICKNESS / 2.0, (arena_height + WALL_THICKNESS) / 2.0),
    ));

    // top wall
    commands.spawn((
        AreaWall,
        Mesh2d(meshes.add(Rectangle::new(arena_width + WALL_THICKNESS, WALL_THICKNESS).mesh())),
        MeshMaterial2d(material_handle.clone()),
        Transform::from_translation(Vec3::new(0.0, top_wall, 0.)),
        RigidBody::Fixed,
        Collider::cuboid((arena_width + WALL_THICKNESS) / 2.0, WALL_THICKNESS / 2.0),
    ));

    // bottom wall
    commands.spawn((
        AreaWall,
        Mesh2d(meshes.add(Rectangle::new(arena_width + WALL_THICKNESS, WALL_THICKNESS).mesh())),
        MeshMaterial2d(material_handle.clone()),
        Transform::from_translation(Vec3::new(0.0, bottom_wall, 0.)),
        RigidBody::Fixed,
        Collider::cuboid((arena_width + WALL_THICKNESS) / 2.0, WALL_THICKNESS / 2.0),
    ));
}
