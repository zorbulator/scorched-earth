use crate::{State, Screen};
use eframe::egui;

pub fn render(state: &mut State, ui: &mut egui::Ui) {
    ui.heading("Scorched Earth");
    if ui.button("rules").clicked() {
        state.screen = Screen::Rules;
    }
    if ui.button("host").clicked() {
        state.screen = Screen::Host;
    }
    if ui.button("join").clicked() {
        state.screen = Screen::Join;
    }
}

