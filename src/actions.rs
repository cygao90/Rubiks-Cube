use std::{collections::VecDeque, f32::consts::FRAC_PI_2};

use bevy::prelude::*;
use bevy_mod_picking::prelude::*;
use bevy_mod_picking::prelude::Listener;
use bevy::math::Vec3;
use rand::Rng;
use crate::{cube::{Cube, CubeInfo, Direction, Movement, RotateAxis, RotateX, RotateY, RotateZ, Face}, settings::Settings};

#[derive(Resource)]
pub struct ActionStatus {
    pub angle_to_process: f32,
    pub action_queue: VecDeque<Movement>,
    pub cur_action: Option<Movement>,
    pub drag_start: Option<Vec3>,
    pub drag_end: Option<Vec3>,
    pub selected_entity: Option<Entity>,
    pub computing_solution: bool,
}

pub fn handle_drag_start(
    event: Listener<Pointer<DragStart>>,
    mut status: ResMut<ActionStatus>,
) {
    if status.computing_solution {
        return;
    }
    if event.button == PointerButton::Secondary {
        return;
    }
    info!("drag start at {:?}", event.hit.position);
    status.drag_start = event.hit.position;
    status.selected_entity = Some(event.target);
}

pub fn handle_drag_move(
    event: Listener<Pointer<Move>>,
    rx: Query<&Transform, With<RotateX>>,
    ry: Query<&Transform, With<RotateY>>,
    rz: Query<&Transform, With<RotateZ>>,
    cubes: Query<&Cube>,
    mut status: ResMut<ActionStatus>,
    settings: Res<Settings>
) {
    if status.computing_solution {
        return;
    }

    let c = settings.rotation_trigger_value;
    if !(status.drag_start.is_some() && status.drag_end.is_none()) {
        return;
    }

    let start_pos = status.drag_start.unwrap();
    let cur_pos = event.hit.position.unwrap();
    info!("{:?}", cur_pos);

    if start_pos.distance(cur_pos) < c {
        return;
    }

    status.drag_end = Some(cur_pos);
    let rotation_quat = Quat::from_rotation_arc(Vec3::from_array([1.0, 0.0, 0.0]), rx.single().translation);

    let drag_vec = rotation_quat * (cur_pos - start_pos).normalize();
    let binding = vec![
        (drag_vec.dot(rx.single().translation), RotateAxis::X),
        (drag_vec.dot(ry.single().translation), RotateAxis::Y),
        (drag_vec.dot(rz.single().translation), RotateAxis::Z),
    ];
    info!("dot with xyz: {:?}, vec: {:?}", binding, drag_vec);
    let (_, vertical_axis) = binding.iter().min_by(|a, b| a.0.abs().partial_cmp(&b.0.abs()).unwrap()).unwrap();
    let (axis, dir) = match vertical_axis {
        // rotate along y or z
        RotateAxis::X => {
            if drag_vec.y.abs() > drag_vec.z.abs() {
                // along z
                (RotateAxis::Z, if drag_vec.y * start_pos.x > 0.0 { Direction::CounterClockwise } else { Direction::Clockwise })
            } else {
                // along y
                (RotateAxis::Y, if drag_vec.z * start_pos.x > 0.0 { Direction::Clockwise } else { Direction::CounterClockwise })
            }
        },
        // rotate along x or z
        RotateAxis::Y => {
            if drag_vec.x.abs() > drag_vec.z.abs() {
                // z
                (RotateAxis::Z, if drag_vec.x * start_pos.y > 0.0 { Direction::Clockwise } else { Direction::CounterClockwise })
            } else {
                // x
                (RotateAxis::X, if drag_vec.z * start_pos.y > 0.0 { Direction::CounterClockwise } else { Direction::Clockwise })
            }
        },
        RotateAxis::Z => {
            if drag_vec.x.abs() > drag_vec.y.abs() {
                // y
                (RotateAxis::Y, if drag_vec.x * start_pos.z > 0.0 { Direction::CounterClockwise } else { Direction::Clockwise })
            } else {
                // x
                (RotateAxis::X, if drag_vec.y * start_pos.z > 0.0 { Direction::Clockwise } else { Direction::CounterClockwise })
            }
        },
    };

    let layer = cubes.get(status.selected_entity.unwrap()).unwrap().coord[axis as usize];

    let m = Movement {
        axis: axis,
        layer: layer as u32,
        direction: dir
    };
    info!("generate movement: {:?}", m);
    status.action_queue.push_back(m);

}

pub fn handle_drag_end(
    mut status: ResMut<ActionStatus>,
) {
    if status.computing_solution {
        return;
    }
    status.drag_end = None;
    status.drag_start = None;
    status.selected_entity = None;
}

pub fn frame_handler(
    time: Res<Time>,
    mut query: Query<&mut Transform>,
    mut cubes: Query<&mut Cube>,
    cube_info: Res<CubeInfo>,
    mut status: ResMut<ActionStatus>,
    settings: Res<Settings>
) {

    let c = settings.layer_rotation_speed;

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
    let angle = f32::min(FRAC_PI_2 * time.delta_seconds() * c, status.angle_to_process);
    status.angle_to_process -= angle;

    if status.angle_to_process == 0.0 {
        status.cur_action = None;
    }

    let quat = match movement.direction {
        Direction::Clockwise => Quat::from_axis_angle(axis.translation, -angle),
        Direction::CounterClockwise => Quat::from_axis_angle(axis.translation, angle)
    };

    for e in cube_info.cubes.iter() {
        let mut cube = cubes.get_mut(*e).unwrap();
        if cube.coord[coord_idx] == movement.layer as i32 {
            if let Ok(mut cube_transform) = query.get_mut(*e) {
                cube_transform.rotate_around(Vec3::ZERO, quat);
            } else {
                panic!("Something weird happened");
            }
            if status.angle_to_process == 0.0 {
                adjust_coords(&mut cube, &movement, &settings);
                adjust_faces(&mut cube, &movement);
            }
        }
    }
}

fn adjust_coords(cube: &mut Cube, movement: &Movement, settings: &Settings) {
    // let old = cube.coord;
    if movement.axis == RotateAxis::X {
        cube.coord.swap(1, 2);
        if movement.direction == Direction::Clockwise {
            cube.coord[2] = settings.layers as i32 - cube.coord[2] - 1;
        } else if movement.direction == Direction::CounterClockwise {
            cube.coord[1] = settings.layers as i32 - cube.coord[1] - 1;
        }
    } else if movement.axis == RotateAxis::Y {
        cube.coord.swap(0, 2);
        if movement.direction == Direction::Clockwise {
            cube.coord[0] = settings.layers as i32 - cube.coord[0] - 1;
        } else if movement.direction == Direction::CounterClockwise {
            cube.coord[2] = settings.layers as i32 - cube.coord[2] - 1;
        }
    } else if movement.axis == RotateAxis::Z {
        cube.coord.swap(0, 1);
        if movement.direction == Direction::Clockwise {
            cube.coord[1] = settings.layers as i32 - cube.coord[1] - 1;
        } else if movement.direction == Direction::CounterClockwise {
            cube.coord[0] = settings.layers as i32 - cube.coord[0] - 1;
        }
    }

    // info!("{:?} -> {:?}", old, cube.coord);
}

fn adjust_faces(cube: &mut Cube, movement: &Movement) {
    let up = *cube.colors.get(&Face::UP).unwrap();
    let down = *cube.colors.get(&Face::DOWN).unwrap();
    let front = *cube.colors.get(&Face::FRONT).unwrap();
    let back = *cube.colors.get(&Face::BACK).unwrap();
    let left = *cube.colors.get(&Face::LEFT).unwrap();
    let right = *cube.colors.get(&Face::RIGHT).unwrap();
    if movement.axis == RotateAxis::X {
        if movement.direction == Direction::Clockwise {
            cube.colors.insert(Face::UP, front);
            cube.colors.insert(Face::DOWN, back);
            cube.colors.insert(Face::FRONT, down);
            cube.colors.insert(Face::BACK, up);
            cube.colors.insert(Face::LEFT, left);
            cube.colors.insert(Face::RIGHT, right);
        } else if movement.direction == Direction::CounterClockwise {
            cube.colors.insert(Face::UP, back);
            cube.colors.insert(Face::DOWN, front);
            cube.colors.insert(Face::FRONT, up);
            cube.colors.insert(Face::BACK, down);
            cube.colors.insert(Face::LEFT, left);
            cube.colors.insert(Face::RIGHT, right);
        }
    } else if movement.axis == RotateAxis::Y {
        if movement.direction == Direction::Clockwise {
            cube.colors.insert(Face::UP, up);
            cube.colors.insert(Face::DOWN, down);
            cube.colors.insert(Face::FRONT, right);
            cube.colors.insert(Face::BACK, left);
            cube.colors.insert(Face::LEFT, front);
            cube.colors.insert(Face::RIGHT, back);
        } else if movement.direction == Direction::CounterClockwise {
            cube.colors.insert(Face::UP, up);
            cube.colors.insert(Face::DOWN, down);
            cube.colors.insert(Face::FRONT, left);
            cube.colors.insert(Face::BACK, right);
            cube.colors.insert(Face::LEFT, back);
            cube.colors.insert(Face::RIGHT, front);
        }
    } else if movement.axis == RotateAxis::Z {
        if movement.direction == Direction::Clockwise {
            cube.colors.insert(Face::UP, left);
            cube.colors.insert(Face::DOWN, right);
            cube.colors.insert(Face::FRONT, front);
            cube.colors.insert(Face::BACK, back);
            cube.colors.insert(Face::LEFT, down);
            cube.colors.insert(Face::RIGHT, up);
        } else if movement.direction == Direction::CounterClockwise {
            cube.colors.insert(Face::UP, right);
            cube.colors.insert(Face::DOWN, left);
            cube.colors.insert(Face::FRONT, front);
            cube.colors.insert(Face::BACK, back);
            cube.colors.insert(Face::LEFT, up);
            cube.colors.insert(Face::RIGHT, down);
        }
    }
}

pub fn gen_random_movements(steps: u32) -> VecDeque<Movement> {
    let mut rng = rand::thread_rng();
    let axis = vec![RotateAxis::X, RotateAxis::Y, RotateAxis::Z];
    let dirs = vec![Direction::Clockwise, Direction::CounterClockwise];
    let mut ret: VecDeque<Movement> = VecDeque::new();
    let mut cnt = 0;
    while cnt < steps {
        let next = Movement {
            axis: axis[rng.gen_range(0..3)],
            layer: rng.gen_range(0..3),
            direction: dirs[rng.gen_range(0..2)]
        };
        ret.push_back(next);
        cnt += 1;
    }
    ret
}