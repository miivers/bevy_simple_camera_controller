use crate::data::key_binding::CameraKeyBindings;
use bevy::prelude::Resource;
use bevy::math::Vec3;

#[derive(Resource, Clone)]
pub struct CameraProperties {
    pub input_enabled: bool,
    pub hide_cursor: bool,
    pub lock_y_axis_movement: bool,
    pub movement_speed: f32,
    pub rotation_speed: f32,
    pub grab_mouse: bool,
    pub key_bindings: CameraKeyBindings,
}


#[derive(Resource, Clone)]
pub struct InitialPosition {
    pub position: Vec3,
    pub look_at: Vec3,
    pub up_vector: Vec3,
}

impl Default for CameraProperties {
    fn default() -> Self {
        Self {
            input_enabled: true,
            hide_cursor: true,
            lock_y_axis_movement: false,
            movement_speed: 10.0,
            rotation_speed: 0.2,
            grab_mouse: true,
            key_bindings: CameraKeyBindings::wasd(),
        }
    }
}

impl Default for InitialPosition {
    fn default() -> Self {
        Self {
            position: Vec3::new(-2.5, 4.5, 9.0),
            look_at: Vec3::ZERO,
            up_vector: Vec3::Y,
        }
    }
}