use crate::Screen;
use eframe::egui;

pub fn render(screen: &mut Screen, ui: &mut egui::Ui) {
    ui.heading("Host");
    if ui.button("continue").clicked() {
    }
}
