mod common;

use bevy::prelude::*;
use bevy_simple_camera_controller::prelude::*;
use common::utils;

fn main() {
    let mut app = App::new();

    app.add_plugins((
        DefaultPlugins,
        // Add camera plugin
        CameraControllerPlugin::new(InitialPosition {
                position: Vec3::new(-2.5, 4.5, 9.0),
                look_at: Vec3::ZERO,
                up_vector: Vec3::Y
            }, CameraProperties{
                input_enabled: true,
                hide_cursor: false,
            lock_y_axis_movement: false,
            movement_speed: 5.0,
                rotation_speed: 0.1,
                grab_cursor: true,
                key_bindings: CameraKeyBindings::wasd(),
            }, CameraControllerBuilderData {
            with_movement: true,
            with_free_flight_rotation: true,
            with_orbit_rotation: false,
            with_hide_cursor: false,
            with_grab_cursor: false,
            lock_y_axis_movement: false,
            rotation_speed: None,
        }
        )));

    app.add_systems(Startup, (
        manual_camera_creation,
        utils::setup_example_scene
    ));

    app.run();
}

pub fn manual_camera_creation(
    mut commands: Commands,
    initial_position: Res<InitialPosition>
) {
    commands.spawn((Camera3dBundle {
        transform: Transform::from_xyz(
            initial_position.position.x,
            initial_position.position.y,
            initial_position.position.z,
        ).looking_at(
            initial_position.look_at,
            initial_position.up_vector,
        ),
        ..default()
    },
    CameraTag
    ));
}