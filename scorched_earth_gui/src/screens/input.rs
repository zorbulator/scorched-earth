use std::{sync::mpsc::{channel, Receiver}, thread};
use scorched_earth_network::Connection;

use crate::Screen;
use eframe::{egui::{self, RichText, FontId}, epaint::{Color32, Vec2}};

const ADDR: &str = "169.231.11.248:8080";

pub fn render(screen: &mut Screen, ui: &mut egui::Ui) {
    // only set if join is clicked
    let mut join_rx: Option<Receiver<_>> = None;

    if let Screen::Input { joinid } = screen {
        ui.vertical_centered(|ui| {
            ui.add_space(30.0);
            ui.heading(RichText::new("Input Join Code")
                .color(Color32::WHITE)
                .font(FontId::proportional(25.0))
                .size(30.0)
            );
            ui.add_space(50.0);
            ui.add(egui::widgets::text_edit::TextEdit::singleline(joinid)
               .min_size(Vec2 { x: 300.0, y: 30.0 })
               .font(FontId::proportional(25.0))
            );

            ui.add_space(250.0);
        });

        let clear_button = egui::widgets::Button::new(RichText::new("clear")
            .size(30.0)
            .color(Color32::WHITE))
            .min_size(Vec2 { x: 100.0, y: 50.0 });

        let back_button = egui::widgets::Button::new(RichText::new("back")
            .size(30.0)
            .color(Color32::WHITE))
            .min_size(Vec2 { x: 100.0, y: 50.0 });

        let join_button = egui::widgets::Button::new(RichText::new("join")
            .size(30.0)
            .color(Color32::WHITE))
            .min_size(Vec2 { x: 100.0, y: 50.0 });

        egui::Grid::new("keypad")
            .show(ui, |ui| {
            ui.add_space(40.0);
            let num_array: [char; 9] = ['7', '8', '9', '4', '5', '6', '1', '2', '3'];
            for _i in num_array{
                let keypad_button = egui::widgets::Button::new(RichText::new(_i.to_string())
                    .size(30.0)
                    .color(Color32::WHITE))
                    .min_size(Vec2 { x: 100.0, y: 50.0 });
                if ui.add(keypad_button).clicked() {
                    joinid.push(_i);
                }
                if _i == '9' {
                    ui.end_row();
                    ui.add_space(40.0);
                }
                if _i == '6' {
                    ui.end_row();
                    ui.add_space(40.0);
                }
                if _i == '3' {
                    ui.end_row();
                    ui.add_space(40.0);
                }
            }

            if ui.add(clear_button).clicked() {
                joinid.clear();
            }

            let zero_button = egui::widgets::Button::new(RichText::new('0')
                .size(30.0)
                .color(Color32::WHITE))
                .min_size(Vec2 { x: 100.0, y: 50.0 });
            if ui.add(zero_button).clicked() {
                joinid.push('0');
            }

            if ui.add(back_button).clicked() {
                joinid.pop();
            }
            ui.end_row();
        });

        ui.vertical_centered(|ui| {
            if ui.add(join_button).clicked() {
                let (tx, rx) = channel();
                let joinid2 = joinid.clone();
                thread::spawn(move || {
                    tx.send(Connection::conn(ADDR, joinid2.as_bytes())).unwrap();
                });
                join_rx = Some(rx);
            }
        });
    }

    if let Some(rx) = join_rx {
        *screen = Screen::Join(rx);
    }
}
