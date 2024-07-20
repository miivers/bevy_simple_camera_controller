use crate::camera_controller::CameraControllerPlugin;
use crate::prelude::CustomCameraControllerBuilder;


pub struct FreeFlightControllerBuilder {
    config: CustomCameraControllerBuilder,
}

impl FreeFlightControllerBuilder {
    pub fn new() -> Self {
        Self {
            config: CustomCameraControllerBuilder::new()
        }
    }

    pub fn with_movement(&mut self) ->  &mut Self {
        self.config.with_movement();
        self
    }

    pub fn with_rotation(&mut self) ->  &mut Self {
        self.config.with_rotation();
        self
    }

    pub fn build(&self) -> CameraControllerPlugin {
        self.config.build()
    }
}

impl Default for FreeFlightControllerBuilder {
    fn default() -> Self {
        let mut camera_builder_config = Self::new();
        camera_builder_config.with_movement();
        camera_builder_config.with_rotation();

        return  camera_builder_config
    }
}