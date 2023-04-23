use crate::{State, Screen};
use eframe::egui;

pub fn render(state: &mut State, ui: &mut egui::Ui) {
    ui.heading("Joining");
    if ui.button("continue").clicked() {
        state.screen = Screen::Game;
    }
}
