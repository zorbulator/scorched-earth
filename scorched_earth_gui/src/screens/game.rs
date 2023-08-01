use std::{sync::mpsc::channel, thread};

use crate::{convert_color, Screen, back_button};
use eframe::{
    egui::{self, RichText},
    epaint::{Color32, Rect, Rounding, Vec2},
};
use scorched_earth_core::{Board, Direction, Move, PlayerColor, TileContents, Vector, BOARD_SIZE};
use scorched_earth_network::MoveMessage;

fn draw_board(ui: &mut egui::Ui, board: &Board, preview_move: &Option<Move>, i: usize) {
    //let desired_size = ui.available_width() * 0.6 * egui::vec2(1.0, 1.0);
    let width = (ui.available_width()) as usize / 11 * 11;
    //let desired_size = egui::vec2(11f32 * 30f32, 11f32 * 30f32);
    let desired_size = egui::vec2(width as f32, width as f32);

    let (rect, _response) =
        ui.allocate_exact_size(desired_size, egui::Sense::focusable_noninteractive());

    if ui.is_rect_visible(rect) {
        let w = rect.width() / BOARD_SIZE as f32;
        for i in 0..BOARD_SIZE {
            for j in 0..BOARD_SIZE {
                if let Some(tile) = board.tile_contents_at(Vector {
                    x: i as isize,
                    y: j as isize,
                }) {
                    let color = match tile {
                        TileContents::Empty => Color32::BLACK,
                        TileContents::Scorched => Color32::RED,
                        TileContents::Player(p) => convert_color(p),
                    };

                    let corner = rect.left_top() + egui::vec2(i as f32 * w, j as f32 * w);
                    ui.painter().rect_filled(
                        Rect {
                            min: corner,
                            max: corner + egui::vec2(w, w),
                        },
                        Rounding::none(),
                        color,
                    );
                }
            }
        }

        if let Some(potential_move) = preview_move {
            let target_position = board.players[i].pos + potential_move.to_vector();
            if (0..BOARD_SIZE as isize).contains(&target_position.x)
                && (0..BOARD_SIZE as isize).contains(&target_position.y)
            {
                let color = if board.is_move_valid(i, *potential_move) {
                    Color32::WHITE
                } else {
                    Color32::GRAY
                };
                for tile in potential_move.tiles_along_path() {
                    let pos = board.players[i].pos + tile;
                    let corner = rect.left_top() + egui::vec2(pos.x as f32 * w, pos.y as f32 * w);
                    ui.painter().rect_filled(
                        Rect {
                            min: corner,
                            max: corner + egui::vec2(w, w),
                        },
                        Rounding::none(),
                        color,
                    );
                }
            }
        }
    }
}

pub fn render(screen: &mut Screen, ui: &mut egui::Ui) {
    back_button(ui, screen);
    ui.add_space(15.0);
    let mut error_message: Option<String> = None;
    let mut won: Option<(bool, PlayerColor)> = None;
    if let Screen::Game {
        conn,
        board,
        preview_move,
        rx,
        conn_player,
    } = screen
    {
        let i = board.turn;
        let conn_player = *conn_player;
        ui.vertical_centered(|ui| {
            draw_board(ui, board, preview_move, i);
            ui.add_space(15.0);
        });

        // it's the online player's turn
        if conn_player == i {
            *preview_move = None;
            if let None = rx {
                let (t, r) = channel();
                let conn2 = conn.clone();
                thread::spawn(move || {
                    t.send(conn2.lock().unwrap().recv_move()).unwrap();
                });
                *rx = Some(r);
            }

            if let Some(r) = rx {
                if let Ok(res) = r.try_recv() {
                    match res {
                        Ok(m) => {
                            let res = board.make_move(i, m.new_move);
                            if board != &m.new_board {
                                error_message =
                                    Some(String::from("Other player's board doesn't match!"));
                            }
                            if let Some(color) = res.winner {
                                let lost = color == board.players[conn_player].color;
                                won = Some((!lost, color));
                            }
                        }
                        Err(e) => {
                            error_message = Some(e.to_string());
                        }
                    }
                    *rx = None;
                }
            }
        } else {
            let mut input: Option<Direction> = None;

            let left_button =
                egui::widgets::Button::new(RichText::new("left").size(20.0).color(Color32::WHITE));

            let right_button =
                egui::widgets::Button::new(RichText::new("right").size(20.0).color(Color32::WHITE));

            let down_button =
                egui::widgets::Button::new(RichText::new("down").size(20.0).color(Color32::WHITE));

            let up_button =
                egui::widgets::Button::new(RichText::new("up").size(20.0).color(Color32::WHITE));

            let done_button =
                egui::widgets::Button::new(RichText::new("done").size(20.0).color(Color32::WHITE));

            let row_length = ui.available_width() * 0.95;
            ui.vertical_centered(|ui| {
                ui.allocate_ui(
                    Vec2 {
                        x: row_length,
                        y: 50.0,
                    },
                    |ui| {
                        ui.columns(3, |columns| {
                            columns[1].vertical_centered(|ui| {
                                if ui.add_sized(ui.available_size(), up_button).clicked() {
                                    input = Some(Direction::Up);
                                }
                            })
                        });
                    },
                );

                ui.allocate_ui(
                    Vec2 {
                        x: row_length,
                        y: 50.0,
                    },
                    |ui| {
                        ui.columns(3, |columns| {
                            columns[0].vertical_centered(|ui| {
                                if ui.add_sized(ui.available_size(), left_button).clicked() {
                                    input = Some(Direction::Left);
                                }
                            });
                            columns[1].vertical_centered(|ui| {
                                if ui.add_sized(ui.available_size(), done_button).clicked() {
                                    if let Some(m) = preview_move {
                                        if board.is_move_valid(i, *m) {
                                            let res = board.make_move(i, *m);
                                            if let Some(color) = res.winner {
                                                let lost =
                                                    color == board.players[conn_player].color;
                                                won = Some((!lost, color));
                                            }
                                            match conn.lock().unwrap().send_move(MoveMessage {
                                                new_board: board.clone(),
                                                new_move: *m,
                                                player: i,
                                            }) {
                                                Err(e) => error_message = Some(e.to_string()),
                                                _ => {}
                                            }
                                            *preview_move = None;
                                        }
                                    }
                                }
                            });
                            columns[2].vertical_centered(|ui| {
                                if ui.add_sized(ui.available_size(), right_button).clicked() {
                                    input = Some(Direction::Right);
                                }
                            });
                        });
                    },
                );
                ui.allocate_ui(
                    Vec2 {
                        x: row_length,
                        y: 50.0,
                    },
                    |ui| {
                        ui.columns(3, |columns| {
                            columns[1].vertical_centered(|ui| {
                                if ui.add_sized(ui.available_size(), down_button).clicked() {
                                    input = Some(Direction::Down);
                                }
                            })
                        });
                    },
                );
            });
            if let Some(dir) = input {
                match preview_move {
                    None => {
                        *preview_move = Some(Move { dir, len: 1 });
                    }
                    Some(old) => {
                        if dir == old.dir && old.len == 1 {
                            old.len = 2;
                        } else if dir == old.dir.opposite() && old.len == 2 {
                            old.len = 1;
                        } else {
                            old.len = 1;
                            old.dir = dir;
                        }
                    }
                }
            }
        }
    }
    if let Some(e) = error_message {
        *screen = Screen::Error(e);
    }

    if let Some((won, color)) = won {
        *screen = Screen::End { won, color };
    }
}
