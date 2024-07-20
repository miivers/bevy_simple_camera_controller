use bevy::math::Vec3;
use bevy::prelude::{EventReader, Query, Real, Res, Time, Transform, With};
use crate::data::camera_properties::CameraProperties;
use crate::data::key_binding::{CameraAction, CameraMovementEvents};
use crate::input::capture_cursor::CameraTag;

pub fn update_movement(
    mut query: Query<&mut Transform, With<CameraTag>>,
    properties: Res<CameraProperties>,
    mut movement_events: EventReader<CameraMovementEvents>,
    time: Res<Time<Real>>,
) {
    if movement_events.is_empty() {
        return;
    }

    let mut movement_vector = Vec3::ZERO;

    let mut camera_transform = query.single_mut();

    for events in movement_events.read() {
        for event in &events.0 {
            match event {
                CameraAction::MoveForward  => movement_vector += Vec3::from(camera_transform.forward()),
                CameraAction::MoveBackward => movement_vector -= Vec3::from(camera_transform.forward()),
                CameraAction::MoveLeft     => movement_vector -= Vec3::from(camera_transform.right()),
                CameraAction::MoveRight    => movement_vector += Vec3::from(camera_transform.right()),
                CameraAction::MoveUp       => movement_vector += Vec3::from(camera_transform.up()),
                CameraAction::MoveDown     => movement_vector -= Vec3::from(camera_transform.up()),
            }
        }
    }

    if properties.lock_y_axis_movement {
        movement_vector = Vec3::new(movement_vector.x, 0., movement_vector.z);
    }

    // Prevent NaN when normalizing vector
    if movement_vector == Vec3::ZERO {
        return;
    }

    camera_transform.translation += movement_vector.normalize() * time.delta_seconds() * properties.movement_speed;
}