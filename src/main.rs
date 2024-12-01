use bevy::prelude::*;

mod camera;
mod cube;
mod actions;

fn main() {
    App::new()
    .add_plugins(DefaultPlugins)
    .add_systems(Startup, (camera::setup_camera, cube::setup_cube))
    .add_systems(Update, actions::rotate)
    .run();
}

