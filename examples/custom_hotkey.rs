mod common;

use std::collections::HashMap;
use bevy_simple_camera_controller::free_flight::*;
use bevy_simple_camera_controller::camera_properties::CameraProperties;
use bevy::prelude::*;
use bevy_simple_camera_controller::key_binding::{CameraAction, CameraKeyBindings};
use common::utils;

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
        FreeCameraPlugin {
            properties: CameraProperties{
                movement_speed: 5.0,
                rotation_speed: 0.1,
                grab_mouse: true,
                key_bindings: CameraKeyBindings {
                    bindings
                }
            }
        }
    ));

    app.add_systems(Startup, (
        FreeCameraPlugin::create_camera, // 2: Creates a default Camera3dBundle  (This is optionally, you can do this manually)
        utils::setup_scene
    ));

    app.run();
}