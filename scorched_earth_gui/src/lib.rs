use std::{sync::{mpsc::Receiver, Mutex, Arc}, time::Duration};

#[cfg(target_os = "android")]
use android_activity::{AndroidApp, WindowManagerFlags};
use eframe::egui;
use scorched_earth_core::{Board, Move};
use scorched_earth_network::{Connection, MoveMessage};
mod screens;

#[no_mangle]
#[cfg(target_os = "android")]
pub fn android_main(app: AndroidApp) -> Result<(), eframe::Error> {
    let mut options: eframe::NativeOptions = Default::default();

    use winit::platform::android::EventLoopBuilderExtAndroid;

    app.set_window_flags(
        WindowManagerFlags::FORCE_NOT_FULLSCREEN,
        WindowManagerFlags::NOT_FOCUSABLE | WindowManagerFlags::FULLSCREEN,
    );

    let app2 = app.clone();
    options.event_loop_builder = Some(Box::new(move |builder| {
        builder.with_android_app(app2);
    }));

    eframe::run_native(
        "Scorched Earth",
        options,
        Box::new(|_| Box::new(State::default())),
    )
}

pub enum Screen {
    Title,
    Rules,
    Host {
        joinid: String,
        board: Board,
        rx: Receiver<Result<Connection, scorched_earth_network::Error>>,
    },
    Input { joinid: String },
    Join(Receiver<Result<(Connection, Board), scorched_earth_network::Error>>),
    Game {
        conn: Arc<Mutex<Connection>>,
        board: Board,
        preview_move: Option<Move>,
        rx: Option<Receiver<Result<MoveMessage, scorched_earth_network::Error>>>,
        conn_player: usize,
    },
    Error(String),
}

impl Default for Screen {
    fn default() -> Self {
        Screen::Title
    }
}

#[derive(Default)]
pub struct State {
    screen: Screen,
}

impl eframe::App for State {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| match self.screen {
            Screen::Title => {
                screens::title::render(self, ui);
            }
            Screen::Rules => {
                screens::rules::render(&mut self.screen, ui);
            }
            Screen::Host { .. } => {
                screens::host::render(&mut self.screen, ui);
            }
            Screen::Input { .. } => {
                screens::input::render(&mut self.screen, ui)
            }
            Screen::Join(_) => {
                screens::join::render(&mut self.screen, ui);
            }
            Screen::Game { .. } => {
                screens::game::render(&mut self.screen, ui);
            }
            Screen::Error(_) => {
                screens::error::render(&mut self.screen, ui);
            }
        });
        ctx.request_repaint_after(Duration::from_millis(100));
    }
}
