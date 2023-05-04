use crate::Screen;
use eframe::{egui::{self, RichText}, epaint::{Color32, Vec2}};

pub fn render(screen: &mut Screen, ui: &mut egui::Ui) {
    ui.vertical_centered(|ui| {
        ui.add_space(30.0);
        ui.heading(RichText::new("Error")
            .color(Color32::WHITE)
            .size(50.0)
        );
        ui.add_space(15.0);
        if let Screen::Error(s) = screen {
            ui.heading(format!("details: {}", s));
        }
    });
    ui.add_space(75.0);

    ui.vertical_centered(|ui| {
        let back_button = egui::Button::new(RichText::new("back").size(20.0).color(Color32::WHITE))
            .min_size(Vec2 { x: 100.0, y: 50.0 });
        if ui.add(back_button).clicked() {
            *screen = Screen::Input { joinid: String::new() };
        }
    });

}
