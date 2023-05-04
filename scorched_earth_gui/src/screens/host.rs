use std::sync::{Arc, Mutex};

use crate::Screen;
use eframe::egui;

pub fn render(screen: &mut Screen, ui: &mut egui::Ui) {
    ui.heading("Hosting");
    if let Screen::Host { joinid, board, rx } = screen {
        ui.heading(format!("game id: {}", joinid));
        if let Ok(res) = rx.try_recv() {
            match res {
                Ok(conn) => {
                    let conn_player = conn.player_num;
                    *screen = Screen::Game {
                        conn: Arc::new(Mutex::new(conn)),
                        board: board.clone(),
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
