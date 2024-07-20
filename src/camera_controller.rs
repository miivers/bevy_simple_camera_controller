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
use crate::controllers::rotation::update_rotation;
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
        Self {
            initial_position,
            properties,
            builder_config
        }
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

        if self.properties.grab_mouse {
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

        if self.builder_config.with_rotation {
            app.add_event::<CameraRotationEvents>();
            app.observe(handle_mouse_input);

            app.add_systems(Update, update_rotation);
        }

        app.add_event::<LookAt>();
        app.observe(handle_look_at);

        app.add_event::<Teleport>();
        app.observe(handle_teleport);
    }
}