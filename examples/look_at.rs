mod common;

use bevy_simple_camera_controller::prelude::*;
use bevy::prelude::*;
use rand::Rng;
use common::utils;

fn main() {
    let mut app = App::new();

    app.add_plugins((
        DefaultPlugins,
        // 1: Setup camera with teleport support
        // Note: Keyboard and mouse input is not enabled in this example
        // To enable it call "with_movement" and/or "with_rotation"
        FreeFlightControllerBuilder::default().
            with_look_at().
            build(),
    ));

    app.add_systems(Startup, (
        // 2: Create camera
        CameraControllerPlugin::create_camera,
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
        let target = generate_random_vec3((-5.0, 5.0), (-1.0, 1.0), (-2.0, 2.0));
        commands.trigger(LookAt{
            target
        });
    }
}

fn generate_random_vec3(x_bound: (f32, f32), y_bound: (f32, f32), z_bound: (f32, f32)) -> Vec3 {
    let mut rng = rand::thread_rng();
    let x = rng.gen_range(x_bound.0..=x_bound.1);
    let y = rng.gen_range(y_bound.0..=y_bound.1);
    let z = rng.gen_range(z_bound.0..=z_bound.1);
    Vec3::new(x, y, z)
}