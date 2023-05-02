use crate::Screen;
use eframe::egui;
use tracing::info;

pub fn render(screen: &mut Screen, ui: &mut egui::Ui) {
    ui.heading("joining room...");
    if let Screen::Join(rx) = screen {
        if let Ok(res) = rx.try_recv() {
            match res {
                Ok((conn, board)) => {
                    info!("conn succeeded");
                    *screen = Screen::Game { conn, board };
                }
                Err(e) => {
                    info!("conn failed: {}", e);
                    *screen = Default::default();
                }
            }
        }
    }
}
