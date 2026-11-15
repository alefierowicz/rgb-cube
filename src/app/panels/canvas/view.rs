use eframe::egui::{self, Pos2, Rect};

use crate::app::Camera;
use crate::math::{Vec3, mul, rx, ry};

/// distance from the camera to the cube
const DIST: f32 = 4.0;

/// everything needed to put a point of the cube on screen
pub(super) struct View<'a> {
    pub camera: &'a Camera,
    pub rect: Rect,
}

impl View<'_> {
    fn rotate(&self, p: Vec3) -> Vec3 {
        // centering the cube around (0, 0, 0)
        let [r, g, b] = p.map(|x| x - 0.5);
        let xyz = [r, b, g]; // swapping around so blue is the vertical axis
        let yawed = mul(ry(self.camera.yaw), xyz);
        mul(rx(self.camera.pitch), yawed)
    }

    /// colour space -> screen: rotate, then perspective projection
    pub fn project(&self, p: Vec3) -> Pos2 {
        let [x, y, z] = self.rotate(p);
        let shrink_factor = DIST / (DIST - z);
        let scale = self.rect.width().min(self.rect.height()) * 0.35;

        self.rect.center() + egui::vec2(x, -y) * scale * shrink_factor
    }
}
