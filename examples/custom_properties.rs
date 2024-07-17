use bevy_simple_camera_controller::free_camera::{FreeCameraPlugin, CameraTag};
use bevy_simple_camera_controller::camera_properties::CameraProperties;
use bevy::prelude::*;

#[path = "utils.rs"]
mod utils;

fn main() {
    let mut app = App::new();

    app.add_plugins((
        DefaultPlugins,
        // Add camera plugin
        FreeCameraPlugin {
            properties: CameraProperties{
                movement_speed: 5.0,
                rotation_speed: 0.1,
            }
        }
    ));

    app.add_systems(Startup, (
        FreeCameraPlugin::create_camera, // 2: Creates a default Camera3dBundle  (This is optionally, you can do this manually)
        utils::setup_scene
    ));

    app.run();
}