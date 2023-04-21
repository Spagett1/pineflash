use std::time::Duration;

use eframe::{egui::{CentralPanel, self, menu, RichText, ScrollArea, Stroke}};

use crate::Flasher;

impl Flasher {
    pub fn render_main_windows(&mut self, ctx: &egui::Context) {
        CentralPanel::default().show(ctx, |ui|{

            let _new_style = egui::style::WidgetVisuals {
                bg_fill: egui::Color32::BLACK,
                bg_stroke: Stroke { width: 1., color: egui::Color32::RED },
                rounding: egui::Rounding { nw: 2., ne: 2., sw: 2., se: 2. },
                fg_stroke: Stroke{ width: 1., color: egui::Color32::RED} ,
                expansion: 2.,
            };

            // ctx.set_visuals(egui::style::Visuals { widgets: egui::style::Widgets { 
                // noninteractive: new_style, inactive: new_style, hovered: new_style, active: new_style, open: new_style
            // }, ..Default::default()});

            ui.label("Select your Soldering Iron");
            menu::bar(ui, |ui|{
                egui::ComboBox::from_label(" ")
                    .selected_text(self.config.iron.to_string())
                    .show_ui(ui, |ui| {
                        ui.selectable_value(&mut self.config.iron, "Pinecil V1".to_string(), "Pinecil V1");
                        ui.selectable_value(&mut self.config.iron, "Pinecil V2".to_string(), "Pinecil V2");
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
                            self.config.logs.push_str("PineFlash: PineFlash: Incorrect filetype selected.\n");
                            self.config.picked_path = None;
                        } else {
                            self.config.picked_path = Some(path.display().to_string());
                            self.config.version = "Custom".to_string();
                            self.toasts.dismiss_all_toasts();
                            self.toasts.info("Custom file selected").set_duration(Some(Duration::from_secs(4))).set_closable(false);
                            self.config.logs.push_str("PineFlash: Custom file selected.\n");
                        }
                    }
                }
            });

            ui.label("Select Your Language.");
            ui.add_enabled_ui({
                // bool
                self.config.version != *"Select" && 
                !self.config.download_metadata
            }, |ui|{
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

            if self.config.picked_path.is_some() || //&&
                // self.config.iron_connected.as_ref() == Some(&self.config.int_name) || 
                // self.config.iron_connected.as_ref() == Some(&"Both".to_string()) || 
                self.config.version != *"Custom" && 
                self.config.version != *"Select" && 
                !self.config.download //&& 
                // self.config.iron_connected.as_ref() == Some(&self.config.int_name) || 
                // self.config.iron_connected.as_ref() == Some(&"Both".to_string())
            {
                self.config.ready_to_flash = true

            } else {
                self.config.ready_to_flash = false
            }

            if !self.config.ready_to_flash {
                ui.add_enabled(false, egui::Button::new("Update!")).on_disabled_hover_text(
                    // Tell user why they can not flash
                    if  self.config.iron_connected.as_ref() == Some(&self.config.int_name) ||
                        self.config.iron_connected.as_ref() == Some(&"Both".to_string())
                        { "Select a firmware version or a custom file." } 
                    else if self.config.iron_connected.is_some() && 
                        self.config.iron_connected.as_ref() != Some(&self.config.int_name) &&
                        self.config.iron_connected.as_ref() != Some(&"Both".to_string())
                        {"The selected soldering iron does \nnot match the one currently plugged in."}
                    else if self.config.version != *"Custom" ||
                        self.config.picked_path.is_some() && 
                        self.config.version != *"Select"
                        {"Connect your soldering iron and \nmake sure it is in flashing mode."} 
                    else 
                        {"Please select a firmware version and\nplug your soldering iron in whilst in flashing mode."} 
                );
            } else if ui.button("Update!").clicked() {
                if self.config.version != *"Custom" {
                    self.config.download = true;
                } else {
                    self.toasts.dismiss_all_toasts();
                    self.toasts.info("Flashing.").set_duration(None).set_closable(false);
                    self.config.flash = true;
                }
            };
            
            egui::CollapsingHeader::new("Logs")
                .default_open(true)
                .show(ui, |ui| {
                    if ui.button("Copy Log").clicked() {
                        ui.output().copied_text = self.config.logs.clone();

                    }
                    ScrollArea::vertical().stick_to_bottom(true).show(ui, |ui|{
                        ui.monospace(self.config.logs.clone());
                    });
                });

        self.toasts.show(ctx);
            // })
        });
    }
}