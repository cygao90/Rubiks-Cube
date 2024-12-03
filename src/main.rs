use std::collections::VecDeque;

use bevy::color::palettes::css;
use actions::ActionStatus;
use bevy::prelude::*;
use bevy_mod_picking::{highlight::{ Highlight , HighlightKind}, prelude::DefaultHighlightingPlugin, DefaultPickingPlugins};
use cube::CubeInfo;

mod camera;
mod cube;
mod actions;

fn main() {
    App::new()
    .add_plugins((
        DefaultPlugins,
        DefaultPickingPlugins
    ))
    .insert_resource(CubeInfo::default())
    .insert_resource(ActionStatus { 
        angle_to_process: 0.0,
        action_queue: VecDeque::new(),
        cur_action: None,
        drag_start: None,
        drag_end: None,
        selected_entity: None,
    })
    .insert_resource(Settings::default())
    .add_systems(
        Startup, 
        (
            camera::setup_camera, 
            cube::setup_cube,
            actions::gen_random_movements,
        )
    )
    .add_systems(
        Update, 
        (
            camera::handle_view,
            actions::frame_handler,
        )
    )
    .run();
}


#[derive(Resource)]
pub struct Settings {
    pub layers: u32,
    pub color_up: Color,
    pub color_down: Color,
    pub color_left: Color,
    pub color_right: Color,
    pub color_front: Color,
    pub color_back: Color,
    pub color_beleved: Color,
}

impl Default for Settings {
    fn default() -> Self {
        Settings {
            layers: 3,
            color_down: Color::Srgba(css::WHITE),
            color_front: Color::Srgba(css::GREEN),
            color_up:  Color::Srgba(css::YELLOW),
            color_left: Color::Srgba(css::RED),
            color_back: Color::Srgba(css::BLUE),
            color_right: Color::Srgba(css::ORANGE),
            color_beleved: Color::Srgba(css::BLACK)
        }
    }
}

