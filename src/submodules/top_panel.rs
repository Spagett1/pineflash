use eframe::{egui::{TopBottomPanel, self, Button, Layout, menu}, emath::Align};

use crate::Flasher;

impl Flasher {
    pub fn render_header(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        TopBottomPanel::top("top_panel").show(ctx, |ui| {
            menu::bar(ui, |ui|{
                ui.vertical_centered(|ui|{
                    ui.heading("IronOs Flasher")
                });
                ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                    ui.add_space(5.);
                    if ui.add(Button::new("❌")).clicked() {
                        frame.close();
                    }
                })
            });
        });
    }
}