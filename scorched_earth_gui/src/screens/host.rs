use crate::{State, Screen};
use eframe::egui;

pub fn render(state: &mut State, ui: &mut egui::Ui) {
    ui.heading("Host");
    if ui.button("continue").clicked() {
        state.screen = Screen::Game;
    }
}
