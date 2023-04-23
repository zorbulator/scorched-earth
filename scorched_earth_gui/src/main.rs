use eframe::egui;
use scorched_earth_gui_lib::State;

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(320.0, 240.0)),
        ..Default::default()
    };
    eframe::run_native("Scorched Earth", options, Box::new(|_| Box::new(State::default())))
}
