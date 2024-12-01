use bevy::{input::{mouse::{MouseButtonInput, MouseMotion}, ButtonState}, prelude::{EventReader, MouseButton}, transform::commands, window::{CursorGrabMode, CursorMoved}};

use bevy::prelude::*;

use crate::cube::*;

pub fn rotate(
    mut windows: Query<&mut Window>,
    mut mouse_button_input_events: EventReader<MouseButtonInput>,
    mut mouse_motion_events: EventReader<MouseMotion>,
    mut query: Query<&mut Transform, With<Rotator>>,
) {
    let mut window = windows.single_mut();

    for event in mouse_button_input_events.read() {
        if event.button == MouseButton::Right {
            if event.state == ButtonState::Pressed {
                info!("right button pressed");
                window.cursor.visible = false;
                window.cursor.grab_mode = CursorGrabMode::Locked;
            } else if event.state == ButtonState::Released {
                info!("right button released");
                window.cursor.visible = true;
                window.cursor.grab_mode = CursorGrabMode::None;
            }
        }
    }

    for event in mouse_motion_events.read() {
        if window.cursor.grab_mode == CursorGrabMode::Locked {
            info!("{:?}", event);
            for mut obj in &mut query {
                process_rotation(&mut obj, &event.delta);
            }
        }
    }
}

fn process_rotation(obj: &mut Transform, delta: &Vec2) {
    let c = 0.05;
    obj.rotate_x(delta.x * c);
    obj.rotate_y(delta.y * c);
}