use bevy::prelude::*;
use std::collections::{HashMap, HashSet};

#[derive(Debug, Hash, PartialEq, Eq, Clone)]
pub enum CameraAction {
    MoveForward,
    MoveBackward,
    MoveLeft,
    MoveRight,
    MoveUp,
    MoveDown,
}

#[derive(Debug, PartialEq, Clone)]
pub enum CameraRotationAction {
    Horizontal(f32),
    Vertical(f32),
}

pub type CameraRotationActions = Vec<CameraRotationAction>;
pub type CameraMovementActions = HashSet<CameraAction>;

#[derive(Event)]
pub struct CameraMovementEvents(pub CameraMovementActions);

#[derive(Event)]
pub struct CameraRotationEvents(pub CameraRotationActions);

#[derive(Event)]
pub struct HandleInput;

#[derive(Resource, Clone)]
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