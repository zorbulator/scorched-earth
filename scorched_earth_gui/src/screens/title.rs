use std::{sync::mpsc::{channel, Receiver}, thread};

use rand::{thread_rng, Rng, distributions::Uniform};
use scorched_earth_core::Board;
use scorched_earth_network::Connection;

use crate::{Screen, State};
use eframe::egui;

const ADDR: &str = "169.231.11.248:8080";

pub fn render(state: &mut State, ui: &mut egui::Ui) {
    ui.label("-");
    ui.label("-");
    ui.label("-");
    ui.heading("Scorched Earth");
    if ui.button("rules").clicked() {
        state.screen = Screen::Rules;
    }
    if ui.button("host").clicked() {
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

    ui.separator();

    // only set if join is clicked
    let mut join_rx: Option<Receiver<_>> = None;

    if let Screen::Title { joinid } = &mut state.screen {
        ui.text_edit_singleline(joinid);
        for i in '0'..='9' {
            if ui.button(i.to_string()).clicked() {
                joinid.push(i);
            }
        }
        if ui.button("clear").clicked() {
            joinid.clear();
        }
        if ui.button("back").clicked() {
            joinid.pop();
        }
        if ui.button("join").clicked() {
            let (tx, rx) = channel();
            let joinid2 = joinid.clone();
            thread::spawn(move || {
                tx.send(Connection::conn(ADDR, joinid2.as_bytes())).unwrap();
            });
            join_rx = Some(rx);
        }
    }
    if let Some(rx) = join_rx {
        state.screen = Screen::Join(rx);
    }
}
