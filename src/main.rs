pub mod algo;
pub mod main_ui;

use main_ui::QApp;

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        //initial_window_size: Some(egui::vec2(800.0, 600.0)),
        ..Default::default()
    };
    eframe::run_native(
        "8 Vezir problemi",
        options,
        Box::new(|_cc| Ok(Box::<QApp>::default())),
    )
}
