use bevy::app::App;
use bevy::audio::CpalSample;
use bevy::pbr::PbrBundle;
use bevy::prelude::*;

use crate::entity::entity::Health;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Player>()
            .add_startup_system(spawn_player)
            .add_system(player_movement_control);
    }
}

#[derive(Reflect, Component, Default)]
#[reflect(Component)]
pub struct Player;

pub fn spawn_player(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut material: ResMut<Assets<StandardMaterial>>,
) {
    commands
        .spawn(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Cube { size: 0.4 })),
            material: material.add(Color::rgb(0.87, 0.84, 0.92).into()),
            transform: Transform::from_xyz(0.0, 0.2, 1.0),
            ..default()
        })
        .insert(Player)
        .with_children(|parent| {
            parent.spawn(Camera3dBundle {
                transform: Transform::from_xyz(0.0, 0.7, 2.8).with_scale(Vec3 {
                    x: 1.0,
                    y: 1.0,
                    z: 1.0,
                }),
                ..default()
            });
        }).insert(Name::new("Player Camera"))
        .insert(Health { value: 10 })
        .insert(Name::new("Player"));
}

pub fn player_movement_control(
    keyboard: Res<Input<KeyCode>>,
    mut player_query: Query<&mut Transform, With<Player>>,
    time: Res<Time>,
) {
    let speed: f32 = 3.0;
    let mut player = player_query.single_mut();
    let mut player_forward = player.forward();
    let mut player_left = player.left();
    let rotate_speed = 0.3;
    player_left.y = 0.0;
    player_left = player_left.normalize();
    player_forward.y = 0.0;
    player_forward = player_forward.normalize();

    if keyboard.pressed(KeyCode::W) {
        player.translation += player_forward * time.delta_seconds() * speed;
    }
    if keyboard.pressed(KeyCode::S) {
        player.translation -= player_forward * time.delta_seconds() * speed;
    }
    if keyboard.pressed(KeyCode::A) {
        player.translation += player_left * time.delta_seconds() * speed;
    }
    if keyboard.pressed(KeyCode::D) {
        player.translation -= player_left * time.delta_seconds() * speed;
    }
    if keyboard.pressed(KeyCode::Q) {
        player.rotate_axis(Vec3::Y, rotate_speed * time.delta_seconds())
    }
    if keyboard.pressed(KeyCode::E) {
        player.rotate_axis(Vec3::Y, -rotate_speed * time.delta_seconds())
    }
}
