use bevy::input::ButtonState;
use bevy::input::mouse::MouseButtonInput;
use bevy::prelude::*;
use bevy::window::{CursorGrabMode, PrimaryWindow};

#[derive(Component)]
pub struct CameraTag;

pub fn capture_cursor(
    mut q_windows: Query<&mut Window, With<PrimaryWindow>>,
    mut mouse_button_event: EventReader<MouseButtonInput>,
) {
    let mut primary_window = q_windows.single_mut();

    if primary_window.cursor.grab_mode != CursorGrabMode::Locked {
        for event in mouse_button_event.read() {
            if event.state == ButtonState::Pressed && event.button == MouseButton::Left {
                if let Some(position) = primary_window.cursor_position() {
                    let window_size = Vec2::new(primary_window.width() as f32, primary_window.height() as f32);
                    if position.x >= 0.0 && position.x <= window_size.x && position.y >= 0.0 && position.y <= window_size.y {
                        primary_window.cursor.grab_mode = CursorGrabMode::Locked;
                        primary_window.cursor.visible = false;
                    }
                }
            }
        }
    }
}

pub fn disable_capture_cursor(
    mut q_windows: Query<&mut Window, With<PrimaryWindow>>,
    keys: Res<ButtonInput<KeyCode>>,
) {
    if keys.pressed(KeyCode::Escape) {
        let mut primary_window = q_windows.single_mut();

        primary_window.cursor.grab_mode = CursorGrabMode::None;
        primary_window.cursor.visible = true;
    }
}