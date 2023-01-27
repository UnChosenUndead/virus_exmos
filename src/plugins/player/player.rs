use bevy::app::App;
use bevy::pbr::PbrBundle;
use bevy::prelude::*;

use crate::entity::entity::Health;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Player>()
            .add_startup_system(spawn_player)
            .add_startup_system(spawn_player_camera)
            .add_system(camera_player_control)
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
        .insert(Health { value: 10 })
        .insert(Name::new("Player"));
}

// pub fn camera_player_control(
//     mut ps: ParamSet<(
//         Query<&mut Transform, With<Player>>,
//         Query<&mut Transform, With<Camera3d>>,
//     )>,
// ) {
//     let player = ps.p0().single().to_owned();
//     let mut camera = ps
//         .p1()
//         .single_mut()
//         .rotation
//         .x = player.rotation.x - 1.0;
// }

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

pub fn spawn_player_camera(mut commands: Commands) {
    commands
        .spawn(Camera3dBundle {
            transform: Transform::from_xyz(0.0, 0.7, 2.8).with_scale(Vec3 {
                x: 1.0,
                y: 1.0,
                z: 1.0,
            }),
            ..default()
        })
        .insert(Name::new("Player Camera"));
}

pub fn camera_player_control(
    keyboard: Res<Input<KeyCode>>,
    mut ps: ParamSet<(
        Query<&mut Transform, With<Camera3d>>,
        Query<&mut Transform, With<Player>>,
    )>,
    time: Res<Time>,
) {
    let speed: f32 = 3.0;
    let player = ps.p1().single().clone();
    let mut binding = ps.p0();
    let mut camera = binding.single_mut();
    let mut camera_forward = camera.forward();
    let mut camera_left = camera.left();
    let rotate_speed = 0.3;
    camera_left.y = 0.0;
    camera_left = camera_left.normalize();
    camera_forward.y = 0.0;
    camera_forward = camera_forward.normalize();

    if keyboard.pressed(KeyCode::W) {
        camera.translation += camera_forward * time.delta_seconds() * speed;
    }
    if keyboard.pressed(KeyCode::S) {
        camera.translation -= camera_forward * time.delta_seconds() * speed;
    }
    if keyboard.pressed(KeyCode::A) {
        camera.translation += camera_left * time.delta_seconds() * speed;
    }
    if keyboard.pressed(KeyCode::D) {
        camera.translation -= camera_left * time.delta_seconds() * speed;
    }
    if keyboard.pressed(KeyCode::Q) {
        camera.rotate_axis(Vec3::Y, rotate_speed * time.delta_seconds())
    }
    if keyboard.pressed(KeyCode::E) {
        camera.rotate_axis(Vec3::Y, -rotate_speed * time.delta_seconds())
    }
}
