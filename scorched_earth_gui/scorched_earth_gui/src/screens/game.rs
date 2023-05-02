use crate::Screen;
use eframe::egui;
use scorched_earth_core::{Direction, Move};
use scorched_earth_network::MoveMessage;

pub fn render(screen: &mut Screen, ui: &mut egui::Ui) {
    ui.heading("Game");
    if let Screen::Game { conn, board } = screen {
        if ui.button("left").clicked() {
            let i = board.turn;
            let m = Move {dir: Direction::Left, len: 1};
            board.make_move(
                board.turn,
                m
            );
            conn.send_move(MoveMessage {
                new_board: board.clone(),
                new_move: m,
                player: i,
            });
        }
        if ui.button("right").clicked() {
            let i = board.turn;
            let m = Move {dir: Direction::Right, len: 1};
            board.make_move(
                board.turn,
                m
            );
            conn.send_move(MoveMessage {
                new_board: board.clone(),
                new_move: m,
                player: i,
            });
        }
    }
    if ui.button("back").clicked() {
        *screen = Default::default();
    }
}
