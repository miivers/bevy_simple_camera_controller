mod common;

use bevy_simple_camera_controller::camera_common::CameraTag;
use bevy_simple_camera_controller::free_camera::*;
use bevy::prelude::*;
use common::utils;


fn main() {
    let mut app = App::new();

        app.add_plugins((
            DefaultPlugins,
            FreeCameraPlugin::default() // 1: Add camera plugin
        ));

        app.add_systems(Startup, (
            create_camera, // 2: Creates a default Camera3dBundle  (This is optionally, you can do this manually)
            utils::setup_scene
        ));

        app.run();
}

fn create_camera(mut commands: Commands) {
    commands.spawn((Camera3dBundle {
        transform: Transform::from_xyz(-2.5, 4.5, 9.0).looking_at(Vec3::ZERO, Vec3::Y), // Camera position and look at position
        ..default()
    },
    CameraTag
    ));
}