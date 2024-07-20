use bevy::math::{Quat, Vec3};
use bevy::prelude::{EventReader, Query, Res, Time, Transform, With, Without};
use crate::data::key_binding::{CameraRotationAction, CameraRotationEvents};
use crate::prelude::{CameraOrbitTag, CameraProperties, CameraTag};

pub fn update_orbit_rotation(
    mut camera_transform_q: Query<&mut Transform, (With<CameraTag>, Without<CameraOrbitTag>)>,
    target_transform_q: Query<&Transform, With<CameraOrbitTag>>,
    properties: Res<CameraProperties>,
    mut rotation_events: EventReader<CameraRotationEvents>,
    time: Res<Time>,
) {
    // Guard clause to exit early if there are no relevant events or queries
    if rotation_events.is_empty() ||
        target_transform_q.is_empty() ||
        camera_transform_q.is_empty() {
        return;
    }

    let mut yaw_rotation_delta: f32 = 0.;
    let mut pitch_rotation_delta: f32 = 0.;

    let mut camera_transform = camera_transform_q.single_mut();
    let target_transform = target_transform_q.single();

    // Accumulate rotation deltas from the events
    for events in rotation_events.read() {
        for event in &events.0 {
            match event {
                CameraRotationAction::Horizontal(delta) => yaw_rotation_delta += delta,
                CameraRotationAction::Vertical(delta)   => pitch_rotation_delta += delta,
            }
        }
    }

    // Scale the rotations by the delta time
    let delta_time = time.delta_seconds();

    let yaw = Quat::from_rotation_y(-yaw_rotation_delta * properties.rotation_speed * delta_time);
    let pitch = Quat::from_rotation_x(-pitch_rotation_delta * properties.rotation_speed * delta_time);

    // Rotate the camera around the global Y axis (yaw)
    camera_transform.rotation = yaw * camera_transform.rotation;

    // Calculate the new rotation without applying it to the camera yet
    let new_rotation = camera_transform.rotation * pitch;

    // Check if new rotation will cause the camera to go beyond the 180-degree vertical bounds
    let up_vector = new_rotation * Vec3::Y;
    if up_vector.y > 0.0 {
        camera_transform.rotation = new_rotation;
    }

    // Calculate the camera's new position based on the rotation and the distance to the target
    let radius = camera_transform.translation.distance(target_transform.translation);
    let offset = camera_transform.rotation * Vec3::new(0.0, 0.0, radius);

    // Update the camera's position relative to the target
    camera_transform.translation = target_transform.translation + offset;
}