# bevy_simple_camera_controller

Work in progress!  
I am new to rust and Bevy. This project will change as my understanding of bouth increases.
  
An attempt to make it easy to add camera controller to a bevy project. The main focus is ease of use.  
  
Currently only support keyboard and mouse.
  
```rust
use bevy_simple_camera_controller::free_camera::{FreeCameraPlugin, CameraTag};
use bevy::prelude::*;

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

pub(crate) fn setup_scene(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // cube
    commands.spawn(PbrBundle {
        mesh: meshes.add(Cuboid::new(1.0, 1.0, 1.0)),
        material: materials.add(Color::srgb_u8(124, 144, 255)),
        transform: Transform::from_xyz(0.0, 0.5, 0.0),
        ..default()
    });
}
```
