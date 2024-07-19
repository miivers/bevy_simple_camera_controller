# bevy_simple_camera_controller

Work in progress! Made for Bevy 0.14.0  
I am new to rust and Bevy. This project will change as my understanding of bouth increases.
  
An attempt to make it easy to add camera controller to a bevy project. The main focus is ease of use.  
  
Currently only support keyboard and mouse.

## Features
- Free flight camera style
- Keyboard input for movement (default = wasd)
- Mouse input for rotate
- Change keybinding for movement
- Capture input (optional, default = on) - user input is disabled until user clicks window


## Example

```rust
mod common;

use bevy_simple_camera_controller::camera_properties::{CameraProperties, InitialPosition};
use bevy_simple_camera_controller::free_camera::*;
use bevy::prelude::*;
use common::utils;

fn main() {
    let mut app = App::new();

        app.add_plugins((
            DefaultPlugins,
            // 1: Setup camera
            FreeCameraPlugin::new(
                InitialPosition {
                    position: Vec3::new(-2.5, 4.5, 9.0),
                    look_at: Vec3::ZERO,
                    up_vector: Vec3::Y
                },
                CameraProperties::default(),
            )
        ));

        app.add_systems(Startup, (
            // 2: Create camera
            FreeCameraPlugin::create_camera,
            utils::setup_example_scene,
        ));
        app.run();
}
```

## Compatibility

| Bevy  | bevy_simple_camera_controller |
|:------|:------------------------------|
| `0.14`| `0.1.0`                       |

