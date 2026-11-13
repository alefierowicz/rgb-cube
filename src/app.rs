mod panels;

use eframe::egui;

#[derive(Default)]
pub struct ColorApp {}

impl eframe::App for ColorApp {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        self.show_main(ui);
    }
}
