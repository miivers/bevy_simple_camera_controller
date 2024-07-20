use bevy::prelude::{Camera3dBundle, Commands, Res, Transform};
use bevy::app::{Plugin, App, Update};
use bevy::utils::default;
use crate::builders::custom_camera_controller_builder::CameraControllerBuilderData;
use crate::controllers::look_at::{handle_look_at, LookAt};
use crate::input::capture_cursor::{CameraTag, capture_cursor, disable_capture_cursor};
use crate::data::camera_properties::{CameraProperties, InitialPosition};
use crate::input::input::{handle_disable_input, handle_keyboard_input, handle_mouse_input};
use crate::data::key_binding::{CameraMovementEvents, CameraRotationEvents};
use crate::controllers::movement::update_movement;
use crate::controllers::free_flight_rotation::update_free_flight_rotation;
use crate::controllers::orbit_rotation::update_orbit_rotation;
use crate::controllers::teleport::{handle_teleport, Teleport};

#[derive(Default)]
pub struct CameraControllerPlugin {
    pub initial_position: InitialPosition,
    pub properties: CameraProperties,
    pub builder_config: CameraControllerBuilderData,
}

impl CameraControllerPlugin {
    pub fn new(
        initial_position: InitialPosition,
        properties: CameraProperties,
        builder_config: CameraControllerBuilderData,
    ) -> Self {
        let mut camera_controller = Self {
            initial_position,
            properties,
            builder_config
        };

        camera_controller.properties.lock_y_axis_movement = camera_controller.builder_config.lock_y_axis_movement;
        camera_controller.properties.hide_cursor = camera_controller.builder_config.with_hide_cursor;
        camera_controller.properties.grab_cursor = camera_controller.builder_config.with_grab_cursor;

        if let Some(rotation_speed) = camera_controller.builder_config.rotation_speed {
            camera_controller.properties.rotation_speed = rotation_speed;
        }

        return camera_controller;
    }

    pub fn create_camera(
        mut commands: Commands,
        initial_position: Res<InitialPosition>
    ) {
        commands.spawn((Camera3dBundle {
            transform: Transform::from_xyz(
                initial_position.position.x,
                initial_position.position.y,
                initial_position.position.z,
            ).looking_at(
                initial_position.look_at,
                initial_position.up_vector,
            ),
            ..default()
        },
            CameraTag
        ));
    }
}

impl Plugin for CameraControllerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, handle_disable_input);

        if self.properties.grab_cursor {
            app.add_systems(Update, capture_cursor);
            app.add_systems(Update, disable_capture_cursor);
        }

        app.insert_resource(self.initial_position.clone());
        app.insert_resource(self.properties.clone());
        app.insert_resource(self.properties.key_bindings.clone());

        if self.builder_config.with_movement {
            app.add_event::<CameraMovementEvents>();
            app.observe(handle_keyboard_input);

            app.add_systems(Update, update_movement);
        }

        let multiple_rotation_controllers =
            self.builder_config.with_free_flight_rotation == true &&
            self.builder_config.with_orbit_rotation == true;
        assert_eq!(multiple_rotation_controllers, false, "Multiple rotation controllers is not supported");

        if self.builder_config.with_free_flight_rotation {
            app.add_event::<CameraRotationEvents>();
            app.observe(handle_mouse_input);

            app.add_systems(Update, update_free_flight_rotation);
        }

        if self.builder_config.with_orbit_rotation {
            app.add_event::<CameraRotationEvents>();
            app.observe(handle_mouse_input);

            app.add_systems(Update, update_orbit_rotation);
        }

        app.add_event::<LookAt>();
        app.observe(handle_look_at);

        app.add_event::<Teleport>();
        app.observe(handle_teleport);
    }
}