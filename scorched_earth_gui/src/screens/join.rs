use std::sync::{Arc, Mutex};

use crate::Screen;
use eframe::egui;

pub fn render(screen: &mut Screen, ui: &mut egui::Ui) {
    ui.heading("joining room...");
    if ui.button("cancel (may cause problems)").clicked() {
        *screen = Default::default();
    }

    if let Screen::Join(rx) = screen {
        if let Ok(res) = rx.try_recv() {
            match res {
                Ok((conn, board)) => {
                    let conn_player = conn.player_num;
                    *screen = Screen::Game {
                        conn: Arc::new(Mutex::new(conn)),
                        board,
                        preview_move: None,
                        rx: None,
                        conn_player,
                    };
                }
                Err(e) => {
                    *screen = Screen::Error(e.to_string());
                }
            }
        }
    }
}
