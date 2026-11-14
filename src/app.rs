mod panels;
use eframe::egui;

use crate::color::{self, Cmyk, Rgb};

pub(crate) struct Camera {
    pub yaw: f32,
    pub pitch: f32,
}

pub struct ColorApp {
    rgb: Rgb,
    cmyk: Cmyk,
    camera: Camera,
}

impl Default for ColorApp {
    fn default() -> Self {
        let rgb = [242.0 / 255.0, 140.0 / 255.0, 40.0 / 255.0];
        Self { rgb, cmyk: color::rgb_to_cmyk(rgb), camera: Camera { yaw: 0.5, pitch: -0.3 } }
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
