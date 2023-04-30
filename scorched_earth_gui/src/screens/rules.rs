use crate::{State, Screen};
use eframe::egui;

pub fn render(state: &mut State, ui: &mut egui::Ui) {
    ui.heading("Rules");
    ui.label("* Alternating each turn, players input a command to move their avatar along the grid.");
    ui.label("* Every move leaves behind a scorched tile that neither player can touch.");
    ui.label("* A player loses if they are captured by the other player or surrounded by scorched areas.");
    if ui.button("back").clicked() {
        state.screen = Screen::Title;
    }
}
