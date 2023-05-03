use std::{sync::mpsc::{channel, Receiver}, thread};
use scorched_earth_network::Connection;

use crate::Screen;
use eframe::{egui::{self, RichText, FontId}, epaint::{Color32, Vec2}};

const ADDR: &str = "169.231.11.248:8080";

pub fn render(screen: &mut Screen, ui: &mut egui::Ui) {
    if let Screen::Input { joinid } = screen {
        ui.vertical_centered(|ui| {
            ui.add_space(30.0);
            ui.heading(RichText::new("Input Join Code")
                .color(Color32::WHITE)
                .font(FontId::proportional(25.0))
                .size(30.0)
            );
            ui.add_space(10.0);
            ui.add(egui::widgets::text_edit::TextEdit::singleline(joinid)
               .min_size(Vec2 { x: 300.0, y: 30.0 })
               .font(FontId::proportional(25.0))
            );

            ui.add_space(20.0);
            let mut grid = egui::Grid::new("keypad");
            egui::Grid::new("keypad").show(ui, |ui| {
                for _i in '0'..='9' {
                    let keypad_button = egui::widgets::Button::new(RichText::new(_i.to_string())
                                                                   .size(10.0)
                                                                   .color(Color32::WHITE))
                        .min_size(Vec2 { x: 100.0, y: 50.0 });
                    if ui.add(keypad_button).clicked() {
                        joinid.push(_i);
                    }
                    if _i == '2' {
                        ui.end_row();
                    }
                    if _i == '5' {
                        ui.end_row();
                    }
                    if _i == '8' {
                        ui.end_row();
                    }
                }
                // only set if join is clicked
                let mut join_rx: Option<Receiver<_>> = None;

                let clear_button = egui::widgets::Button::new(RichText::new("clear")
                                                              .size(10.0)
                                                              .color(Color32::WHITE))
                    .min_size(Vec2 { x: 100.0, y: 50.0 });

                let back_button = egui::widgets::Button::new(RichText::new("back")
                                                             .size(10.0)
                                                             .color(Color32::WHITE))
                    .min_size(Vec2 { x: 100.0, y: 50.0 });

                let join_button = egui::widgets::Button::new(RichText::new("join")
                                                             .size(10.0)
                                                             .color(Color32::WHITE))
                    .min_size(Vec2 { x: 100.0, y: 50.0 });

                if ui.add(clear_button).clicked() {
                    joinid.clear();
                }
                if ui.add(back_button).clicked() {
                    joinid.pop();
                }
                ui.end_row();
                if ui.add(join_button).clicked() {
                    let (tx, rx) = channel();
                    let joinid2 = joinid.clone();
                    thread::spawn(move || {
                        tx.send(Connection::conn(ADDR, joinid2.as_bytes())).unwrap();
                    });
                    // *screen = Screen::Join(rx);
                    join_rx = Some(rx);
                }
            });
        });
    }
}
