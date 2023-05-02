use std::sync::mpsc::Receiver;

use eframe::egui;
use scorched_earth_core::Board;
use scorched_earth_network::Connection;
mod screens;

#[no_mangle]
#[cfg(android)]
pub fn android_main(app: AndroidApp) -> Result<(), eframe::Error> {
    use android_activity::AndroidApp;
    let mut options: eframe::NativeOptions = Default::default();

    use winit::platform::android::EventLoopBuilderExtAndroid;

    options.event_loop_builder = Some(Box::new(move |builder| {
        builder.with_android_app(app);
    }));

    eframe::run_native(
        "Scorched Earth",
        options,
        Box::new(|_| Box::new(State::default())),
    )
}

pub enum Screen {
    Title { joinid: String },
    Rules,
    Host,
    Join(Receiver<Result<(Connection, Board), scorched_earth_network::Error>>),
    Game { conn: Connection, board: Board },
}

impl Default for Screen {
    fn default() -> Self {
        Screen::Title {
            joinid: String::new(),
        }
    }
}

#[derive(Default)]
pub struct State {
    screen: Screen,
}

impl eframe::App for State {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| match self.screen {
            Screen::Title { .. } => {
                screens::title::render(self, ui);
            }
            Screen::Rules => {
                screens::rules::render(&mut self.screen, ui);
            }
            Screen::Host => {
                screens::host::render(&mut self.screen, ui);
            }
            Screen::Join(_) => {
                screens::join::render(&mut self.screen, ui);
            }
            Screen::Game { .. } => {
                screens::game::render(&mut self.screen, ui);
            }
        });
    }
}
