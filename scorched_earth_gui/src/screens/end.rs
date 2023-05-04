use crate::{Screen, convert_color};

use eframe::{
    egui::{self, Button, RichText, Sense},
    epaint::{Color32, FontId, Rounding, Vec2},
};

pub fn render(screen: &mut Screen, ui: &mut egui::Ui) {
    ui.add_space(75.0);

    let back_button = Button::new(RichText::new("back").size(20.0).color(Color32::WHITE))
        .min_size(Vec2 { x: 100.0, y: 50.0 });

    if ui.add(back_button).clicked() {
        *screen = Default::default();
    }
    if let Screen::End { won, color } = screen {
        //ui.painter().rect_filled(ui.painter().clip_rect(), Rounding::none(), convert_color(*color));
        ui.add_space(100.0);


        ui.vertical_centered(|ui| {
            ui.heading(
                RichText::new(if *won { "You win!" } else { "You lost" })
                    .color(Color32::WHITE)
                    .font(FontId::proportional(50.0))
                    .size(50.0),
            );
            let (rect, _response) = ui.allocate_exact_size(egui::vec2(500.0, 500.0), Sense::focusable_noninteractive());
            ui.painter().rect_filled(rect, Rounding::none(), convert_color(*color));
        });
    }


}
