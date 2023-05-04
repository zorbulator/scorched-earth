use std::sync::{Arc, Mutex};

use crate::Screen;
use eframe::{
    egui::{self, RichText, Button},
    epaint::{Color32, FontId, Vec2},
};

pub fn render(screen: &mut Screen, ui: &mut egui::Ui) {
    ui.add_space(100.0);

    let back_button = Button::new(RichText::new("back").size(30.0).color(Color32::WHITE))
        .min_size(Vec2 { x: 150.0, y: 60.0 });

    if ui.add(back_button).clicked() {
        *screen = Default::default();
    }

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
                    .size(150.0),
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
