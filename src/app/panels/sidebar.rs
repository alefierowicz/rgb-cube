use eframe::egui;

use crate::app::ColorApp;

impl ColorApp {
    pub(super) fn show_sidebar(&mut self, ui: &mut egui::Ui) {
        egui::Panel::left("info").resizable(false).exact_size(300.0).show(ui, |ui| ui.label("sidebar here"));
    }
}
