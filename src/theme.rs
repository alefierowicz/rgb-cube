use eframe::egui::{self, Color32, CornerRadius, FontData, FontDefinitions, FontFamily, Stroke, Theme};
use std::sync::Arc;

// palette: Melange Redux Dark (https://github.com/rtud/melange-redux)
pub const BG: Color32 = Color32::from_rgb(0x29, 0x25, 0x22); // #292522 editor background
pub const SURFACE: Color32 = Color32::from_rgb(0x34, 0x30, 0x2C); // #34302C sidebar / widgets
pub const SURFACE_HI: Color32 = Color32::from_rgb(0x40, 0x3A, 0x36); // #403A36 hover / selection
pub const DEEP: Color32 = Color32::from_rgb(0x21, 0x1E, 0x1B); // #211E1B slider rails, text fields (one step darker than BG)
pub const TEXT: Color32 = Color32::from_rgb(0xEC, 0xE1, 0xD7); // #ECE1D7 foreground
pub const BRIGHT: Color32 = Color32::from_rgb(0xF4, 0xF0, 0xED); // #F4F0ED text on hover / active
pub const ACCENT: Color32 = Color32::from_rgb(0xE4, 0x9B, 0x5D); // #E49B5D orange (focus border)
pub const ERROR: Color32 = Color32::from_rgb(0xD4, 0x77, 0x66); // #D47766 red

pub const BOLD: &str = "bold";

pub fn apply(ctx: &egui::Context) {
    load_fonts(ctx);

    ctx.set_theme(Theme::Dark);
    ctx.style_mut_of(Theme::Dark, |style| {
        let v = &mut style.visuals;
        v.panel_fill = BG;
        v.window_fill = SURFACE;

        v.extreme_bg_color = DEEP;
        v.faint_bg_color = SURFACE;
        v.hyperlink_color = ACCENT;
        v.error_fg_color = ERROR;
        v.selection.bg_fill = ACCENT;
        v.selection.stroke = Stroke::new(1.0, BG);

        let w = &mut v.widgets;
        w.noninteractive.bg_fill = BG;
        w.noninteractive.bg_stroke = Stroke::new(1.0, SURFACE_HI);
        w.noninteractive.fg_stroke = Stroke::new(1.0, TEXT);

        w.inactive.weak_bg_fill = SURFACE;
        w.inactive.bg_fill = SURFACE_HI;
        w.inactive.fg_stroke = Stroke::new(1.0, TEXT);

        w.hovered.weak_bg_fill = SURFACE_HI;
        w.hovered.bg_fill = SURFACE_HI;
        w.hovered.bg_stroke = Stroke::new(1.0, ACCENT);
        w.hovered.fg_stroke = Stroke::new(1.5, BRIGHT);

        w.active.weak_bg_fill = ACCENT;
        w.active.bg_fill = ACCENT;
        w.active.bg_stroke = Stroke::new(1.0, ACCENT);
        w.active.fg_stroke = Stroke::new(2.0, BRIGHT);

        for state in [&mut w.inactive, &mut w.hovered, &mut w.active] {
            state.corner_radius = CornerRadius::same(6);
        }
    })
}

fn load_fonts(ctx: &egui::Context) {
    let mut fonts = FontDefinitions::default();
    fonts
        .font_data
        .insert(BOLD.to_owned(), Arc::new(FontData::from_static(include_bytes!("../assets/MartianMono-SemiBold.ttf"))));
    fonts.families.insert(FontFamily::Name(BOLD.into()), vec![BOLD.to_owned()]);
    ctx.set_fonts(fonts);
}
