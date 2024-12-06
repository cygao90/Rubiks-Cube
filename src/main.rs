use std::collections::VecDeque;

use actions::ActionStatus;
use bevy::prelude::*;
use bevy_egui::EguiPlugin;
use bevy_mod_picking::DefaultPickingPlugins;
use cube::CubeInfo;

mod camera;
mod cube;
mod actions;
mod ui;
mod settings;
mod solver;

#[macro_use(lazy_static)]
extern crate lazy_static;

fn main() {
    App::new()
    .add_plugins((
        DefaultPlugins,
        DefaultPickingPlugins,
        EguiPlugin,
    ))
    .insert_resource(CubeInfo::default())
    .insert_resource(ActionStatus { 
        angle_to_process: 0.0,
        action_queue: VecDeque::new(),
        cur_action: None,
        drag_start: None,
        drag_end: None,
        selected_entity: None,
        computing_solution: false,
    })
    .insert_resource(settings::Settings::default())
    .add_systems(
        Startup, 
        (
            camera::setup_camera, 
            cube::setup_cube,
        )
    )
    .add_systems(
        Update, 
        (
            camera::handle_view,
            actions::frame_handler,
            ui::update_ui,
            ui::handle_solve_complete,
        )
    )
    .run();
}


