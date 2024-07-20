mod common;

use bevy_simple_camera_controller::prelude::*;
use bevy::prelude::*;
use common::utils;

fn main() {
    let mut app = App::new();

    app.add_plugins((
        DefaultPlugins,
        // 1: Setup camera
        TopDownControllerBuilder::default().build()
    ));

    app.add_systems(Startup, (
        // 2: Create camera
        CameraControllerPlugin::create_camera,
        utils::setup_example_scene,
    ));
    app.run();
}