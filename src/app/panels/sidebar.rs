use eframe::egui;

use crate::app::ColorApp;
use crate::color;
const RGB_LABELS: [&str; 3] = ["R", "G", "B"];
const CMYK_LABELS: [&str; 4] = ["C", "M", "Y", "K"];

impl ColorApp {
    pub(super) fn show_sidebar(&mut self, ui: &mut egui::Ui) {
        egui::Panel::left("picker").resizable(false).exact_size(300.0).show(ui, |ui| {
            ui.spacing_mut().slider_width = 180.0;
            ui.add_space(8.0);

            let (rect, _) = ui.allocate_exact_size(egui::vec2(ui.available_width(), 90.0), egui::Sense::hover());
            ui.painter().rect_filled(rect, 6.0, color::to_color32(self.rgb));
            ui.monospace(color::hex(self.rgb));
            ui.add_space(12.0);

            // rgb
            ui.heading("RGB");
            let mut rgb = self.rgb;
            let mut rgb_changed: bool = false;

            for (i, label) in RGB_LABELS.into_iter().enumerate() {
                let mut shown = (rgb[i] * 255.0).round();
                let response = ui.add(channel_slider(&mut shown, 255.0, "").text(label));
                if response.changed() {
                    rgb[i] = shown / 255.0;
                    rgb_changed = true;
                }
            }
            if rgb_changed {
                self.set_rgb(rgb);
            }
            // cmyk
            ui.add_space(12.0);
            ui.heading("CMYK");
            let mut cmyk_changed: bool = false;
            let mut cmyk = self.cmyk;

            for (i, label) in CMYK_LABELS.into_iter().enumerate() {
                let mut shown = (cmyk[i] * 100.0).round();
                let response = ui.add(channel_slider(&mut shown, 100.0, "%").text(label));
                if response.changed() {
                    cmyk[i] = shown / 100.0;
                    cmyk_changed = true;
                }
            }

            if cmyk_changed {
                self.set_cmyk(cmyk);
            }

            ui.add_space(12.0);
            ui.separator();
            let [r, g, b] = self.rgb.map(|x| (x * 255.0).round() as u8);
            let [c, m, y, k] = self.cmyk.map(|x| x * 100.0);

            ui.monospace(format!("RGB  ({r}, {g}, {b})"));
            ui.monospace(format!("CMYK ({c:.1}%, {m:.1}%, {y:.1}%, {k:.1}%)"));
        });
    }
}
fn channel_slider<'a>(value: &'a mut f32, max: f32, suffix: &'static str) -> egui::Slider<'a> {
    egui::Slider::new(value, 0.0..=max).integer().suffix(suffix)
}
