use bevy::prelude::*;
use bevy_async_task::{AsyncTaskRunner, AsyncTaskStatus};
use bevy_egui::{egui, EguiContexts};
use crate::{actions::{gen_random_movements, ActionStatus}, cube::{Cube, CubeInfo, Movement}, settings::Settings};
use crate::solver::*;

pub fn update_ui(
    mut contexts: EguiContexts,
    mut settings: ResMut<Settings>,
    mut status: ResMut<ActionStatus>,
    cube_info: Res<CubeInfo>,
    cube_query: Query<&Cube>,
    mut task_runner: AsyncTaskRunner<Vec<Movement>>
) {
    egui::Window::new("Settings")
        .vscroll(true)
        .show(contexts.ctx_mut(), |ui| {
            ui.add(egui::Slider::new(&mut settings.view_rotation_speed, 1.0..=10.0).text("view rotation speed"));
            ui.add(egui::Slider::new(&mut settings.layer_rotation_speed, 1.0..=50.0).text("layer rotation speed"));
            if ui.add(egui::Button::new("scramble")).clicked() {
                if status.action_queue.is_empty() && !status.computing_solution {
                    status.action_queue.append(&mut gen_random_movements(25));
                }
            }

            if ui.add(egui::Button::new("solve")).clicked() {
                if task_runner.is_idle() {
                    status.computing_solution = true;

                    let cubes: Vec<Cube> = cube_info.cubes.iter().filter_map(|c| cube_query.get(*c).ok().cloned()).collect();
                    task_runner.start(solve(cubes));
                }
            }
        });

    match task_runner.poll() {
        AsyncTaskStatus::Finished(res) => {
            for m in res {
                status.action_queue.push_back(m);
            }
            status.computing_solution = false;
        },

        _ => ()
    }
}

