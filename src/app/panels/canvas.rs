use eframe::egui;

use crate::app::ColorApp;

impl ColorApp {
    pub(super) fn show_canvas(&mut self, ui: &mut egui::Ui) {
        egui::CentralPanel::default().show(ui, |ui| {
            ui.label("cube will go here somewhere");
            ui.allocate_exact_size(ui.available_size(), egui::Sense::click());
        });
    }
}
