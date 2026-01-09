#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use eframe::egui;

mod main_window;

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([600.0, 400.0]),
        ..Default::default()
    };
    eframe::run_native(
        "self introduce",
        options,
        Box::new(|_cc| Ok(Box::<main_window::MainWindow>::default())),
    )
}
