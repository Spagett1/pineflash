#![windows_subsystem = "windows"]
use std::{
    collections::HashMap,
    env,
    fs::{self, File},
    io::{Cursor, Read},
    path::PathBuf,
    time::Duration,
};

use curl::easy::Easy;
use eframe::{egui::{self, ImageSource}, epaint::ColorImage};
use eframe::{
    emath,
    epaint::{Color32, Rounding, Stroke},
    CreationContext,
};
mod submodules;
use egui::Context;
use egui_file::FileDialog;
use egui_notify::{Anchor, Toasts};
use serde::{Deserialize, Serialize};
use serde_json::Value;

const ICON: &[u8] = include_bytes!("../assets/pine64logo.ico");

#[derive(Serialize, Deserialize)]
struct Language {
    language_code: String,
    language_name: String,
}

#[derive(Serialize, Deserialize)]
struct YourValue {
    contents: HashMap<String, Language>,
}

struct FlasherConfig {
    iron: String,
    int_name: String,
    version: String,
    fancy_names: Vec<String>,
    code_names: Vec<String>,
    lang: String,
    versions_checked: bool,
    vers: Vec<String>, download_metadata: bool,
    blisp_version: String,
    run_once_vers: bool,
    download: bool,
    picked_path: Option<String>,
    download_versions: bool,
    download_firm_notify: bool,
    ready_to_flash: bool,
    logs: String,
    json: String,
    iron_connected: Option<String>,
    check_count: i32,
    flash: bool,
    flash_notified_count: i32,
    v2_serial_path: Option<String>,
    // connection_guide_image: Vec<ColorImage>,
    current_step: usize,
    json_checked: bool,
    metadata_path: PathBuf,
    open_file_dialog: Option<FileDialog>,
}

#[derive(Serialize, Deserialize)]
struct FlashSavedConfig {
    pub dark_mode: bool,
}

impl Default for FlashSavedConfig {
    fn default() -> Self {
        Self { dark_mode: true }
    }
}

struct Flasher {
    config: FlasherConfig,
    saved_config: FlashSavedConfig,
    toasts: Toasts,
}

impl Default for FlasherConfig {
    fn default() -> Self {
        Self {
            iron: "Pinecil V2".to_string(),
            int_name: "Pinecilv2".to_string(),
            version: "Select".to_string(),
            fancy_names: vec![],
            code_names: vec![],
            lang: "EN".to_string(),
            versions_checked: false,
            vers: vec![],
            download_metadata: false,
            run_once_vers: true,
            json_checked: false,
            download: false,
            blisp_version: "".to_string(),
            download_versions: true,
            download_firm_notify: true,
            picked_path: None,
            ready_to_flash: false,
            #[cfg(feature = "appimage")]
            logs: format!("Pineflash v{} Linux Appimage\n", env!("CARGO_PKG_VERSION")),
            #[cfg(not(feature = "appimage"))]
            #[cfg(target_os = "linux")]
            logs: format!("Pineflash v{} Linux Native\n", env!("CARGO_PKG_VERSION")),
            #[cfg(target_os = "windows")]
            logs: format!("Pineflash v{} Windows\n", env!("CARGO_PKG_VERSION")),
            #[cfg(target_os = "macos")]
            logs: format!("Pineflash v{} MacOs\n", env!("CARGO_PKG_VERSION")),
            json: "".to_string(),
            iron_connected: None,
            check_count: 0,
            flash: false,
            flash_notified_count: 0,
            v2_serial_path: None,
            // connection_guide_image: [
                //
                // RetainedImage::from_svg_bytes("Step1", include_bytes!("../assets/Step1.svg"))
                //     .unwrap(),
                // RetainedImage::from_svg_bytes("Step2", include_bytes!("../assets/Step2.svg"))
                //     .unwrap(),
                // RetainedImage::from_svg_bytes("Step3", include_bytes!("../assets/Step3.svg"))
                //     .unwrap(),
            // ],
            current_step: 0,
            metadata_path: [std::env::temp_dir(), "metadata.json".into()]
                .iter()
                .collect(),
            open_file_dialog: None,
        }
    }
}

impl Flasher {
    fn new(cc: &CreationContext) -> Flasher {
        let config: FlasherConfig = FlasherConfig::default();
        // Flasher::configure_fonts(&cc.egui_ctx);
        let saved_config: FlashSavedConfig = confy::load("PineFlash", None).unwrap_or_default();
        let toasts = Toasts::default()
            .with_anchor(Anchor::TopRight)
            .with_margin(emath::vec2(0.0, 120.0));

        let mut style: egui::Style = (*cc.egui_ctx.style()).clone();
        style.spacing.item_spacing = egui::vec2(5.0, 10.0);

        cc.egui_ctx.set_style(style);
        let new_style = egui::style::WidgetVisuals {
            bg_fill: Color32::from_rgb(17, 17, 17),
            weak_bg_fill: Color32::from_rgb(17, 17, 17),

            rounding: Rounding {
                nw: 4.,
                ne: 4.,
                sw: 4.,
                se: 4.,
            },

            bg_stroke: Stroke {
                width: 1.,
                color: Color32::from_rgb(140, 140, 140),
            },
            fg_stroke: Stroke {
                width: 1.,
                color: Color32::from_rgb(140, 140, 140),
            },

            expansion: 2.,
        };
        let new_hovered_style = egui::style::WidgetVisuals {
            bg_fill: Color32::from_rgb(17, 17, 17),
            weak_bg_fill: Color32::from_rgb(17, 17, 17),

            rounding: Rounding {
                nw: 4.,
                ne: 4.,
                sw: 4.,
                se: 4.,
            },

            bg_stroke: Stroke {
                width: 1.5,
                color: egui::Color32::from_rgb(56, 55, 55),
            },
            fg_stroke: Stroke {
                width: 1.,
                color: Color32::from_rgb(140, 140, 140),
            },

            expansion: 2.,
        };
        cc.egui_ctx.set_visuals(egui::style::Visuals {
            widgets: egui::style::Widgets {
                active: new_style,
                inactive: new_style,
                hovered: new_hovered_style,
                noninteractive: new_style,
                open: new_hovered_style,
            },
            ..Default::default()
        });

        if !saved_config.dark_mode {
            cc.egui_ctx.set_visuals(egui::Visuals::light());
        }

        Flasher::configure_fonts(cc.egui_ctx.clone());
        Flasher {
            config,
            toasts,
            saved_config,
        }
    }
}

impl eframe::App for Flasher {
    fn update(&mut self, ctx: &Context, _frame: &mut eframe::Frame) {
        // always repaint to have accurate pinecil detection
        ctx.request_repaint();
        ctx.set_pixels_per_point(1.8);

        if self.config.check_count < 180 {
            self.config.check_count += 1
        } else {
            self.config.iron_connected = Flasher::check_connections(self);
            self.config.check_count = 0
        }

        if self.config.download_versions {
            let _ = std::fs::remove_file(self.config.metadata_path.clone());
            self.config.download_versions = !self.config.download_versions;
            self.toasts
                .info("Fetching versions")
                .set_duration(None)
                .set_closable(false);

            std::thread::spawn(|| {
                let mut data = Vec::new();
                let mut handle = Easy::new();
                let mut internet = true;
                handle
                    .url("https://api.github.com/repos/Ralim/IronOS/releases")
                    .unwrap();
                handle.useragent("PineFlash").unwrap();
                {
                    let mut transfer = handle.transfer();
                    transfer
                        .write_function(|new_data| {
                            data.extend_from_slice(new_data);
                            Ok(new_data.len())
                        })
                        .unwrap();
                    if transfer.perform().is_err() {
                        internet = false;
                    }
                }
                let path: PathBuf = [std::env::temp_dir(), "metadata.json".into()]
                    .iter()
                    .collect();

                if internet {
                    let json = String::from_utf8(data).unwrap();
                    std::fs::write(path, json).unwrap();
                } else {
                    std::fs::write(path, "No Internet").unwrap();
                }
            });
        } else if self.config.metadata_path.exists()
            && !self.config.versions_checked
            && !String::from_utf8(std::fs::read(self.config.metadata_path.clone()).unwrap())
                .unwrap()
                .contains("No Internet")
        {
            self.toasts.dismiss_all_toasts();
            self.toasts
                .info("Versions Found")
                .set_duration(Some(Duration::from_secs(5)))
                .set_closable(false);
            self.config
                .logs
                .push_str("PineFlash: Versions successfully fetched.\n");
            self.config.json_checked = !self.config.json_checked;
            let string =
                String::from_utf8(std::fs::read(self.config.metadata_path.clone()).unwrap())
                    .unwrap();
            let json: Result<Value, serde_json::Error> = serde_json::from_str(string.as_str());
            if json.is_err() {
                self.toasts.dismiss_all_toasts();
                self.toasts
                    .error("Could not access github, Online Files Will be Unavailable")
                    .set_duration(Some(Duration::from_secs(5)))
                    .set_closable(false);
                self.config
                    .logs
                    .push_str("PineFlash: Invalid json downloaded, could not fetch versions.\n");
                self.config.versions_checked = true;
            } else {
                for i in 0..3 {
                    let version = json.as_ref().unwrap()[i]["tag_name"].as_str().unwrap();
                    self.config.vers.push(version.to_string());
                }
                self.config.versions_checked = true;
                self.config.download_metadata = true;
            }
        } else if !self.config.versions_checked
            && self.config.metadata_path.exists()
            && String::from_utf8(std::fs::read(self.config.metadata_path.clone()).unwrap())
                .unwrap()
                .contains("No Internet")
        {
            self.toasts.dismiss_all_toasts();
            self.toasts
                .error("No Internet, Online Files Will be Unavailable")
                .set_duration(Some(Duration::from_secs(5)))
                .set_closable(false);
            self.config
                .logs
                .push_str("PineFlash: No internet, could not fetch versions.\n");
            self.config.versions_checked = true;
        }

        Flasher::render_header(self, ctx);
        Flasher::render_main_windows(self, ctx);

        if self.config.download {
            let path: PathBuf = [
                std::env::temp_dir(),
                format!("{}-{}.zip", self.config.version, self.config.int_name).into(),
            ]
            .iter()
            .collect();
            let pathlock: PathBuf = [std::env::temp_dir(), "firmware.lock".into()]
                .iter()
                .collect();
            if self.config.download_firm_notify {
                let _ = std::fs::write(pathlock, "Locked");
                self.toasts
                    .info("Downloading")
                    .set_duration(None)
                    .set_closable(false);
                self.config.logs.push_str(
                    format!(
                        "PineFlash: Downloading Firmware {} {}.\n",
                        self.config.int_name, self.config.version
                    )
                    .as_str(),
                );
                let _ = std::fs::remove_file(path);
                let version = self.config.version.clone();
                let int_name = self.config.int_name.clone();
                std::thread::spawn(move || {
                    let mut data = Vec::new();
                    let mut handle = Easy::new();
                    let path: PathBuf = [
                        std::env::temp_dir(),
                        format!("{}-{}.zip", version, int_name).into(),
                    ]
                    .iter()
                    .collect();
                    let pathlock: PathBuf = [std::env::temp_dir(), "firmware.lock".into()]
                        .iter()
                        .collect();
                    let _ = handle.url(
                        format!(
                            "https://github.com/Ralim/IronOS/releases/download/{}/{}.zip",
                            version, int_name
                        )
                        .as_str(),
                    );
                    let _ = handle.follow_location(true);
                    handle.useragent("PineFlash").unwrap();
                    {
                        let mut transfer = handle.transfer();
                        transfer
                            .write_function(|new_data| {
                                data.extend_from_slice(new_data);
                                Ok(new_data.len())
                            })
                            .unwrap();
                        transfer.perform().unwrap()
                    }
                    let zip = data.as_slice();
                    if std::fs::write(path, zip).is_err() {
                        println!("Could not write zip file")
                    };
                    if std::fs::remove_file(pathlock).is_err() {
                        println!("Could not remove lockfile")
                    };
                });
                self.config.download_firm_notify = false
            } else if !self.config.download_firm_notify && !pathlock.exists() {
                self.toasts.dismiss_all_toasts();
                self.config.logs.push_str("PineFlash: Download Complete.\n");
                self.toasts
                    .info("Flashing.")
                    .set_duration(None)
                    .set_closable(false);
                self.config.download = false;
                self.config.flash = true;
            }
        }
        if !self.config.version.contains("Select")
            && !self.config.version.contains("Custom")
            && self.config.download_metadata
        {
            let path: PathBuf = [std::env::temp_dir(), "langs.zip".into()].iter().collect();
            let pathlock: PathBuf = [std::env::temp_dir(), "langs.lock".into()].iter().collect();
            if self.config.run_once_vers {
                self.toasts
                    .info("Downloading Language information.")
                    .set_duration(None)
                    .set_closable(false);
                let _ = std::fs::remove_file(path);
                let _ = std::fs::write(pathlock, "Locked");
                let version = self.config.version.clone();
                std::thread::spawn(move || {
                    let mut data = Vec::new();
                    let mut handle = Easy::new();
                    let path: PathBuf = [std::env::temp_dir(), "langs.zip".into()].iter().collect();
                    let pathlock: PathBuf =
                        [std::env::temp_dir(), "langs.lock".into()].iter().collect();
                    let _ = std::fs::remove_file(path.clone());

                    let _ = handle.follow_location(true);
                    handle
                        .url(
                            format!(
                                "https://github.com/Ralim/IronOS/releases/download/{}/metadata.zip",
                                version
                            )
                            .as_str(),
                        )
                        .unwrap();
                    handle.useragent("PineFlash").unwrap();
                    {
                        let mut transfer = handle.transfer();
                        transfer
                            .write_function(|new_data| {
                                data.extend_from_slice(new_data);
                                Ok(new_data.len())
                            })
                            .unwrap();
                        transfer.perform().unwrap();
                    }
                    let zip = data.as_slice();
                    if std::fs::write(path, zip).is_err() {
                        println!("Could not write zip file")
                    };
                    if std::fs::remove_file(pathlock).is_err() {
                        println!("Could not remove lockfile")
                    };
                });
                self.config.run_once_vers = false;
            } else if !pathlock.exists() {
                let path: PathBuf = [std::env::temp_dir(), "langs.zip".into()].iter().collect();
                self.toasts.dismiss_all_toasts();
                self.config
                    .logs
                    .push_str("PineFlash: Download of Language Info Complete.\n");
                self.toasts
                    .info("Languages Downloaded.")
                    .set_duration(Some(Duration::from_secs(3)))
                    .set_closable(false);
                let mut file = File::open(path).unwrap();
                let mut data = Vec::new();
                file.read_to_end(&mut data).unwrap();
                let target_dir: PathBuf =
                    [std::env::temp_dir(), "metadata".into()].iter().collect();
                zip_extract::extract(Cursor::new(data), &target_dir, false).unwrap();
                let json_path: PathBuf = [
                    std::env::temp_dir(),
                    "metadata".into(),
                    format!("{}.json", self.config.int_name).into(),
                ]
                .iter()
                .collect();
                self.config.json = fs::read_to_string(json_path).unwrap();
                self.config.download_metadata = false;

                let value: YourValue = serde_json::from_str(self.config.json.as_str()).unwrap();
                for i in value.contents {
                    if !i.0.contains(".hex") {
                        let a = i.1;
                        self.config.fancy_names.push(a.language_name);
                        self.config.code_names.push(a.language_code);
                    }
                }
                self.config.download_metadata = false;
            }
        }

        if self.config.flash {
            if self.config.flash_notified_count < 60 {
                self.config.flash_notified_count += 1
            } else {
                Flasher::flash(self);
            }
        }
    }
}

fn main() {
    let options = eframe::NativeOptions::default();
    // let options = eframe::NativeOptions {
    //     decorated: true,
    //     follow_system_theme: true,
    //     icon_data: Some(eframe::IconData {
    //         rgba: (ICON.to_vec()),
    //         width: (32),
    //         height: (32),
    //     }),
    //     resizable: true,
    //     initial_window_size: Some(emath::Vec2 { x: 780., y: 680. }),
    //     min_window_size: Some(emath::Vec2 { x: 780., y: 280. }),
    //     ..Default::default()
    // };
    match eframe::run_native(
        "PineFlash",
        options,
        // Box::new(|cc| Box::new(egui_extras::install_image_loaders(&cc.egui_ctx) Flasher::new(cc))),
        //
        Box::new(|cc| {
            // This gives us image support:
            egui_extras::install_image_loaders(&cc.egui_ctx);
            Box::new(Flasher::new(cc))
        }),
        ) {
        Ok(_) => (),
        Err(error) => panic!("A massive error occured, not sure whats goin on here: \n {}", error),
    }
}
