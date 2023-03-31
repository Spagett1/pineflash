use std::time::Duration;

use eframe::egui::{CentralPanel, self, menu, RichText, ScrollArea};

use crate::Flasher;

impl Flasher {
    pub fn render_main_windows(&mut self, ctx: &egui::Context) {
        CentralPanel::default().show(ctx, |ui|{

            ui.label("Select your Soldering Iron");
            menu::bar(ui, |ui|{
                egui::ComboBox::from_label(" ")
                    .selected_text(self.config.iron.to_string())
                    .show_ui(ui, |ui| {
                        ui.selectable_value(&mut self.config.iron, "Pinecil V1".to_string(), "Pinecil V1");
                        ui.selectable_value(&mut self.config.iron, "Pinecil V2".to_string(), "Pinecil V2 W.I.P.");
                    }
                );
                if self.config.iron == "Pinecil V1"  {
                    self.config.int_name = "Pinecil".to_string();
                } else if self.config.iron == "Pinecil V2" {
                    self.config.int_name = "Pinecilv2".to_string();
                }
            });

            ui.label("Specify Version");
            menu::bar(ui, |ui|{
                ui.add_enabled_ui(self.config.versions_checked, |ui|{
                    egui::ComboBox::from_label("")
                        .selected_text(self.config.version.to_string())
                        .show_ui(ui, |ui| {
                            if self.config.versions_checked {
                                for i in &self.config.vers {
                                    ui.selectable_value(&mut self.config.version, i.clone(), i);
                                }
                                ui.selectable_value(&mut self.config.version, "Custom".to_string(), "Custom");
                            }

                        }
                    );           
                });
                if ui.button(RichText::new("📁").size(17.)).clicked() {
                    if let Some(path) = rfd::FileDialog::new().pick_file() {
                        if !path.display().to_string().contains("dfu") && self.config.int_name == "Pinecil" || 
                            !path.display().to_string().contains("bin") && self.config.int_name == "Pinecilv2" 
                        {
                            self.toasts.dismiss_all_toasts();
                            self.toasts.error("File has the incorrect format").set_duration(Some(Duration::from_secs(4))).set_closable(false);
                            self.config.logs.push_str("Incorrect filetype selected.\n");
                        } else {
                            self.config.picked_path = Some(path.display().to_string());
                            self.config.version = "Custom".to_string();
                            self.toasts.dismiss_all_toasts();
                            self.toasts.info("Custom file selected").set_duration(Some(Duration::from_secs(4))).set_closable(false);
                            self.config.logs.push_str("Custom file selected.\n");
                            self.config.ready_to_flash = true;
                        }
                    }
                }
            });


            ui.label("Select Your Language.");
            ui.add_enabled_ui({
                if self.config.version != "Select".to_string() 
                    && !self.config.download_metadata 
                {true} else {false}
            }, |ui|{
                menu::bar(ui, |ui|{
                    egui::ComboBox::from_label("  ")
                        .selected_text(self.config.lang.to_string())
                        .show_ui(ui, |ui| {
                            for i in 0..self.config.code_names.len() {
                                let code_name = &self.config.code_names[i];
                                let fancy_name = &self.config.fancy_names[i];
                                ui.selectable_value(&mut self.config.lang, code_name.to_string(), fancy_name);
                            }
                        }
                    );               
                });
            });


            // ui.vertical_centered(|ui|{

            if self.config.version != "Custom".to_string() && self.config.version != "Select".to_string() {
                self.config.ready_to_flash = true
            } else if self.config.version == "Custom".to_string() && self.config.picked_path == None {
                self.config.ready_to_flash = false
            }

            if !self.config.ready_to_flash {
                ui.add_enabled(false, egui::Button::new("Update!")).on_disabled_hover_text("Select a firmware version or custom file");
            } else {
                if ui.button("Update!").clicked() {
                    if self.config.version != "Custom".to_string() {
                        self.config.download = true;
                    } else {
                        Flasher::flash(self)
                    }
                };
            }
            ui.collapsing("Logs", |ui|{
                if ui.button("Copy Log").clicked() {
                    ui.output().copied_text = self.config.logs.clone();

                }
                ScrollArea::vertical().show(ui, |ui|{
                    ui.monospace(self.config.logs.clone());
                });

            });
        self.toasts.show(ctx);
            // })
        });
    }
}