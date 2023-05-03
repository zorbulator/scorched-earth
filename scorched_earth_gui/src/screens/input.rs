use scorched_earth_network::Connection;
use std::sync::mpsc::channel;
use crate::Screen;
use eframe::{egui::{self, RichText}, epaint::{Color32, Vec2}};

pub fn render(screen: &mut Screen, ui: &mut egui::Ui) {
        if let Screen::Input { joinid } = screen {
            ui.text_edit_singleline(joinid);
            ui.with_layout(egui::Layout::with_main_wrap(egui::Layout::top_down(egui::Align::LEFT), true), |ui| {});
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
                let (tx, rx) = channel();
                let joinid2 = joinid.clone();
                tx.send(Connection::conn("169.231.11.248:8080", joinid2.as_bytes()))
                    .unwrap();
                *screen = Screen::Join(rx);
            }
        }
    
}
