use crate::{State, Screen};
use eframe::egui;

pub fn render(state: &mut State, ui: &mut egui::Ui) {
    ui.heading("Rules");
    if ui.button("back").clicked() {
        state.screen = Screen::Title;
    }
}
