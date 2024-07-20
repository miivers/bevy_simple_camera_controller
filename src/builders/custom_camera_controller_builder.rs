use crate::camera_controller::CameraControllerPlugin;
use crate::prelude::{CameraProperties, InitialPosition};

#[derive(Default, Clone)]
pub struct CameraControllerBuilderData {
    pub with_movement: bool,
    pub with_free_flight_rotation: bool,
    pub with_orbit_rotation: bool,
    pub with_hide_cursor: bool,
    pub with_grab_cursor: bool,
    pub lock_y_axis_movement: bool,

    pub rotation_speed: Option<f32>,
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

    pub fn with_free_flight_rotation(&mut self) -> &mut Self {
        self.config.with_free_flight_rotation = true;
        self
    }

    pub fn with_orbit_rotation(&mut self) -> &mut Self {
        self.config.with_orbit_rotation = true;
        self
    }

    pub fn with_hide_cursor(&mut self) -> &mut Self {
        self.config.with_hide_cursor = true;
        self
    }

    pub fn with_grab_cursor(&mut self) -> &mut Self {
        self.config.with_grab_cursor = true;
        self
    }

    pub fn with_lock_y_axis_movement(&mut self) -> &mut Self {
        self.config.lock_y_axis_movement = true;
        self
    }

    pub fn set_rotation_speed(&mut self, speed: f32) -> &mut Self {
        self.config.rotation_speed = Some(speed);
        self
    }
}

impl Default for CustomCameraControllerBuilder {
     fn default() -> Self {
         let mut camera_builder_config = Self::new();
         camera_builder_config.with_movement();
         camera_builder_config.with_free_flight_rotation();

         return  camera_builder_config
    }
}