use crate::entity::entity::Health;
use crate::plugins::bullet::bullet::{
    bullet_collision, bullet_despawn, move_bullets, Bullet, GameAssets, LifeTime,
};
use bevy::app::App;
use bevy::asset::{AssetServer, Assets, Handle};
use bevy::hierarchy::{BuildChildren, DespawnRecursiveExt};
use bevy::math::{Quat, Vec3};
use bevy::pbr::{PointLight, PointLightBundle, SpotLightBundle, StandardMaterial};
use bevy::prelude::*;
use bevy::scene::{Scene, SceneBundle};
use bevy::time::Timer;
use bevy::utils::{default, FloatOrd};
use std::f32::consts::PI;
use std::os::unix::raw::time_t;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_camera);
    }
}

pub fn spawn_camera(mut commands: Commands) {
    commands
        .spawn(Camera3dBundle {
            transform: Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        })
        .insert(Name::new("Main Camera"));
}
