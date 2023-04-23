use eframe::egui;
mod screens;

#[no_mangle]
#[cfg(android)]
fn android_main(app: AndroidApp) -> Result<(), eframe::Error> {
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

enum Screen {
    Title,
    Rules,
    Host,
    Join,
    Game,
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
                screens::rules::render(self, ui);
            }
            Screen::Host => {
                screens::host::render(self, ui);
            }
            Screen::Join => {
                screens::join::render(self, ui);
            }
            Screen::Game => {
                screens::game::render(self, ui);
            }
        });
    }
}
