# bevy_simple_camera_controller

Work in progress!  
3d camera controller made for Bevy 0.14.0  
I am new to rust and Bevy. This project will change as my understanding of both increases.

An attempt to make it easy to add camera controller to a bevy project. The main focus is ease of use.  
## Camera types
- Free flight
- Top down  
- Orbit
- Custom camera through builder pattern

## Input
- Keyboard
- Mouse
- Custom key binding for Keyboard

## Features
- Capture cursor on focus. Cursor is allways in the middle of the screen (escape to cancel)
- Hide cursor.


## Example

### Free flight
Move camera with WASD.  
Rotate camera with mouse.

```rust
mod common;

use bevy_simple_camera_controller::prelude::*;
use bevy::prelude::*;
use common::utils;

fn main() {
    let mut app = App::new();

        app.add_plugins((
            DefaultPlugins,
            // 1: Setup camera
            FreeFlightControllerBuilder::default().build()
        ));

        app.add_systems(Startup, (
            // 2: Create camera
            CameraControllerPlugin::create_camera,
            utils::setup_example_scene,
        ));
        app.run();
}
```
  
### Top Down
Move camera with WASD.  

```rust
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
```

### Orbit
Rotate camera with mouse

```rust
mod common;

use bevy_simple_camera_controller::prelude::*;
use bevy::prelude::*;
use common::utils;

fn main() {
    let mut app = App::new();

    app.add_plugins((
        DefaultPlugins,
        // 1: Setup camera
        OrbitControllerBuilder::default().
            with_grab_cursor().
            set_rotation_speed(2.0).
            build()
    ));

    app.add_systems(Startup, (
        // 2: Create camera
        CameraControllerPlugin::create_camera,
        // Adds CameraOrbitTag to cube in order to set it as rotation target
        utils::setup_example_scene,
    ));
    app.run();
}
```

## Compatibility

| Bevy  | bevy_simple_camera_controller |
|:------|:------------------------------|
| `0.14`| `0.1.0`                       |

