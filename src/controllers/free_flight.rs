use bevy::prelude::{Camera3dBundle, Commands, EventReader, KeyCode, Query, Res, Transform, Window, With};
use bevy::window::{CursorGrabMode, PrimaryWindow};
use bevy::app::{Plugin, App, FixedUpdate};
use bevy::input::mouse::MouseMotion;
use bevy::input::ButtonInput;
use bevy::math::{Quat, Vec3};
use bevy::time::{Real, Time};
use bevy::utils::default;
use crate::camera_common::{CameraTag, capture_cursor, disable_capture_cursor};
use crate::camera_properties::{CameraProperties, InitialPosition};
use crate::key_binding::{CameraAction, CameraKeyBindings};

#[derive(Default)]
pub struct FreeFlightCameraPlugin {
    pub initial_position: InitialPosition,
    pub properties: CameraProperties,
}

impl FreeFlightCameraPlugin {
    pub fn new(
        initial_position: InitialPosition,
        properties: CameraProperties
    ) -> Self {
        Self {
            initial_position,
            properties,
        }
    }

    pub fn create_camera(
        mut commands: Commands,
        initial_position: Res<InitialPosition>
    ) {
        commands.spawn((Camera3dBundle {
            transform: Transform::from_xyz(
                initial_position.position.x,
                initial_position.position.y,
                initial_position.position.z,
            ).looking_at(
                initial_position.look_at,
                initial_position.up_vector,
            ),
            ..default()
        },
            CameraTag
        ));
    }
}

impl Plugin for FreeFlightCameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(FixedUpdate, update);

        if self.properties.grab_mouse {
            app.add_systems(FixedUpdate, capture_cursor);
            app.add_systems(FixedUpdate, disable_capture_cursor);
        }

        app.insert_resource(self.initial_position.clone());
        app.insert_resource(self.properties.clone());
        app.insert_resource(self.properties.key_bindings.clone());
    }
}

fn update(
    mut query: Query<&mut Transform, With<CameraTag>>,
    mut q_windows: Query<&mut Window, With<PrimaryWindow>>,
    mut mouse_motion_events: EventReader<MouseMotion>,
    key_bindings: Res<CameraKeyBindings>,
    keys: Res<ButtonInput<KeyCode>>,
    properties: Res<CameraProperties>,
    time: Res<Time<Real>>,
) {

    if properties.grab_mouse {
        // We use CursorGrabMode::Locked to signal the user has clicked and given focus to the window
        if q_windows.single_mut().cursor.grab_mode != CursorGrabMode::Locked {
            return;
        }
    }

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
            transform.translation += movement_vector.normalize() * time.delta_seconds() * properties.movement_speed;
        }

        for mouse_event in mouse_motion_events.read() {
            let yaw_rotation = Quat::from_rotation_y(-mouse_event.delta.x * properties.rotation_speed * time.delta_seconds());
            let pitch_rotation = Quat::from_rotation_x(-mouse_event.delta.y * properties.rotation_speed * time.delta_seconds());

            transform.rotation = yaw_rotation * transform.rotation;
            transform.rotation = transform.rotation * pitch_rotation;
        }
    }
}