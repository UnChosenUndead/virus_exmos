use bevy::prelude::*;


use crate::entity::entity::Health;
use crate::plugins::bullet::bullet::BulletPlugin;
use crate::plugins::player::player::PlayerPlugin;
use crate::plugins::target::target::{Target, TargetPlugin};
use crate::plugins::tower::tower::{Tower, TowerPlugin};

pub struct BasicScenePlugin;

impl Plugin for BasicScenePlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_basic_scene)
            .add_plugin(BulletPlugin)
            .add_plugin(TowerPlugin)
            .add_plugin(TargetPlugin)
            .add_plugin(PlayerPlugin);
    }
}

pub fn spawn_basic_scene(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut material: ResMut<Assets<StandardMaterial>>,
) {
    commands
        .spawn(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Cube { size: 0.4 })),
            material: material.add(Color::rgb(0.67, 0.84, 0.92).into()),
            transform: Transform::from_xyz(-2.0, 0.2, 1.5),
            ..default()
        })
        .insert(Target { speed: 0.3 })
        .insert(Health { value: 3 })
        .insert(Name::new("Target"));
    commands
        .spawn(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Cube { size: 0.4 })),
            material: material.add(Color::rgb(0.67, 0.84, 0.92).into()),
            transform: Transform::from_xyz(-4.0, 0.2, 1.5),
            ..default()
        })
        .insert(Target { speed: 0.3 })
        .insert(Health { value: 3 })
        .insert(Name::new("Target"));
    commands
        .spawn(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Plane { size: 5.0 })),
            material: material.add(Color::rgb(0.3, 0.5, 0.3).into()),
            ..default()
        })
        .insert(Name::new("World Plato"));
    commands
        .spawn(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
            material: material.add(Color::rgb(0.67, 0.84, 0.92).into()),
            transform: Transform::from_xyz(0.0, 0.5, 0.0),
            ..default()
        })
        .insert(Tower {
            shooting_timer: Timer::from_seconds(1.0, TimerMode::Repeating),
            bullet_offset: Vec3::new(0.0, 0.2, 0.5),
        })
        .insert(Name::new("Tower"));
    commands
        .spawn(PointLightBundle {
            point_light: PointLight {
                color: Default::default(),
                intensity: 1500.0,
                shadows_enabled: true,
                ..default()
            },
            transform: Transform::from_xyz(8.0, 8.0, 4.0),
            ..default()
        })
        .insert(Name::new("World Light"));
}
