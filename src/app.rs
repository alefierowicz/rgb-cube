mod panels;

use eframe::egui;

use crate::color::{self, Cmyk, Rgb};

pub struct ColorApp {
    rgb: Rgb,
    cmyk: Cmyk,
}

impl Default for ColorApp {
    fn default() -> Self {
        // kolor startowy: akcent z theme.rs (#F28C28)
        let rgb = [242.0 / 255.0, 140.0 / 255.0, 40.0 / 255.0];
        Self { rgb, cmyk: color::rgb_to_cmyk(rgb) }
    }
}

impl ColorApp {
    fn set_rgb(&mut self, rgb: Rgb) {
        self.rgb = rgb;
        self.cmyk = color::rgb_to_cmyk(rgb);
    }
    fn set_cmyk(&mut self, cmyk: Cmyk) {
        self.cmyk = cmyk;
        self.rgb = color::cmyk_to_rgb(cmyk);
    }
}

impl eframe::App for ColorApp {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        self.show_main(ui);
    }
}
