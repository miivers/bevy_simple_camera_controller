use bevy::prelude::{Camera3dBundle, Commands, Res, Transform};
use bevy::app::{Plugin, App, Update};
use bevy::utils::default;
use crate::input::capture_cursor::{CameraTag, capture_cursor, disable_capture_cursor};
use crate::data::camera_properties::{CameraProperties, InitialPosition};
use crate::input::input::{handle_disable_input, handle_keyboard_input, handle_mouse_input};
use crate::data::key_binding::{CameraMovementEvents, CameraRotationEvents};
use crate::controllers::movement::update_movement;
use crate::controllers::rotation::update_rotation;

#[derive(Default)]
pub struct FreeFlightCameraPlugin {
    pub initial_position: InitialPosition,
    pub properties: CameraProperties,
}

impl FreeFlightCameraPlugin {
    pub fn new(
        initial_position: InitialPosition,
        properties: CameraProperties
    ) -> Self {
        Self {
            initial_position,
            properties,
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

impl Plugin for FreeFlightCameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<CameraMovementEvents>();
        app.add_event::<CameraRotationEvents>();

        app.add_systems(Update, handle_disable_input);
        app.add_systems(Update, update_movement);
        app.add_systems(Update, update_rotation);

        app.observe(handle_keyboard_input);
        app.observe(handle_mouse_input);

        if self.properties.grab_mouse {
            app.add_systems(Update, capture_cursor);
            app.add_systems(Update, disable_capture_cursor);
        }

        app.insert_resource(self.initial_position.clone());
        app.insert_resource(self.properties.clone());
        app.insert_resource(self.properties.key_bindings.clone());
    }
}