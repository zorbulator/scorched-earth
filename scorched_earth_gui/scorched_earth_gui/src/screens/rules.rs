use crate::Screen;
use eframe::egui;

pub fn render(screen: &mut Screen, ui: &mut egui::Ui) {
    ui.heading("Rules");
    if ui.button("back").clicked() {
        *screen = Default::default();
    }
}
