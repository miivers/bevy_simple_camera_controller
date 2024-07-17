mod common;

use bevy_simple_camera_controller::free_camera::*;
use bevy_simple_camera_controller::camera_properties::CameraProperties;
use bevy::prelude::*;
use common::utils;

fn main() {
    let mut app = App::new();

    app.add_plugins((
        DefaultPlugins,
        // Add camera plugin
        FreeCameraPlugin {
            properties: CameraProperties{
                movement_speed: 5.0,
                rotation_speed: 0.1,
                grab_mouse: false,
            }
        }
    ));

    app.add_systems(Startup, (
        FreeCameraPlugin::create_camera, // 2: Creates a default Camera3dBundle  (This is optionally, you can do this manually)
        utils::setup_scene
    ));

    app.run();
}