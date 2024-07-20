use crate::camera_controller::CameraControllerPlugin;
use crate::prelude::CustomCameraControllerBuilder;


pub struct FreeFlightControllerBuilder {
    builder: CustomCameraControllerBuilder,
}

impl FreeFlightControllerBuilder {
    pub fn new() -> Self {
        Self {
            builder: CustomCameraControllerBuilder::new()
        }
    }

    pub fn build(&self) -> CameraControllerPlugin {
        self.builder.build()
    }
}

impl Default for FreeFlightControllerBuilder {
    fn default() -> Self {
        let mut camera_builder_config = Self::new();
        camera_builder_config.builder.with_movement();
        camera_builder_config.builder.with_rotation();

        return  camera_builder_config
    }
}