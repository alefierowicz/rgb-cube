mod color_box;
mod view;

use crate::app::ColorApp;
use color_box::{ColorBox, G_MAX, TOP};
use eframe::egui::{self, Stroke};
use view::View;

impl ColorApp {
    pub(super) fn show_canvas(&mut self, ui: &mut egui::Ui) {
        egui::CentralPanel::default().show(ui, |ui| {
            let (rect, response) = ui.allocate_exact_size(ui.available_size(), egui::Sense::click_and_drag());
            let delta = response.drag_delta();
            self.camera.yaw += delta.x * 0.004;
            self.camera.pitch = (self.camera.pitch + delta.y * 0.004).clamp(-1.5, 1.5);

            let painter = ui.painter_at(rect);
            let stroke = Stroke::new(1.5, ui.visuals().text_color());
            let [r, g, b] = self.rgb;
            let view = View { camera: &self.camera, rect };

            let [slab, back, front] = [
                // ideally this would be 3 boxes with one variable channel each (ie. going from [0,0,0] to [1,1,b], [1,g,1], [r,1,1])
                // unfortunately those overlap, so im going for something like this:
                // slab (bottom one) - [0,0,0] -> [1,1,b]
                ColorBox { min: [0.0, 0.0, 0.0], max: [1.0, 1.0, b] },
                // back - [0,0,b] -> [1,g,1]
                ColorBox { min: [0.0, 0.0, b], max: [1.0, g, 1.0] },
                // front - [0,g,b] -> [r,1,1]
                ColorBox { min: [0.0, g, b], max: [r, 1.0, 1.0] },
            ];
            // backface culling is only done per-box, so the order of rendering the boxes can result in see-through faces sometimes
            // calculating which one we're facing at this time
            // if true, camera is above the cube cross-section (looking down at 'slab')
            let above = slab.faces_camera(TOP, &view);

            // if true, camera is on the side looking at 'back'
            let behind = back.faces_camera(G_MAX, &view);
            let upper = if behind { [back, front] } else { [front, back] };
            let order = if above { [slab, upper[0], upper[1]] } else { [upper[0], upper[1], slab] };

            for color_box in order {
                painter.extend(color_box.shapes(&view, stroke));
            }
        });
    }
}
