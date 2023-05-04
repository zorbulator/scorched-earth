use scorched_earth_network::Connection;
use std::{
    sync::mpsc::{channel, Receiver},
    thread,
};

use crate::Screen;
use eframe::{
    egui::{self, Button, FontId, RichText},
    epaint::{Color32, Vec2},
};

const ADDR: &str = "169.231.11.248:8080";

pub fn render(screen: &mut Screen, ui: &mut egui::Ui) {
    // only set if join is clicked
    let back_button = Button::new(RichText::new("back").size(30.0).color(Color32::WHITE))
        .min_size(Vec2 { x: 150.0, y: 60.0 });

    if ui.add(back_button).clicked() {
        *screen = Default::default();
    }
    let mut join_rx: Option<Receiver<_>> = None;

    if let Screen::Input { joinid } = screen {
        ui.vertical_centered(|ui| {
            ui.add_space(30.0);
            ui.heading(
                RichText::new("Input Join Code")
                    .color(Color32::WHITE)
                    .font(FontId::proportional(25.0))
                    .size(30.0),
            );
            ui.add_space(50.0);
            ui.add(
                egui::widgets::text_edit::TextEdit::singleline(joinid)
                    .min_size(Vec2 { x: 300.0, y: 30.0 })
                    .font(FontId::proportional(25.0)),
            );

            ui.add_space(100.0);
        });

        let clear_button = egui::widgets::Button::new(RichText::new("clear")
            .size(30.0)
            .color(Color32::WHITE));

        let back_button = egui::widgets::Button::new(RichText::new("back")
            .size(30.0)
            .color(Color32::WHITE));

        let join_button = egui::widgets::Button::new(RichText::new("join")
            .size(30.0)
            .color(Color32::WHITE));

        let zero_button = egui::widgets::Button::new(RichText::new('0')
            .size(30.0)
            .color(Color32::WHITE));
        let row_length = ui.available_width() * 0.8;
        ui.vertical_centered(|ui| {
            for start in [7, 4, 1] {
                ui.allocate_ui(Vec2 { x: row_length, y: 50.0 }, |ui| {
                    ui.columns(3, |columns| {
                        for i in 0..3 {
                            columns[i].vertical_centered(|ui| {
                                let b = Button::new(RichText::new(format!("{}", start + i)).size(30.0).color(Color32::WHITE));
                                if ui.add_sized(ui.available_size(), b).clicked() {
                                    joinid.push(char::from_digit((start + i) as u32, 10).unwrap());
                                };
                            });
                        }
                    });
                });
            }
            ui.allocate_ui(Vec2 { x: row_length, y: 50.0 }, |ui| {
                ui.columns(3, |columns| {
                    columns[0].vertical_centered(|ui| {
                        if ui.add_sized(ui.available_size(), clear_button).clicked() {
                            joinid.clear();
                        }
                    });
                    columns[1].vertical_centered(|ui| {
                        if ui.add_sized(ui.available_size(), zero_button).clicked() {
                            joinid.push('0');
                        }
                    });
                    columns[2].vertical_centered(|ui| {
                        if ui.add_sized(ui.available_size(), back_button).clicked() {
                            joinid.pop();
                        }
                    });
                });
            });
            ui.allocate_ui(Vec2 { x: row_length, y: 50.0 }, |ui| {
                ui.columns(3, |columns| {
                    columns[1].vertical_centered(|ui| {
                        if ui.add_sized(ui.available_size(), join_button).clicked() {
                            let (tx, rx) = channel();
                            let joinid2 = joinid.clone();
                            thread::spawn(move || {
                                tx.send(Connection::conn(ADDR, joinid2.as_bytes())).unwrap();
                            });
                            join_rx = Some(rx);
                        }
                    });
                });
            });
        });
    }

    if let Some(rx) = join_rx {
        *screen = Screen::Join(rx);
    }
}
