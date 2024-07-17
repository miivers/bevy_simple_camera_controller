use bevy::app::{Plugin, App, PreUpdate};
use bevy::ecs::component::Component;
use bevy::input::ButtonInput;
use bevy::input::mouse::MouseMotion;
use bevy::math::{Quat, Vec3};
use bevy::prelude::{EventReader, KeyCode, Query, Res, Transform, With};
use bevy::time::{Real, Time};

#[derive(Component)]
pub struct CameraTag;

pub struct FreeCameraPlugin;

impl Plugin for FreeCameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PreUpdate, update);
    }
}

fn update(
    mut query: Query<&mut Transform, With<CameraTag>>,
    mut mouse_motion_events: EventReader<MouseMotion>,
    keys: Res<ButtonInput<KeyCode>>,
    time: Res<Time<Real>>,
) {
    for mut transform in &mut query {
        let mut movement_vector = Vec3::ZERO;

        if keys.pressed(KeyCode::KeyW) {
            movement_vector += Vec3::from(transform.forward());
        }

        if keys.pressed(KeyCode::KeyS) {
            movement_vector -= Vec3::from(transform.forward());
        }

        if keys.pressed(KeyCode::KeyA) {
            movement_vector -= Vec3::from(transform.right());
        }

        if keys.pressed(KeyCode::KeyD) {
            movement_vector += Vec3::from(transform.right());
        }

        let movement_speed: f32 = 10.0;

        if movement_vector != Vec3::ZERO {
            transform.translation += movement_vector.normalize() * time.delta_seconds() * movement_speed;
        }

        for mouse_event in mouse_motion_events.read() {
            println!("Mouse moved: X: {} px, Y: {} px", mouse_event.delta.x, mouse_event.delta.y);
            let yaw_rotation = Quat::from_rotation_y(-mouse_event.delta.x * 0.2 * time.delta_seconds());

            // Calculate rotation around the local X axis (pitch)
            let pitch_rotation = Quat::from_rotation_x(-mouse_event.delta.y * 0.2 * time.delta_seconds());

            // Apply the rotations to the transform
            transform.rotation = yaw_rotation * transform.rotation; // Yaw first to rotate around global Y axis
            transform.rotation = transform.rotation * pitch_rotation; // Then pitch to rotate around local X axis
        }
    }
}