use bevy::{prelude::*, sprite::MaterialMesh2dBundle};
use bevy_rapier2d::prelude::{Collider, RigidBody};

pub const WALL_THICKNESS: f32 = 10.0;
// 墙壁x轴和y轴坐标
pub const LEFT_WALL: f32 = -450.;
pub const RIGHT_WALL: f32 = 450.;
pub const BOTTOM_WALL: f32 = -300.;
pub const TOP_WALL: f32 = 300.;

pub fn setup_wall(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let arena_height = TOP_WALL - BOTTOM_WALL;
    let arena_width = RIGHT_WALL - LEFT_WALL;
    let wall_color = Color::rgb(0.8, 0.8, 0.8);
    let material_handle = materials.add(wall_color.into());

    // left wall
    commands.spawn((
        MaterialMesh2dBundle {
            mesh: meshes
                .add(
                    shape::Quad::new(Vec2::new(WALL_THICKNESS, arena_height + WALL_THICKNESS))
                        .into(),
                )
                .into(),
            material: material_handle.clone(),
            transform: Transform::from_translation(Vec3::new(LEFT_WALL, 0., 0.)),
            ..default()
        },
        RigidBody::Fixed,
        Collider::cuboid(WALL_THICKNESS / 2.0, (arena_height + WALL_THICKNESS) / 2.0),
    ));

    // right wall
    commands.spawn((
        MaterialMesh2dBundle {
            mesh: meshes
                .add(
                    shape::Quad::new(Vec2::new(WALL_THICKNESS, arena_height + WALL_THICKNESS))
                        .into(),
                )
                .into(),
            material: material_handle.clone(),
            transform: Transform::from_translation(Vec3::new(RIGHT_WALL, 0., 0.)),
            ..default()
        },
        RigidBody::Fixed,
        Collider::cuboid(WALL_THICKNESS / 2.0, (arena_height + WALL_THICKNESS) / 2.0),
    ));

    // top wall
    commands.spawn((
        MaterialMesh2dBundle {
            mesh: meshes
                .add(
                    shape::Quad::new(Vec2::new(arena_width + WALL_THICKNESS, WALL_THICKNESS))
                        .into(),
                )
                .into(),
            material: material_handle.clone(),
            transform: Transform::from_translation(Vec3::new(0.0, TOP_WALL, 0.)),
            ..default()
        },
        RigidBody::Fixed,
        Collider::cuboid((arena_width + WALL_THICKNESS) / 2.0, WALL_THICKNESS / 2.0),
    ));

    // bottom wall
    commands.spawn((
        MaterialMesh2dBundle {
            mesh: meshes
                .add(
                    shape::Quad::new(Vec2::new(arena_width + WALL_THICKNESS, WALL_THICKNESS))
                        .into(),
                )
                .into(),
            material: material_handle.clone(),
            transform: Transform::from_translation(Vec3::new(0.0, BOTTOM_WALL, 0.)),
            ..default()
        },
        RigidBody::Fixed,
        Collider::cuboid((arena_width + WALL_THICKNESS) / 2.0, WALL_THICKNESS / 2.0),
    ));
}
