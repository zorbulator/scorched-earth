use std::{sync::mpsc::channel, thread};

use rand::{thread_rng, Rng, distributions::Uniform};
use scorched_earth_core::Board;
use scorched_earth_network::Connection;

use crate::{Screen, State};
use eframe::{egui::{self, RichText, FontId}, epaint::{Color32, Vec2}};

const ADDR: &str = "169.231.11.248:8080";

pub fn render(state: &mut State, ui: &mut egui::Ui) {
    let host_button = egui::widgets::Button::new(RichText::new("Host Game")
        .size(30.0)
        .color(Color32::WHITE))
        .min_size(Vec2 { x: 300.0, y: 50.0 });
    let join_button = egui::widgets::Button::new(RichText::new("Join Game")
        .size(30.0)
        .color(Color32::WHITE))
        .min_size(Vec2 { x: 300.0, y: 50.0 });
    let rules_button = egui::widgets::Button::new(RichText::new("Rules")
        .size(30.0)
        .color(Color32::WHITE))
        .min_size(Vec2 { x: 300.0, y: 50.0 });
    
    let svg_image = egui_extras::RetainedImage::from_svg_bytes_with_size(
        "flame-colored.svg",
        include_bytes!("../assets/flame-colored.svg"),
        egui_extras::image::FitTo::Original,
        )
    .unwrap();
    ui.vertical_centered(|ui| {
        ui.add_space(30.0);
        ui.heading(RichText::new("Scorched Earth")
            .color(Color32::WHITE)
            .font(FontId::proportional(40.0))
            .size(50.0)
        );

        ui.add_space(10.0);

        svg_image.show_size(ui, Vec2 { x: 300.0, y: 300.0 });

        ui.add_space(50.0);

        if ui.add(host_button).clicked() {
            let mut board = Board::default();
            let mut rng = thread_rng();
            board.turn = if rng.gen_bool(0.5) { 1 } else { 0 };
            let mut secret = [0u8; 32];
            for b in &mut secret {
                *b = rng.sample(Uniform::new_inclusive(b'0', b'9'));
            }
            let secret_string = String::from_utf8_lossy(&secret);
            let (tx, rx) = channel();
            let board2 = board.clone();
            thread::spawn(move || {
                tx.send(Connection::host(ADDR, &secret, &board2));
            });
            state.screen = Screen::Host { joinid: secret_string.to_string(), board, rx };
        }

        ui.add_space(30.0);

        if ui.add(join_button).clicked() {
            state.screen = Screen::Input { joinid: String::new() };
        }

        ui.add_space(30.0);

        if ui.add(rules_button).clicked() {
            state.screen = Screen::Rules;
        }
    });
}
