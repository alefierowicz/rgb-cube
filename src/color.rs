use eframe::egui::Color32;

pub type Rgb = [f32; 3];
pub type Cmyk = [f32; 4];

/*
Black = minimum(1-Red,1-Green,1-Blue)
Cyan = (1-Red-Black)/(1-Black)
Magenta = (1-Green-Black)/(1-Black)
Yellow = (1-Blue-Black)/(1-Black)
*/

pub fn rgb_to_cmyk([r, g, b]: Rgb) -> Cmyk {
    let black = f32::min(1.0 - r, f32::min(1.0 - g, 1.0 - b));
    if black == 1.0 {
        return [0.0, 0.0, 0.0, 1.0];
    }
    let cyan: f32 = ((1.0 - r) - black) / (1.0 - black);
    let magenta: f32 = ((1.0 - g) - black) / (1.0 - black);
    let yellow: f32 = ((1.0 - b) - black) / (1.0 - black);
    [cyan, magenta, yellow, black]
}

/*
Red = 1-minimum(1,Cyan*(1-Black)+Black)
Green = 1-minimum(1,Magenta*(1-Black)+Black)
Blue = 1-minimum(1,Yellow*(1-Black)+Black)
 */

pub fn cmyk_to_rgb([c, m, y, k]: Cmyk) -> Rgb {
    let channel = |v: f32| -> f32 { 1.0 - f32::min(1.0, v * (1.0 - k) + k) };
    [channel(c), channel(m), channel(y)]
}

pub fn to_color32(rgb: Rgb) -> Color32 {
    let [r, g, b] = rgb.map(|x| (x * 255.0).round() as u8);
    Color32::from_rgb(r, g, b)
}
pub fn hex(rgb: Rgb) -> String {
    let c32 = to_color32(rgb);
    format!("#{:02X}{:02X}{:02X}", c32.r(), c32.g(), c32.b())
}
