mod common;

use bevy_simple_camera_controller::prelude::*;
use bevy::prelude::*;
use rand::Rng;
use bevy_simple_camera_controller::controllers::teleport::Teleport;
use common::utils;

fn main() {
    let mut app = App::new();

    app.add_plugins((
        DefaultPlugins,
        // 1: Setup camera
        FreeFlightCameraPlugin::default(),
    ));

    app.add_systems(Startup, (
        // 2: Create camera
        FreeFlightCameraPlugin::create_camera,
        utils::setup_example_scene,
    ));

    app.add_systems(Update, space_key_system);
    app.run();
}

fn space_key_system(
    mut commands: Commands,
    keys: Res<ButtonInput<KeyCode>>,
) {
    if keys.just_pressed(KeyCode::Space) {
        let mut rng = rand::thread_rng();
        let x = rng.gen_range(-2 ..= 2);

        commands.trigger(Teleport{
            target: Vec3::new(
                x as f32,
                4.5,
                9.0
            )
        });
    }
}