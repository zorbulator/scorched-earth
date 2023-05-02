use std::{sync::mpsc::channel};
use scorched_earth_network::Connection;
use tracing::{info, error};

use crate::{Screen, State};
use eframe::egui;

pub fn render(state: &mut State, ui: &mut egui::Ui) {
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
        if ui.button("join").clicked() {
            info!("clicked");
            let (tx, rx) = channel();
            let joinid2 = joinid.clone();
            info!("connecting...");
            tx.send(Connection::conn("zorbulator.com:8080", joinid2.as_bytes())).unwrap();
            info!("connected; switching screens");
            state.screen = Screen::Join(rx);
        }
    }
}
