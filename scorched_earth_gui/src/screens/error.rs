use crate::Screen;
use eframe::egui;

pub fn render(screen: &mut Screen, ui: &mut egui::Ui) {
    ui.heading("Error");
    if let Screen::Error(s) = screen {
        ui.heading(format!("details: {}", s));
    }
    if ui.button("back").clicked() {
        *screen = Default::default();
    }
}
