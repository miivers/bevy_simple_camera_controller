use bevy_simple_camera_controller::free_camera::{FreeCameraPlugin};
use bevy::prelude::*;

#[path = "utils.rs"]
mod utils;

fn main() {
    let mut app = App::new();

        app.add_plugins((
            DefaultPlugins,
            FreeCameraPlugin::default() // 1: Add camera plugin
        ));

        app.add_systems(Startup, (
            FreeCameraPlugin::create_camera, // 2: Creates a default Camera3dBundle  (This is optionally, you can do this manually)
            utils::setup_scene
        ));

        app.run();
}