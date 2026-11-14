use crate::app::{Camera, ColorApp};
use crate::color;
use eframe::egui::{self, Pos2, Rect, Stroke};

// distance from the camera to the cube
const DIST: f32 = 4.0;
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
#[rustfmt::skip]
const FACES: [[usize; 4]; 6] = [
    [0, 2, 3, 1], [4, 5, 7, 6], // B = 0, B = 1
    [0, 4, 6, 2], [1, 3, 7, 5], // R = 0, R = 1
    [0, 1, 5, 4], [2, 6, 7, 3], // G = 0, G = 1
];

impl ColorApp {
    pub(super) fn show_canvas(&mut self, ui: &mut egui::Ui) {
        egui::CentralPanel::default().show(ui, |ui| {
            let (rect, response) = ui.allocate_exact_size(ui.available_size(), egui::Sense::click_and_drag());
            let delta = response.drag_delta();
            self.camera.yaw += delta.x * 0.004;
            self.camera.pitch = (self.camera.pitch + delta.y * 0.004).clamp(-1.5, 1.5);

            let painter = ui.painter_at(rect);
            let stroke = Stroke::new(1.5, ui.visuals().text_color());

            // filling in color & drawing wireframe
            let mut outlines = Vec::new();
            let mut mesh = egui::Mesh::default();
            for face in FACES {
                let pts = face.map(|i| project(rotate(CORNERS[i], &self.camera), rect));

                // backface culling
                let u = pts[1] - pts[0];
                let v = pts[2] - pts[0];

                let cross = u.x * v.y - u.y * v.x;
                if cross <= 0.0 {
                    continue;
                }

                let base = mesh.vertices.len() as u32;

                // add vertices
                for (p, i) in pts.iter().zip(face) {
                    mesh.colored_vertex(*p, color::to_color32(CORNERS[i]));
                }

                // split into two triangles with base as the start point
                mesh.add_triangle(base, base + 1, base + 2);
                mesh.add_triangle(base, base + 2, base + 3);

                // add outlines only for visible faces
                for i in 0..4 {
                    outlines.push(egui::Shape::line_segment([pts[i], pts[(i + 1) % 4]], stroke))
                }
            }

            painter.add(egui::Shape::mesh(mesh));
            painter.extend(outlines);
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
