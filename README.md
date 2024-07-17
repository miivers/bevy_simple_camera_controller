# bevy_simple_camera_controller

Work in progress!  
I am new to rust and Bevy. This project will change as my understanding of bouth increases.
  
An attempt to make it easy to add camera controller to a bevy project. The main focus is ease of use.  
  
Currently only support keyboard and mouse.
  
```rust
use bevy_simple_camera_controller::free_camera::{FreeCameraPlugin, CameraTag};
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
            FreeCameraPlugin::create_camera, // 2: Creates a default Camera3dBundle (This is optionally, you can do this manually)
            utils::setup_scene
        ));
    
        app.run();
}
```
