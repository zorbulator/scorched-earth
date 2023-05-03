use crate::{State, Screen};
use eframe::{egui::{self, RichText, FontId}, epaint::{Color32, Vec2}};

pub fn render(state: &mut State, ui: &mut egui::Ui) {
    ui.vertical_centered(|ui| {
        ui.add_space(30.0);
        ui.heading(RichText::new("Rules")
            .color(Color32::WHITE)
            .font(FontId::proportional(40.0))
            .size(50.0)
        );
        ui.label("Scorched Earth is a turn based strategy game. Every turn, each player inputs
                 a command to move their avatar along the battlefield. Tread ")
    });
    // ui.label("* Alternating each turn, players input a command to move their avatar along the grid.");
    // ui.label("* Every move leaves behind a scorched tile that neither player can touch.");
    // ui.label("* A player loses if they are captured by the other player or surrounded by scorched areas.");
    if ui.button("back").clicked() {
        state.screen = Screen::Title;
    }
}
