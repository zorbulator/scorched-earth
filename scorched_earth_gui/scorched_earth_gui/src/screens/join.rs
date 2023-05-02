use crate::Screen;
use eframe::egui;

pub fn render(screen: &mut Screen, ui: &mut egui::Ui) {
    ui.heading("joining room...");
    if let Screen::Join(rx) = screen {
        if let Ok(res) = rx.try_recv() {
            match res {
                Ok((conn, board)) => {
                    *screen = Screen::Game { conn, board };
                }
                Err(e) => {
                    *screen = Screen::Error(e.to_string());
                }
            }
        }
    }
}
