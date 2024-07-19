use bevy::prelude::{Event, Query, Transform, Trigger, Vec3, With};
use crate::prelude::CameraTag;

#[derive(Event)]
pub struct LookAt {
    pub target: Vec3,

}

pub fn handle_look_at(
    look_at_event: Trigger<LookAt>,
    mut camera_transform_q: Query<&mut Transform, With<CameraTag>>,
) {
    let mut camera_transform = camera_transform_q.single_mut();
    *camera_transform = camera_transform.looking_at(
        look_at_event.event().target,
        camera_transform.up()
    );
}