use bevy::prelude::{Event, Query, Transform, Trigger, Vec3, With};
use crate::prelude::CameraTag;

#[derive(Event)]
pub struct Teleport {
    pub target: Vec3,

}

pub fn handle_teleport(
    look_at_event: Trigger<Teleport>,
    mut camera_transform_q: Query<&mut Transform, With<CameraTag>>,
) {
    let mut camera_transform = camera_transform_q.single_mut();
    let rotation = camera_transform.rotation;

    *camera_transform = Transform::from_xyz(
        look_at_event.event().target.x,
        look_at_event.event().target.y,
        look_at_event.event().target.z,
    );
    camera_transform.rotation = rotation;
}