mod canvas;
mod sidebar;

use eframe::egui;

use super::ColorApp;

impl ColorApp {
    pub(super) fn show_main(&mut self, ui: &mut egui::Ui) {
        self.show_sidebar(ui);
        self.show_canvas(ui);
    }
}
