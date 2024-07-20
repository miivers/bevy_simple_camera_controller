mod common;

use bevy_simple_camera_controller::prelude::*;
use bevy::prelude::*;
use common::utils;

fn main() {
    let mut app = App::new();

        app.add_plugins((
            DefaultPlugins,
            // 1: Setup camera
            CameraControllerPlugin::new(
                InitialPosition {
                    position: Vec3::new(-2.5, 4.5, 9.0),
                    look_at: Vec3::ZERO,
                    up_vector: Vec3::Y
                },
                CameraProperties::default(),
            )
        ));

        app.add_systems(Startup, (
            // 2: Create camera
            CameraControllerPlugin::create_camera,
            utils::setup_example_scene,
        ));
        app.run();
}