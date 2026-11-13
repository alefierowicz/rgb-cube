mod app;
mod theme;

fn main() -> eframe::Result {
    eframe::run_native(
        "RGB cube",
        eframe::NativeOptions::default(),
        Box::new(|cc| {
            theme::apply(&cc.egui_ctx);
            Ok(Box::new(app::ColorApp::default()))
        }),
    )
}
