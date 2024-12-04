use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts};
use crate::{actions::{gen_random_movements, ActionStatus}, settings::Settings};

pub fn update_ui(
    mut contexts: EguiContexts,
    mut settings: ResMut<Settings>,
    mut status: ResMut<ActionStatus>,
) {
    egui::Window::new("Settings")
        .vscroll(true)
        .show(contexts.ctx_mut(), |ui| {
            ui.add(egui::Slider::new(&mut settings.view_rotation_speed, 1.0..=10.0).text("view rotation speed"));
            ui.add(egui::Slider::new(&mut settings.layer_rotation_speed, 1.0..=50.0).text("layer rotation speed"));
            // ui.add(egui::TextEdit::)
            if ui.add(egui::Button::new("scramble")).clicked() {
                if status.action_queue.is_empty() {
                    status.action_queue.append(&mut gen_random_movements(10));
                }
            }
        });
}