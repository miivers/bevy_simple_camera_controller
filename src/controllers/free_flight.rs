use bevy::prelude::{Camera3dBundle, Commands, EventReader, Query, Res, Transform, With};
use bevy::app::{Plugin, App, Update};
use bevy::math::{Quat, Vec3};
use bevy::time::{Real, Time};
use bevy::utils::default;
use crate::camera_common::{CameraTag, capture_cursor, disable_capture_cursor};
use crate::camera_properties::{CameraProperties, InitialPosition};
use crate::controllers::input::{handle_disable_input, handle_keyboard_input, handle_mouse_input};
use crate::key_binding::{CameraAction, CameraMovementEvents, CameraRotationAction, CameraRotationEvents};

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

fn update_movement(
    mut query: Query<&mut Transform, With<CameraTag>>,
    properties: Res<CameraProperties>,
    mut movement_events: EventReader<CameraMovementEvents>,
    time: Res<Time<Real>>,
) {
    if movement_events.is_empty() {
        return;
    }

    let mut movement_vector = Vec3::ZERO;

    let mut camera_transform = query.single_mut();

    for events in movement_events.read() {
        for event in &events.0 {
            match event {
                CameraAction::MoveForward  => movement_vector += Vec3::from(camera_transform.forward()),
                CameraAction::MoveBackward => movement_vector -= Vec3::from(camera_transform.forward()),
                CameraAction::MoveLeft     => movement_vector -= Vec3::from(camera_transform.right()),
                CameraAction::MoveRight    => movement_vector += Vec3::from(camera_transform.right()),
                CameraAction::MoveUp       => movement_vector += Vec3::from(camera_transform.up()),
                CameraAction::MoveDown     => movement_vector -= Vec3::from(camera_transform.up()),
            }
        }
    }

    camera_transform.translation += movement_vector.normalize() * time.delta_seconds() * properties.movement_speed;
}

fn update_rotation(
    mut query: Query<&mut Transform, With<CameraTag>>,
    properties: Res<CameraProperties>,
    mut rotation_events: EventReader<CameraRotationEvents>,
    time: Res<Time<Real>>,
) {
    if rotation_events.is_empty() {
        return;
    }

    let mut yaw_rotation: f32 = 0.;
    let mut pitch_rotation: f32 = 0.;

    let mut camera_transform = query.single_mut();

    for events in rotation_events.read() {
        for event in &events.0 {
            match event {
                CameraRotationAction::Horizontal(delta) => yaw_rotation += delta,
                CameraRotationAction::Vertical(delta)   => pitch_rotation += delta,
            }
        }
    }

    camera_transform.rotation = Quat::from_rotation_y(-yaw_rotation * properties.rotation_speed * time.delta_seconds()) * camera_transform.rotation;
    camera_transform.rotation = camera_transform.rotation * Quat::from_rotation_x(-pitch_rotation * properties.rotation_speed * time.delta_seconds());
}