use eframe::{egui::{CentralPanel, self, menu, Layout}, emath::Align};

use crate::Flasher;

impl Flasher {
    pub fn render_main_windows(&mut self, ctx: &egui::Context) {
        CentralPanel::default().show(ctx, |ui|{

            menu::bar(ui, |ui|{
                egui::ComboBox::from_label("Select Your Soldering Iron.")
                    .selected_text(format!("{}", self.iron))
                    .show_ui(ui, |ui| {
                        ui.selectable_value(&mut self.iron, "Pinecil".to_string(), "Pinecil");
                        ui.selectable_value(&mut self.iron, "TS100".to_string(), "Ts100");
                        ui.selectable_value(&mut self.iron, "TS80".to_string(), "Ts80");
                    }
                );
                ui.with_layout(Layout::right_to_left(Align::TOP), |ui|{
                    egui::ComboBox::from_label("Specify Release Version")
                        .selected_text(format!("{}", self.version))
                        .show_ui(ui, |ui| {
                            if !self.versions_checked {
                                Flasher::grab_vers(self);
                                self.versions_checked = true;
                            }
                            for i in &self.vers {
                                ui.selectable_value(&mut self.version, i.clone(), i);
                            }
                        }
                    );
                });
            });
            
            menu::bar(ui, |ui|{
                 egui::ComboBox::from_label("Select Your Language.")
                    .selected_text(format!("{}", self.lang))
                    .show_ui(ui, |ui| {
                        for i in &self.langs {
                            ui.selectable_value(&mut self.lang, i.clone(), i);
                        }
                    }
                );               
            });

            ui.vertical_centered(|ui|{
                if ui.button("Flash!").clicked() {
                    Flasher::download(self);
                    Flasher::flash(self);
                };
            })
        });
    }
}