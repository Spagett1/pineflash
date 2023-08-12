use std::time::Duration;
#[cfg(target_family = "windows")]
use std::os::windows::process::CommandExt;

#[cfg(target_family = "windows")]
use std::path::PathBuf;
#[cfg(feature = "appimage")]
use std::path::PathBuf;

use eframe::{
    egui::{self, Button, CentralPanel, RichText, ScrollArea},
    epaint::{Color32, Rounding},
};
use egui::emath;
use egui::Context;
use egui::Vec2;
use egui_file::FileDialog;
use simple_home_dir::home_dir;
use std::process::Command;
use version_compare::Version;

use crate::Flasher;
impl Flasher {
    pub fn render_main_windows(&mut self, ctx: &Context) {
        CentralPanel::default().show(ctx, |ui|{
            ui.horizontal(|ui| {
                if self.config.iron_connected.is_none() {
                    ui.colored_label(Color32::RED, RichText::new("").heading());
                    ui.label("Soldering Iron Disconnected");
                } else {
                    ui.colored_label(Color32::GREEN, RichText::new("").heading());
                    ui.label("Soldering Iron Connected");
                }
            });
            ui.horizontal(|ui| {
                // Disable strokes except for hovered
                ui.visuals_mut().widgets.active.bg_stroke = eframe::epaint::Stroke{ width: 0., color: egui::Color32::RED};
                ui.visuals_mut().widgets.inactive.bg_stroke = eframe::epaint::Stroke{ width: 0., color: egui::Color32::RED};

                let width = ui.available_width();
                ui.vertical(|ui| {
                    ui.label("Select your Soldering Iron");
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
                let width_now = ui.available_width();

                ui.add_space(width / 2. - ((width - width_now) * 1.3));

                ui.vertical(|ui| {
                    ui.label("Specify Version");
                        ui.horizontal(|ui| {

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
                            if ui.button(RichText::new(" ").size(15.)).clicked() {
                                let mut dialog = FileDialog::open_file(home_dir()).default_size(emath::Vec2 {x:264., y: 262.});
                                dialog.open();
                                self.config.open_file_dialog = Some(dialog);
                            }
                            if let Some(dialog) = &mut self.config.open_file_dialog {
                                if dialog.show(ctx).selected() {
                                    if let Some(file) = dialog.path() {
                                        if self.config.int_name == "Pinecilv2" {

                                            #[cfg(feature = "appimage")]
                                            let path = format!(
                                                "{}/linux/blisp",
                                                std::env::current_dir().unwrap().to_str().unwrap()
                                            );
                                            #[cfg(feature = "appimage")]
                                            let tool_path: PathBuf = [std::env::temp_dir(), "flash_tools".into()]
                                                .iter()
                                                .collect();

                                            #[cfg(feature = "appimage")]
                                            let _ = std::fs::create_dir(tool_path.clone());
                                            #[cfg(feature = "appimage")]
                                            let blisppath: PathBuf = [tool_path, "blisp".into()].iter().collect();
                                            #[cfg(feature = "appimage")]
                                            std::fs::copy(path, blisppath.clone()).unwrap();
                                            #[cfg(not(feature = "appimage"))]
                                            #[cfg(target_family = "unix")]
                                            let blisppath = "blisp";

                                            #[cfg(target_family = "unix")]
                                            let blisp_version = String::from_utf8( Command::new(blisppath)
                                                .env("PATH", "/usr/local/sbin:/usr/local/bin:/usr/bin:/bin")
                                                .arg("--version").output().expect("Could not find blisp").stdout).unwrap();
                       
                                            #[cfg(target_family = "windows")]
                                            let blisp: PathBuf = [
                                                std::env::current_dir().unwrap(),
                                                "tools".into(),
                                                "blisp.exe".into(),
                                            ]
                                            .iter()
                                            .collect();

                                            #[cfg(target_os = "windows")]
                                            let blisp_version = String::from_utf8( Command::new(blisp)
                                                .creation_flags(0x00000008)
                                                .arg("--version").output().expect("Could not find blisp").stdout).unwrap();

                                            let ver = blisp_version.split("\n");
                                            for i in ver {
                                                if i.contains("v") {
                                                    for ii in i.split("v") {
                                                        if ii.contains(".") {
                                                            self.config.blisp_version = ii.to_string();
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                        if !file.display().to_string().contains("dfu") && self.config.int_name == "Pinecil"  || self.config.int_name == "Pinecilv2" && !file.display().to_string().contains("bin") && !file.display().to_string().contains("dfu") {
                                            self.toasts.dismiss_all_toasts();
                                            self.toasts.error("File has the incorrect format").set_duration(Some(Duration::from_secs(4))).set_closable(false);
                                            self.config.logs.push_str("PineFlash: Incorrect filetype selected.\n");
                                            self.config.picked_path = None;
                                            
                                        } else if Version::from(self.config.blisp_version.as_str()).unwrap() < Version::from("0.0.4").unwrap() && file.display().to_string().contains("dfu") && self.config.int_name == "Pinecilv2" {
                                            self.toasts.dismiss_all_toasts();
                                            self.toasts.error("Your version of blisp can not flash boot image files.\nPlease update it to version 0.0.4 or higher").set_duration(Some(Duration::from_secs(4))).set_closable(false);
                                            self.config.logs.push_str(format!("PineFlash: Old blisp version {}.\n", self.config.blisp_version).as_str());
                                            self.config.picked_path = None;

                                        }
                                        else {
                                            self.config.picked_path = Some(file.display().to_string());
                                            self.config.version = "Custom".to_string();
                                            self.toasts.dismiss_all_toasts();
                                            self.toasts.info("Custom file selected").set_duration(Some(Duration::from_secs(4))).set_closable(false);
                                            self.config.logs.push_str("PineFlash: Custom file selected.\n");

                                        }
                                    }
                                }
                            }
                    });
                });
                ui.add_space(ui.available_width() - ((width - width_now) / 1.2));
                ui.vertical(|ui| {
                    ui.label("Select Your Language.");
                    ui.add_enabled_ui({
                        self.config.version != *"Select" && 
                        !self.config.download_metadata
                    }, |ui|{
                        ui.horizontal(|ui| {
                            ui.add_space(10.);
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
                        })
                    });
                })
            });
            if self.config.picked_path.is_some() &&
                self.config.iron_connected.as_ref() == Some(&self.config.int_name) ||
                self.config.iron_connected.as_ref() == Some(&"Both".to_string()) ||
                self.config.version != *"Custom" &&
                self.config.version != *"Select" &&
                !self.config.download &&
                self.config.iron_connected.as_ref() == Some(&self.config.int_name) ||
                self.config.iron_connected.as_ref() == Some(&"Both".to_string())
            {
                self.config.ready_to_flash = true

            } else {
                self.config.ready_to_flash = false
            }

            ui.add_space(25.);
            // Disable strokes except for hovered
            ui.visuals_mut().widgets.active.bg_stroke = eframe::epaint::Stroke{ width: 0., color: egui::Color32::RED};
            ui.visuals_mut().widgets.inactive.bg_stroke = eframe::epaint::Stroke{ width: 0., color: egui::Color32::RED};

            if !self.config.ready_to_flash {

                ui.add_enabled_ui(false, |ui| {
                    ui.add_sized([80., 10.], egui::Button::new("Update!")).on_disabled_hover_text(
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
                    )
                });
            } else if ui.add_sized([80., 10.], Button::new("Update")).clicked() {
                if self.config.version != *"Custom" {
                    self.config.download = true;
                } else {
                    self.toasts.dismiss_all_toasts();
                    self.toasts.info("Flashing.").set_duration(None).set_closable(false);
                    self.config.flash = true;
                }
            };
            ui.separator();

            egui::CollapsingHeader::new("Connection Guide")
                .default_open(false)
                .show_unindented(ui, |ui|
            {
                ui.horizontal(|ui|{
                    ui.add_space(10.);
                    egui::Frame::none()
                        .fill(egui::Color32::from_rgb(17, 17, 17))
                        .rounding(Rounding { nw: 4., ne: 4., sw: 4., se: 4. })
                        .show(ui, |ui|
                    {
                        ui.vertical(|ui|{
                            ui.image(self.config.connection_guide_image[self.config.current_step].texture_id(ctx), Vec2 { x: ui.available_width() - 10., y: (ui.available_width() - 10.) / 3.4 });
                            ui.horizontal(|ui|{
                                ui.with_layout(egui::Layout::left_to_right(eframe::emath::Align::TOP), |ui| {
                                    ui.add_space(10.);
                                    if self.config.current_step == 0 {
                                        ui.add_enabled_ui(false, |ui|{
                                            ui.add_sized([80., 10.], Button::new("Previous").fill(egui::Color32::from_rgb(27, 27, 27)))
                                        });
                                    } else if ui.add_sized([80., 10.], Button::new("Previous").fill(egui::Color32::from_rgb(27, 27, 27))).clicked() {
                                            self.config.current_step -= 1;
                                    }

                                });
                                ui.with_layout(egui::Layout::right_to_left(eframe::emath::Align::TOP), |ui| {
                                    ui.add_space(10.);

                                    if self.config.current_step == 2 {
                                        ui.add_enabled_ui(false, |ui|{
                                            ui.add_sized([80., 10.], Button::new("Next").fill(egui::Color32::from_rgb(27, 27, 27)))
                                        });
                                    } else if ui.add_sized([80., 10.], Button::new("Next").fill(egui::Color32::from_rgb(27, 27, 27))).clicked() {
                                            self.config.current_step += 1;
                                    }
                                });
                            });
                            ui.add_space(5.);
                        });
                    });
                });
            });
            egui::CollapsingHeader::new("Logs")
                .default_open(false)
                .show_unindented(ui, |ui|
            {
                ui.horizontal(|ui|{
                    ui.add_space(10.);
                    egui::Frame::none()
                        .fill(egui::Color32::from_rgb(17, 17, 17))
                        .rounding(Rounding { nw: 4., ne: 4., sw: 4., se: 4. })
                        .show(ui, |ui|
                    {
                        ui.vertical(|ui|{
                        ui.add_space(10.);

                            ui.horizontal(|ui|{
                                ui.add_space(10.);
                                if ui.add(Button::new("Copy Log").fill(egui::Color32::from_rgb(27, 27, 27))).clicked() {
                                    // ui.output().copied_text = self.config.logs.clone();
                                    ui.output_mut(|i| i.copied_text = self.config.logs.clone());
                                }
                            });
                            ScrollArea::vertical().auto_shrink([false, false]).stick_to_bottom(true).show(ui, |ui|{
                                ui.horizontal(|ui|{
                                    ui.add_space(10.);
                                    ui.vertical(|ui|{
                                        ui.monospace(self.config.logs.clone());
                                    });
                                    ui.add_space(5.);
                                });
                            });
                        });
                    });
                })
            });

        self.toasts.show(ctx);
        });
    }
}
