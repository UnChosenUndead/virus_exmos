use crate::entity::entity::Health;
use bevy::app::{App, Plugin};
use bevy::hierarchy::DespawnRecursiveExt;
use bevy::prelude::*;

pub struct TargetPlugin;

impl Plugin for TargetPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Target>()
            .register_type::<Health>()
            .add_system(move_targets)
            .add_system(target_death);
    }
}

#[derive(Reflect, Component, Default)]
pub struct Target {
    pub(crate) speed: f32,
}

pub fn move_targets(mut targets: Query<(&Target, &mut Transform)>, time: Res<Time>) {
    for (target, mut transform) in targets.iter_mut() {
        transform.translation.x += target.speed * time.delta_seconds()
    }
}

pub fn target_death(mut commands: Commands, targets: Query<(Entity, &Health)>) {
    for (ent, health) in targets.iter() {
        if health.value <= 0 {
            commands.entity(ent).despawn_recursive();
        }
    }
}
