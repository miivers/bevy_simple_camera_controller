use std::collections::HashMap;
use bevy::prelude::*;
use bevy_simple_camera_controller::prelude::{CameraAction, CameraControllerBuilderData, CameraControllerPlugin, CameraKeyBindings, CameraProperties};
use crate::common::utils;

mod common;


fn main() {
    let mut app = App::new();

    let mut bindings = HashMap::new();
    bindings.insert(CameraAction::MoveForward, KeyCode::ArrowUp);
    bindings.insert(CameraAction::MoveBackward, KeyCode::ArrowDown);
    bindings.insert(CameraAction::MoveLeft, KeyCode::ArrowLeft);
    bindings.insert(CameraAction::MoveRight, KeyCode::ArrowRight);

    app.add_plugins((
        DefaultPlugins,
        // Add camera plugin
        CameraControllerPlugin {
            initial_position: Default::default(),
            properties: CameraProperties{
                input_enabled: true,
                hide_cursor: true,
                movement_speed: 5.0,
                rotation_speed: 0.1,
                grab_mouse: true,
                key_bindings: CameraKeyBindings {
                    bindings
                }
            },
            builder_config: CameraControllerBuilderData {
                with_movement: true,
                with_rotation: true,
            }
        }
    ));

    app.add_systems(Startup, (
        CameraControllerPlugin::create_camera, // 2: Creates a default Camera3dBundle  (This is optionally, you can do this manually)
        utils::setup_example_scene
    ));

    app.run();
}