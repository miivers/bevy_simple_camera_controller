//These systems translates input into camera actions

use bevy::input::mouse::MouseMotion;
use bevy::prelude::*;
use bevy::window::{CursorGrabMode, PrimaryWindow};
use crate::camera_properties::CameraProperties;
use crate::key_binding::{CameraMovementActions, CameraKeyBindings, CameraRotationActions, CameraMovementEvents, CameraRotationEvents, HandleInput};
use crate::prelude::CameraRotationAction;

pub fn handle_disable_input(
    mut commands: Commands,
    mut q_windows: Query<&mut Window, With<PrimaryWindow>>,
    properties: Res<CameraProperties>,
) {
    if !properties.input_enabled {
        return;
    }

    if properties.grab_mouse {
        // We use CursorGrabMode::Locked to signal the user has clicked and given focus to the window
        if q_windows.single_mut().cursor.grab_mode != CursorGrabMode::Locked {
            return;
        }
    }

    commands.trigger(HandleInput);
}

pub fn handle_keyboard_input(
    _: Trigger<HandleInput>,
    key_bindings: Res<CameraKeyBindings>,
    keys: Res<ButtonInput<KeyCode>>,
    mut camera_movement_event_writer: EventWriter<CameraMovementEvents>,
) {
    let mut camera_actions = CameraMovementActions::new();

    for (action, key) in &key_bindings.bindings {
        if keys.pressed(key.clone()) {
            camera_actions.insert(action.clone());
        }
    }

    if !camera_actions.is_empty() {
        camera_movement_event_writer.send(CameraMovementEvents(camera_actions));
    }
}

pub fn handle_mouse_input(
    _: Trigger<HandleInput>,
    mut mouse_motion_events: EventReader<MouseMotion>,
    mut camera_rotation_event_writer: EventWriter<CameraRotationEvents>,
) {
    let mut camera_actions = CameraRotationActions::new();

    for mouse_event in mouse_motion_events.read() {
        camera_actions.push(CameraRotationAction::Horizontal(mouse_event.delta.x));
        camera_actions.push(CameraRotationAction::Vertical(mouse_event.delta.y));
    }

    if !camera_actions.is_empty() {
        camera_rotation_event_writer.send(CameraRotationEvents(camera_actions));
    }
}