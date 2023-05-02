use crate::{State, Screen};
use eframe::{egui::{self, RichText, FontId}, epaint::{Color32, Vec2}};

pub fn render(state: &mut State, ui: &mut egui::Ui) {

    let host_button = egui::widgets::Button::new(RichText::new("Host Game").color(Color32::WHITE)).min_size(Vec2 { x: 100.0, y: 50.0 });
    let join_button = egui::widgets::Button::new(RichText::new("Join Game").color(Color32::WHITE)).min_size(Vec2 { x: 100.0, y: 50.0 });
    let rules_button = egui::widgets::Button::new(RichText::new("Rules").color(Color32::WHITE)).min_size(Vec2 { x: 100.0, y: 50.0 });
    
    let svg_image = egui_extras::RetainedImage::from_svg_bytes_with_size(
        "fire-zone.svg",
        include_bytes!("../assets/fire-zone.svg"),
        egui_extras::image::FitTo::Original,
        )
    .unwrap();
    ui.vertical_centered(|ui| {
        ui.add_space(20.0);
        ui.heading(RichText::new("Scorched Earth").color(Color32::DARK_RED).font(FontId::proportional(40.0)));
        ui.add_space(10.0);
        svg_image.show_size(ui, Vec2 { x: 300.0, y: 300.0 });
        ui.add_space(30.0);
        if ui.add(host_button).clicked() {
            state.screen = Screen::Host;
        }
        ui.add_space(20.0);
        if ui.add(join_button).clicked() {
            state.screen = Screen::Rules;
        }
        ui.add_space(20.0);
        if ui.add(rules_button).clicked() {
            state.screen = Screen::Join;
        }
    });
}

