use crate::{State, Screen};
use eframe::{egui::{self, RichText}, epaint::{Color32, Vec2}};

pub fn render(state: &mut State, ui: &mut egui::Ui) {
    if let Screen::Join { joinid } = &mut state.screen {
        ui.text_edit_singleline(joinid);
        for i in '0'..='9' {
            let keypad_button = egui::widgets::Button::new(RichText::new(i.to_string())
                .size(10.0)
                .color(Color32::WHITE))
                .min_size(Vec2 { x: 75.0, y: 25.0 });
            if ui.add(keypad_button).clicked() {
                joinid.push(i);
            }
        }
        let clear_button = egui::widgets::Button::new(RichText::new("clear")
            .size(10.0)
            .color(Color32::WHITE))
            .min_size(Vec2 { x: 75.0, y: 25.0 });
        let back_button = egui::widgets::Button::new(RichText::new("back")
            .size(10.0)
            .color(Color32::WHITE))
            .min_size(Vec2 { x: 75.0, y: 25.0 });
        let join_button = egui::widgets::Button::new(RichText::new("join")
            .size(10.0)
            .color(Color32::WHITE))
            .min_size(Vec2 { x: 75.0, y: 25.0 });
        if ui.add(clear_button).clicked() {
            joinid.clear();
        }
        if ui.add(back_button).clicked() {
            joinid.pop();
        }
        if ui.add(join_button).clicked() {
            // stuff happens
        }
    }
    if ui.button("Enter").clicked() {
        state.screen = Screen::Game;
    }
}
