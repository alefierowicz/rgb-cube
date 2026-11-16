use eframe::egui::{self, Pos2, Stroke};

use super::view::View;
use crate::color;
use crate::math::Vec3;

/// corners of the rgb cube, defined as rgb channels
const CORNERS: [Vec3; 8] = [
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

/// index into FACES: the B = max face
pub(super) const TOP: usize = 1;
/// index into FACES: the G = max face
pub(super) const G_MAX: usize = 5;

/// an axis-aligned box inside the rgb cube, from `min` to `max` (rgb channels, 0..1)
#[derive(Clone, Copy)]
pub(super) struct ColorBox {
    pub min: Vec3,
    pub max: Vec3,
}

impl ColorBox {
    /// the 8 corners of this box, in the same order as CORNERS
    fn corners(&self) -> [Vec3; 8] {
        CORNERS.map(|t| [0, 1, 2].map(|axis| self.min[axis] + t[axis] * (self.max[axis] - self.min[axis])))
    }

    /// screen positions of the 4 corners of FACES[face]
    pub fn face(&self, face: usize, view: &View) -> [Pos2; 4] {
        let corners = self.corners();
        FACES[face].map(|i| view.project(corners[i]))
    }

    /// true if FACES[face] of this box is turned towards the camera
    pub fn faces_camera(&self, face: usize, view: &View) -> bool {
        faces_camera(self.face(face, view))
    }

    /// everything needed to paint this box: its mesh first, then its outlines on top
    pub fn shapes(&self, view: &View, stroke: Stroke) -> Vec<egui::Shape> {
        let corners = self.corners();
        let mut outlines = Vec::new();
        let mut mesh = egui::Mesh::default();

        for face in FACES {
            let pts = face.map(|i| view.project(corners[i]));
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
}

/// backface culling pretty much
fn faces_camera(pts: [Pos2; 4]) -> bool {
    let u = pts[1] - pts[0];
    let v = pts[2] - pts[0];

    let cross = u.x * v.y - u.y * v.x;
    cross > 1.0
}
