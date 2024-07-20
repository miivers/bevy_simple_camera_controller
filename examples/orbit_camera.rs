mod common;

use bevy_simple_camera_controller::prelude::*;
use bevy::prelude::*;
use common::utils;

fn main() {
    let mut app = App::new();

    app.add_plugins((
        DefaultPlugins,
        // 1: Setup camera
        OrbitControllerBuilder::default().
            with_grab_cursor().
            set_rotation_speed(2.0).
            build()
    ));

    app.add_systems(Startup, (
        // 2: Create camera
        CameraControllerPlugin::create_camera,
        // Adds CameraOrbitTag to cube in order to set it as rotation target
        utils::setup_example_scene,
    ));
    app.run();
}