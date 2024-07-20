use crate::camera_controller::CameraControllerPlugin;
use crate::prelude::CustomCameraControllerBuilder;


pub struct TopDownControllerBuilder {
    builder: CustomCameraControllerBuilder,
}

impl TopDownControllerBuilder {
    pub fn new() -> Self {
        Self {
            builder: CustomCameraControllerBuilder::new()
        }
    }

    pub fn build(&self) -> CameraControllerPlugin {
        self.builder.build()
    }
}

impl Default for TopDownControllerBuilder {
    fn default() -> Self {
        let mut camera_builder_config = Self::new();
        camera_builder_config.builder.with_movement();
        camera_builder_config.builder.with_lock_y_axis_movement();

        return  camera_builder_config
    }
}