use bevy::prelude::*;
use std::collections::HashMap;

#[derive(Debug, Hash, PartialEq, Eq, Clone)]
pub enum CameraAction {
    MoveForward,
    MoveBackward,
    MoveLeft,
    MoveRight,
    MoveUp,
    MoveDown
}

#[derive(Resource)]
pub struct CameraKeyBindings {
    pub bindings: HashMap<CameraAction, KeyCode>,
}

impl CameraKeyBindings {
    pub fn wasd() -> Self {
        let mut bindings = HashMap::new();
        bindings.insert(CameraAction::MoveForward, KeyCode::KeyW);
        bindings.insert(CameraAction::MoveBackward, KeyCode::KeyS);
        bindings.insert(CameraAction::MoveLeft, KeyCode::KeyA);
        bindings.insert(CameraAction::MoveRight, KeyCode::KeyD);
        bindings.insert(CameraAction::MoveUp, KeyCode::KeyE);
        bindings.insert(CameraAction::MoveDown, KeyCode::KeyQ);

        Self { bindings }
    }
}

impl Default for CameraKeyBindings {
    fn default() -> Self {
        CameraKeyBindings::wasd()
    }
}