use std::{
    sync::{mpsc::Receiver, Arc, Mutex},
    time::Duration,
};

#[cfg(target_os = "android")]
use android_activity::{AndroidApp, WindowManagerFlags};
use eframe::{egui::{self, RichText}, epaint::{Color32, Vec2}};
use scorched_earth_core::{Board, Move, PlayerColor};
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
        Box::new(|cc| Box::new(State::new(cc))),
    )
}

pub fn back_button(ui: &mut egui::Ui, screen: &mut Screen) {
    #[cfg(target_os="android")]
    ui.add_space(75.0);

    let back_button = egui::Button::new(RichText::new("back").size(20.0).color(Color32::WHITE))
        .min_size(Vec2 { x: 100.0, y: 50.0 });

    if ui.add(back_button).clicked() {
        *screen = Default::default();
    }
}

pub fn convert_color(p: PlayerColor) -> Color32 {
    match p {
        PlayerColor::Blue => Color32::BLUE,
        PlayerColor::Cyan => Color32::LIGHT_BLUE,
        PlayerColor::Green => Color32::from_rgb(143, 216, 97),
        PlayerColor::Yellow => Color32::from_rgb(246, 167, 55),
        PlayerColor::Magenta => Color32::DARK_BLUE,
    }
}

pub enum Screen {
    Title,
    Rules,
    Host {
        joinid: String,
        board: Board,
        rx: Receiver<Result<Connection, scorched_earth_network::Error>>,
    },
    Input {
        joinid: String,
    },
    Join(Receiver<Result<(Connection, Board), scorched_earth_network::Error>>),
    Game {
        conn: Arc<Mutex<Connection>>,
        board: Board,
        preview_move: Option<Move>,
        rx: Option<Receiver<Result<MoveMessage, scorched_earth_network::Error>>>,
        conn_player: usize,
    },
    Error(String),
    End {
        won: bool,
        color: PlayerColor,
    },
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

impl State {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        let mut style: egui::Style = (*cc.egui_ctx.style()).clone();
        style.spacing.item_spacing = egui::vec2(10.0, 10.0);
        cc.egui_ctx.set_style(style);
        Self::default()
    }
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
            Screen::Input { .. } => screens::input::render(&mut self.screen, ui),
            Screen::Join(_) => {
                screens::join::render(&mut self.screen, ui);
            }
            Screen::Game { .. } => {
                screens::game::render(&mut self.screen, ui);
            }
            Screen::Error(_) => {
                screens::error::render(&mut self.screen, ui);
            }
            Screen::End { .. } => {
                screens::end::render(&mut self.screen, ui);
            }
        });
        ctx.request_repaint_after(Duration::from_millis(100));
    }
}
