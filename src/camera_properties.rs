use bevy::prelude::Resource;

#[derive(Resource, Clone)]
pub struct CameraProperties {
    pub movement_speed: f32,
    pub rotation_speed: f32,
    pub grab_mouse: bool,
}

impl Default for CameraProperties {
    fn default() -> Self {
        Self {
            movement_speed: 10.0,
            rotation_speed: 0.2,
            grab_mouse: true,
        }
    }
}