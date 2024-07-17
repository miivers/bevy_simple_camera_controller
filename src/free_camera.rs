use bevy::prelude::{Commands, EventReader, KeyCode, Query, Res, Transform, With};
use bevy::app::{Plugin, App, PreUpdate, Startup};
use bevy::ecs::component::Component;
use bevy::input::mouse::MouseMotion;
use bevy::input::ButtonInput;
use bevy::math::{Quat, Vec3};
use bevy::time::{Real, Time};

use crate::key_binding::{CameraAction, CameraKeyBindings};

#[derive(Component)]
pub struct CameraTag;

pub struct FreeCameraPlugin;

const MOVEMENT_SPEED: f32 = 10.0;
const ROTATION_SPEED: f32 = 0.2;

impl Plugin for FreeCameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PreUpdate, update);
        app.add_systems(Startup, register_key_bindings);
    }
}

fn register_key_bindings(mut commands: Commands) {
    commands.insert_resource(CameraKeyBindings::default());
}

fn update(
    mut query: Query<&mut Transform, With<CameraTag>>,
    mut mouse_motion_events: EventReader<MouseMotion>,
    key_bindings: Res<CameraKeyBindings>,
    keys: Res<ButtonInput<KeyCode>>,
    time: Res<Time<Real>>,
) {
    for mut transform in &mut query {
        let mut movement_vector = Vec3::ZERO;

        if let Some(&key) = key_bindings.bindings.get(&CameraAction::MoveForward) {
            if keys.pressed(key) {
                movement_vector += Vec3::from(transform.forward());
            }
        }

        if let Some(&key) = key_bindings.bindings.get(&CameraAction::MoveBackward) {
            if keys.pressed(key) {
                movement_vector -= Vec3::from(transform.forward());
            }
        }

        if let Some(&key) = key_bindings.bindings.get(&CameraAction::MoveLeft) {
            if keys.pressed(key) {
                movement_vector -= Vec3::from(transform.right());
            }
        }

        if let Some(&key) = key_bindings.bindings.get(&CameraAction::MoveRight) {
            if keys.pressed(key) {
                movement_vector += Vec3::from(transform.right());
            }
        }

        if movement_vector != Vec3::ZERO {
            transform.translation += movement_vector.normalize() * time.delta_seconds() * MOVEMENT_SPEED;
        }

        for mouse_event in mouse_motion_events.read() {
            let yaw_rotation = Quat::from_rotation_y(-mouse_event.delta.x * ROTATION_SPEED * time.delta_seconds());
            let pitch_rotation = Quat::from_rotation_x(-mouse_event.delta.y * ROTATION_SPEED * time.delta_seconds());

            transform.rotation = yaw_rotation * transform.rotation;
            transform.rotation = transform.rotation * pitch_rotation;
        }
    }
}