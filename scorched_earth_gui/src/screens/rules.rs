use crate::Screen;
use eframe::{egui::{self, RichText, FontId}, epaint::{Color32, Vec2}};

pub fn render(screen: &mut Screen, ui: &mut egui::Ui) {
    ui.add_space(75.0);

    let back_button = egui::Button::new(RichText::new("back").size(20.0).color(Color32::WHITE))
        .min_size(Vec2 { x: 100.0, y: 50.0 });

    if ui.add(back_button).clicked() {
        *screen = Default::default();
    }
    ui.vertical_centered(|ui| {
        ui.add_space(20.0);
        ui.heading(RichText::new("Rules")
            .color(Color32::WHITE)
            .font(FontId::proportional(40.0))
            .size(30.0)
        );
        ui.add_space(10.0);
    });
    ui.label(
        egui::RichText::new("🔥Scorched Earth is an original turn based strategy game that simulates scorched earth warfare.")
        .size(17.0)
        .color(Color32::WHITE)
    );
    ui.add_space(20.0);
    ui.label(
        egui::RichText::new("🔥Control your soldier by moving 1 or 2 tiles in the 4 cardinal directions on a square grid.")
        .size(17.0)
        .color(Color32::WHITE)
    );
    ui.add_space(20.0);
    ui.label(
        egui::RichText::new("🔥Tread carefully however - every move leaves behind scorched tiles that neither player can touch!")
        .size(17.0)
        .color(Color32::WHITE)
    );
    ui.add_space(20.0);
    ui.label(
        egui::RichText::new("🔥As the game progresses, there will be fewer safe tiles on the battlefield.")
        .size(17.0)
        .color(Color32::WHITE)
    );
    ui.add_space(20.0);
    ui.label(
        egui::RichText::new("🔥You win if your opponent is surrounded on 4 sides by scorched tiles.")
        .size(17.0)
        .color(Color32::WHITE)
        );
    ui.add_space(20.0);
    ui.label(
        egui::RichText::new("🔥You can also win by capturing your opponent (moving on top of them during your turn).")
        .size(17.0)
        .color(Color32::WHITE)
    );
}
