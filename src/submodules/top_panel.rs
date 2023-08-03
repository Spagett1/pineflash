use eframe::egui::{self, menu, Button, TopBottomPanel};

use crate::{FlashSavedConfig, Flasher};

impl Flasher {
    pub fn render_header(&mut self, ctx: &egui::Context) {
        TopBottomPanel::top("top_panel").show(ctx, |ui| {
            menu::bar(ui, |ui| {
                ui.vertical_centered(|ui| ui.heading("PineFlash"));
                // ui.with_layout(Layout::align(), |ui| {
                ui.with_layout(egui::Layout::right_to_left(egui::Align::TOP), |ui| {
                    ui.add_space(5.);
                    if ui
                        .add(Button::new(if self.saved_config.dark_mode {
                            " "
                        } else {
                            " "
                        }))
                        .clicked()
                    {
                        if self.saved_config.dark_mode {
                            ctx.set_visuals(egui::Visuals::light());
                        } else {
                            ctx.set_visuals(egui::Visuals::dark());
                        }
                        self.saved_config.dark_mode = !self.saved_config.dark_mode;
                        if let Err(..) = confy::store(
                            "PineFlash",
                            None,
                            FlashSavedConfig {
                                dark_mode: self.saved_config.dark_mode,
                            },
                        ) {
                            self.config
                                .logs
                                .push_str("Pineflash: Failed to save the app state.")
                        }
                    }
                })
                // })
            });
        });
    }
}
