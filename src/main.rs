use std::collections::VecDeque;

use actions::ActionStatus;
use bevy::prelude::*;
use cube::CubeInfo;

mod camera;
mod cube;
mod actions;

fn main() {
    App::new()
    .add_plugins(DefaultPlugins)
    .insert_resource(CubeInfo::default())
    .insert_resource(ActionStatus { 
        angle_to_process: 0.0,
        action_queue: VecDeque::new(),
        cur_action: None
    })
    .add_systems(Startup, (camera::setup_camera, cube::setup_cube))
    .add_systems(Update, actions::rotate_cube)
    .add_systems(Update, actions::frame_handler)
    .run();
}

