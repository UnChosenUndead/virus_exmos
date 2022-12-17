use crate::entity::entity::Health;
use crate::plugins::target::target::Target;
use bevy::app::{App, Plugin};
use bevy::prelude::*;

pub struct BulletPlugin;

impl Plugin for BulletPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(bullet_asset_load)
            .register_type::<Bullet>()
            .register_type::<LifeTime>()
            .add_system(bullet_collision)
            .add_system(bullet_despawn)
            .add_system(move_bullets);
    }
}

#[derive(Reflect, Component, Default)]
#[reflect(Component)]
pub struct LifeTime {
    pub timer: Timer,
}

#[derive(Resource, Default, Debug)]
pub struct GameAssets {
    pub bullet_scene: Handle<Scene>,
}

#[derive(Reflect, Component, Default)]
#[reflect(Component)]
pub struct Bullet {
    pub(crate) direction: Vec3,
    pub(crate) speed: f32,
}

pub fn bullet_collision(
    mut targets: Query<(&mut Health, &Transform), With<Target>>,
    mut commands: Commands,
    bullets: Query<(Entity, &GlobalTransform), With<Bullet>>,
) {
    for (bullet, bullet_transform) in bullets.iter() {
        for (mut health, target_transform) in &mut targets.iter_mut() {
            if Vec3::distance(bullet_transform.translation(), target_transform.translation) < 0.2 {
                commands.entity(bullet).despawn_recursive();
                health.value -= 1;
                break;
            }
        }
    }
}

pub fn bullet_despawn(
    mut commands: Commands,
    mut bullets: Query<(Entity, &mut LifeTime)>,
    time: Res<Time>,
) {
    for (entity, mut lifetime) in &mut bullets.iter_mut() {
        lifetime.timer.tick(time.delta());
        if lifetime.timer.just_finished() {
            commands.entity(entity).despawn_recursive();
        }
    }
}

pub fn move_bullets(mut bullets: Query<(&Bullet, &mut Transform)>, time: Res<Time>) {
    for (mut bullet, mut transform) in bullets.iter_mut() {
        transform.translation += bullet.direction.normalize() * bullet.speed * time.delta_seconds()
    }
}

pub fn bullet_asset_load(mut commands: Commands, assets: Res<AssetServer>) {
    commands.insert_resource(GameAssets {
        bullet_scene: assets.load("Bullet.glb#Scene0"),
    });
}
