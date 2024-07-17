use bevy::app::{Plugin, App, PreUpdate};
use bevy::ecs::component::Component;
use bevy::input::ButtonInput;
use bevy::math::Vec3;
use bevy::prelude::{KeyCode, Query, Res, Transform, With};
use bevy::time::{Real, Time};

#[derive(Component)]
pub struct CameraTag;

pub struct FreeCameraPlugin;

impl Plugin for FreeCameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PreUpdate, update);
    }
}

fn update(
    keys: Res<ButtonInput<KeyCode>>,
    mut query: Query<&mut Transform, With<CameraTag>>,
    time: Res<Time<Real>>,
) {
    for mut transform in &mut query {
        let mut movement_vector = Vec3::ZERO;

        if keys.pressed(KeyCode::KeyW) {
            println!("W pressed");
            movement_vector += Vec3::from(transform.forward());
        }

        if keys.pressed(KeyCode::KeyS) {
            println!("S pressed");
            movement_vector -= Vec3::from(transform.forward());
        }

        if keys.pressed(KeyCode::KeyA) {
            println!("A pressed");
            movement_vector -= Vec3::from(transform.right());
        }

        if keys.pressed(KeyCode::KeyD) {
            println!("A pressed");
            movement_vector += Vec3::from(transform.right());
        }

        let movement_speed: f32 = 10.0;

        if movement_vector != Vec3::ZERO {
            transform.translation += movement_vector.normalize() * time.delta_seconds() * movement_speed;
        }
    }
}