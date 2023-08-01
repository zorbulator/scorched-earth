use std::sync::{Arc, Mutex};

use crate::{Screen, back_button};
use eframe::{
    egui::{self, RichText},
    epaint::{Color32, FontId},
};

pub fn render(screen: &mut Screen, ui: &mut egui::Ui) {
    back_button(ui, screen);

    ui.add_space(50.0);

    if let Screen::Host { joinid, board, rx } = screen {
        ui.vertical_centered(|ui| {
            ui.heading(
                RichText::new("Hosting with game id:")
                    .color(Color32::WHITE)
                    .font(FontId::proportional(50.0)),
            );
            ui.add_space(100.0);
            ui.heading(
                RichText::new(format!("{}", joinid))
                    .color(Color32::WHITE)
                    .font(FontId::proportional(150.0))
                    .size(100.0),
            );
        });
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
