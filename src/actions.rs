use std::{collections::VecDeque, f32::consts::FRAC_PI_2};
use bevy::{input::{mouse::{MouseButtonInput, MouseMotion}, ButtonState}, prelude::{EventReader, MouseButton}, window::{CursorGrabMode, CursorMoved}};

use bevy::prelude::*;

use crate::cube::{Cube, CubeInfo, Direction, Movement, RotateAxis, Rotator};

#[derive(Resource)]
pub struct ActionStatus {
    pub angle_to_process: f32,
    pub action_queue: VecDeque<Movement>,
    pub cur_action: Option<Movement>,
}

pub fn rotate_cube(
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
    let c = 0.005;
    let quat_y = Quat::from_euler(
        EulerRot::XYZ,
        delta.y * c,
        0.0,
        -delta.y * c,
    );

    let quat_x = Quat::from_euler(
        EulerRot::XYZ,
        0.0,
        delta.x * c,
        0.0
    );

    obj.rotate(quat_x);
    obj.rotate(quat_y);
}

pub fn frame_handler(
    time: Res<Time>,
    mut query: Query<&mut Transform>,
    cubes: Query<&Cube>,
    cube_info: Res<CubeInfo>,
    mut status: ResMut<ActionStatus>,
) {

    if status.action_queue.is_empty() && status.cur_action.is_none() {
        return;
    }

    let movement = if let Some(cur_action) = status.cur_action {
        cur_action
    } else {
        let m = status.action_queue.pop_front().unwrap();
        status.cur_action = Some(m);
        status.angle_to_process = FRAC_PI_2;
        m
    };

    let axis = query.get(match movement.axis {
        RotateAxis::X => cube_info.x.unwrap(),
        RotateAxis::Y => cube_info.y.unwrap(),
        RotateAxis::Z => cube_info.z.unwrap(),
    }).unwrap();

    let coord_idx = movement.axis as usize;
    let angle = f32::min(FRAC_PI_2 * time.delta_seconds(), status.angle_to_process);
    status.angle_to_process -= angle;

    if status.angle_to_process == 0.0 {
        status.cur_action = None;
    }

    let quat = match movement.direction {
        Direction::Clockwise => Quat::from_axis_angle(axis.translation, angle),
        Direction::CounterClockwise => Quat::from_axis_angle(axis.translation, -angle)
    };

    for e in cube_info.cubes.iter() {
        let cube = cubes.get(*e).unwrap();
        if cube.coord[coord_idx] == movement.layer {
            if let Ok(mut cube_transform) = query.get_mut(*e) {
                cube_transform.rotate_around(Vec3::ZERO, quat);
            } else {
                panic!("Something weird happened");
            }
        }
    }
}
