#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use eframe::egui;
use scorched_earth_gui::State;

#[cfg(not(target_arch = "wasm32"))]
fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(411.0, 731.0)),
        ..Default::default()
    };
    eframe::run_native(
        "Scorched Earth",
        options,
        Box::new(|cc| Box::new(State::new(cc))),
    )
}

#[cfg(target_arch = "wasm32")]
fn main() {
    console_error_panic_hook::set_once();
    tracing_wasm::set_as_global_default();
    let web_options = eframe::WebOptions::default();
    wasm_bindgen_futures::spawn_local(async {
        eframe::start_web(
            "canvas",
            web_options,
            Box::new(|_| Box::new(State::default())),
        )
        .await
        .expect("failed to start eframe");
    })
}
