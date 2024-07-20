use crate::camera_controller::CameraControllerPlugin;
use crate::prelude::{CameraProperties, InitialPosition};

#[derive(Default, Clone)]
pub struct CameraControllerBuilderData {
    pub with_movement: bool,
    pub with_rotation: bool,
    pub lock_y_axis_movement: bool,
}

pub struct CustomCameraControllerBuilder {
    config: CameraControllerBuilderData,
}

impl CustomCameraControllerBuilder {
    pub fn new() -> Self {
        Self {
            config: CameraControllerBuilderData::default()
        }
    }

    pub fn build(&self) -> CameraControllerPlugin {
        CameraControllerPlugin::new(
            InitialPosition::default(),
            CameraProperties::default(),
            self.config.clone(),
        )
    }

    pub fn with_movement(&mut self) -> &mut Self {
        self.config.with_movement = true;
        self
    }

    pub fn with_rotation(&mut self) -> &mut Self {
        self.config.with_rotation = true;
        self
    }

    pub fn with_lock_y_axis_movement(&mut self) -> &mut Self {
        self.config.lock_y_axis_movement = true;
        self
    }
}

impl Default for CustomCameraControllerBuilder {
     fn default() -> Self {
         let mut camera_builder_config = Self::new();
         camera_builder_config.with_movement();
         camera_builder_config.with_rotation();

         return  camera_builder_config
    }
}