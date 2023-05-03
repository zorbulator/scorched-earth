use scorched_earth_network::Connection;
use std::sync::mpsc::channel;

use crate::{Screen, State};
use eframe::egui;

pub fn render(state: &mut State, ui: &mut egui::Ui) {
    ui.label("-");
    ui.label("-");
    ui.label("-");
    ui.heading("Scorched Earth");
    if ui.button("rules").clicked() {
        state.screen = Screen::Rules;
    }
    if ui.button("host").clicked() {
        state.screen = Screen::Host;
    }

    ui.separator();

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
            tx.send(Connection::conn("169.231.11.248:8080", joinid2.as_bytes()))
                .unwrap();
            state.screen = Screen::Join(rx);
        }
    }
}
