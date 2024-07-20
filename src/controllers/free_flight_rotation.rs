use bevy::math::Quat;
use bevy::prelude::{EventReader, Query, Real, Res, Time, Transform, With};
use crate::data::camera_properties::CameraProperties;
use crate::data::key_binding::{CameraRotationAction, CameraRotationEvents};
use crate::prelude::CameraTag;

pub fn update_free_flight_rotation(
    mut query: Query<&mut Transform, With<CameraTag>>,
    properties: Res<CameraProperties>,
    mut rotation_events: EventReader<CameraRotationEvents>,
    time: Res<Time<Real>>,
) {
    if rotation_events.is_empty() {
        return;
    }

    let mut yaw_rotation: f32 = 0.;
    let mut pitch_rotation: f32 = 0.;

    let mut camera_transform = query.single_mut();

    for events in rotation_events.read() {
        for event in &events.0 {
            match event {
                CameraRotationAction::Horizontal(delta) => yaw_rotation += delta,
                CameraRotationAction::Vertical(delta)   => pitch_rotation += delta,
            }
        }
    }

    camera_transform.rotation = Quat::from_rotation_y(-yaw_rotation * properties.rotation_speed * time.delta_seconds()) * camera_transform.rotation;
    camera_transform.rotation = camera_transform.rotation * Quat::from_rotation_x(-pitch_rotation * properties.rotation_speed * time.delta_seconds());
}