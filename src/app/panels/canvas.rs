use crate::app::{Camera, ColorApp};
use eframe::egui::{self, Pos2, Rect, Stroke};

const CORNERS: [[f32; 3]; 8] = [
    [0.0, 0.0, 0.0], // black
    [1.0, 0.0, 0.0], // red
    [0.0, 1.0, 0.0], // green
    [1.0, 1.0, 0.0], // yellow
    [0.0, 0.0, 1.0], // blue
    [1.0, 0.0, 1.0], // magenta
    [0.0, 1.0, 1.0], // cyan
    [1.0, 1.0, 1.0], // white
];

// distance from the camera to the cube
const DIST: f32 = 4.0;

#[rustfmt::skip]
const EDGES: [(usize, usize); 12] =
    [(0, 1), (2, 3), (4, 5), (6, 7),  // R 
     (0, 2), (1, 3), (4, 6), (5, 7),  // G
     (0, 4), (1, 5), (2, 6), (3, 7)]; // B

impl ColorApp {
    pub(super) fn show_canvas(&mut self, ui: &mut egui::Ui) {
        egui::CentralPanel::default().show(ui, |ui| {
            let (rect, _response) = ui.allocate_exact_size(ui.available_size(), egui::Sense::click_and_drag());
            let painter = ui.painter_at(rect);
            let stroke = Stroke::new(1.5, ui.visuals().text_color());

            for (a, b) in EDGES {
                let pa = project(rotate(CORNERS[a], &self.camera), rect);
                let pb = project(rotate(CORNERS[b], &self.camera), rect);
                painter.line_segment([pa, pb], stroke);
            }
        });
    }
}

// https://msl.cs.uiuc.edu/planning/node102.html, fig. 3.40 and 3.41
type Mat3 = [[f32; 3]; 3];

#[rustfmt::skip]
fn ry(angle: f32) -> Mat3 {
    let (s, c) = angle.sin_cos();
    [
        [ c , 0.0,  s ], 
        [0.0, 1.0, 0.0], 
        [-s , 0.0,  c ]
    ]
}
#[rustfmt::skip]
fn rx(angle: f32) -> Mat3 {
    let (s, c) = angle.sin_cos();
    [
        [1.0, 0.0, 0.0], 
        [0.0,  c , -s ], 
        [0.0,  s ,  c ]
    ]
}

// literally just matrix multiplication
fn mul(m: Mat3, v: [f32; 3]) -> [f32; 3] {
    m.map(|row| row[0] * v[0] + row[1] * v[1] + row[2] * v[2])
}

fn rotate(p: [f32; 3], cam: &Camera) -> [f32; 3] {
    // centering the cube around (0, 0, 0)
    let [r, g, b] = p.map(|x| x - 0.5);
    let xyz = [r, b, g]; // swapping around so blue is the vertical axis
    let yawed = mul(ry(cam.yaw), xyz);
    mul(rx(cam.pitch), yawed)
}

fn project(p: [f32; 3], rect: Rect) -> Pos2 {
    let [x, y, z] = p;
    let shrink_factor = DIST / (DIST - z);
    let scale = rect.width().min(rect.height()) * 0.35;

    rect.center() + egui::vec2(x, -y) * scale * shrink_factor
}
