use crate::app::{Camera, ColorApp};
use crate::color;
use crate::math::{mul, rx, ry};
use eframe::egui::{self, Pos2, Rect, Stroke};

/// distance from the camera to the cube
const DIST: f32 = 4.0;
/// corners of the rgb cube, defined as rgb channels
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
/// faces of the cube, 4 corners each
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
            let [r, g, b] = self.rgb;

            let [slab, back, front] = [
                // ideally this would be 3 boxes with one variable channel each (ie. going from [0,0,0] to [1,1,b], [1,g,1], [r,1,1])
                // unfortunately those overlap, so im going for something like this:
                // slab (bottom one) - [0,0,0] -> [1,1,b]
                ([0.0, 0.0, 0.0], [1.0, 1.0, b]),
                // back - [0,0,b] -> [1,g,1]
                ([0.0, 0.0, b], [1.0, g, 1.0]),
                // front - [0,g,b] -> [r,1,1]
                ([0.0, g, b], [r, 1.0, 1.0]),
            ];
            // backface culling is only done per-box, so the order of rendering the boxes can result in see-through faces sometimes
            // calculating which one we're facing at this time
            let slab_corners = box_corners(slab.0, slab.1);
            // if true, camera is above the cube cross-section (looking down at 'slab')
            let above = faces_camera(FACES[1].map(|i| project(rotate(slab_corners[i], &self.camera), rect)));

            let back_corners = box_corners(back.0, back.1);
            // if true, camera is on the side looking at 'back'
            let behind = faces_camera(FACES[5].map(|i| project(rotate(back_corners[i], &self.camera), rect)));
            let upper = if behind { [back, front] } else { [front, back] };
            let order = if above { [slab, upper[0], upper[1]] } else { [upper[0], upper[1], slab] };

            for (min, max) in order {
                painter.extend(box_shapes(min, max, &self.camera, rect, stroke));
            }
        });
    }
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
/// backface culling pretty much
fn faces_camera(pts: [Pos2; 4]) -> bool {
    let u = pts[1] - pts[0];
    let v = pts[2] - pts[0];

    let cross = u.x * v.y - u.y * v.x;
    cross > 1.0
}

fn box_shapes(min: [f32; 3], max: [f32; 3], cam: &Camera, rect: Rect, stroke: Stroke) -> Vec<egui::Shape> {
    let corners = box_corners(min, max);
    let mut outlines = Vec::new();
    let mut mesh = egui::Mesh::default();

    for face in FACES {
        let pts = face.map(|i| project(rotate(corners[i], cam), rect));
        if !faces_camera(pts) {
            continue;
        }
        let base = mesh.vertices.len() as u32;
        // add vertices
        for (p, i) in pts.iter().zip(face) {
            mesh.colored_vertex(*p, color::to_color32(corners[i]));
        }
        // split into two triangles with base as the start point
        mesh.add_triangle(base, base + 1, base + 2);
        mesh.add_triangle(base, base + 2, base + 3);

        // add outlines only for visible faces
        for i in 0..4 {
            outlines.push(egui::Shape::line_segment([pts[i], pts[(i + 1) % 4]], stroke))
        }
    }
    let mut shapes = vec![egui::Shape::mesh(mesh)];
    shapes.extend(outlines);
    shapes
}

fn box_corners(min: [f32; 3], max: [f32; 3]) -> [[f32; 3]; 8] {
    CORNERS.map(|t| [0, 1, 2].map(|axis| min[axis] + t[axis] * (max[axis] - min[axis])))
}
