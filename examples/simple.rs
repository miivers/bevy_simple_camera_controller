use bevy_simple_camera_controller::free_camera::{FreeCameraPlugin, CameraTag};
use bevy::prelude::*;

#[path = "utils.rs"]
mod utils;

fn main() {
    let mut app = App::new();

        app.add_plugins((
            DefaultPlugins,
            FreeCameraPlugin // Add camera plugin
        ));

        app.add_systems(Startup, (
            setup_camera, // Example setup function
            utils::setup_scene
        ))
        .run();
}

fn setup_camera(mut commands: Commands) {
    // camera
    commands.spawn((Camera3dBundle {
        transform: Transform::from_xyz(-2.5, 4.5, 9.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    },
        CameraTag
    ));
}