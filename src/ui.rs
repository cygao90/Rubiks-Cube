use bevy::{prelude::*, tasks::{self, futures_lite::{future, FutureExt}, Task}};
use bevy_egui::{egui, EguiContexts};
use crate::{actions::{gen_random_movements, ActionStatus}, cube::{Cube, CubeInfo, Movement, Rotator}, settings::Settings};
use crate::solver::*;
use bevy::tasks::AsyncComputeTaskPool;

#[derive(Component)]
pub struct ComputeTask(Option<Task<Vec<Movement>>>);


pub fn update_ui(
    mut contexts: EguiContexts,
    mut settings: ResMut<Settings>,
    mut status: ResMut<ActionStatus>,
    mut commands: Commands,
    cube_info: Res<CubeInfo>,
    cube_query: Query<&Cube>,
    rotator_entity: Query<Entity, With<Rotator>>
) {
    egui::Window::new("Settings")
        .vscroll(true)
        .show(contexts.ctx_mut(), |ui| {
            ui.add(egui::Slider::new(&mut settings.view_rotation_speed, 1.0..=10.0).text("view rotation speed"));
            ui.add(egui::Slider::new(&mut settings.layer_rotation_speed, 1.0..=50.0).text("layer rotation speed"));
            // ui.add(egui::TextEdit::)
            if ui.add(egui::Button::new("scramble")).clicked() {
                if status.action_queue.is_empty() && !status.computing_solution {
                    status.action_queue.append(&mut gen_random_movements(10));
                }
            }

            if ui.add(egui::Button::new("solve")).clicked() {
                status.computing_solution = true;

                let task_pool = AsyncComputeTaskPool::get();
                let cubes: Vec<Cube> = cube_info.cubes.iter().filter_map(|c| cube_query.get(*c).ok().cloned()).collect();
                let task = task_pool.spawn(async move {
                    solve(cubes)
                });

                commands.entity(rotator_entity.single()).insert(ComputeTask(Some(task)));
            }
        });
}

pub fn handle_solve_complete(
    mut task_query: Query<&mut ComputeTask>,
    mut status: ResMut<ActionStatus>,
    mut commands: Commands,
    rotator_entity: Query<Entity, With<Rotator>>
) {
    for mut task_ in &mut task_query {
        if let Some(task) = task_.0.as_mut() {
            if let Some(result) = tasks::block_on(future::poll_once(task)) {
                for m in result {
                    status.action_queue.push_back(m);
                }
                status.computing_solution = false;
                task_.0 = None;
                commands.entity(rotator_entity.single()).remove::<ComputeTask>();            
            }
        }
    }
}