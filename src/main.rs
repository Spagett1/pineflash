#! [windows_subsystem = "windows"]

use std::{fs::{File, self}, io::{Write, Read, Cursor}, time::Duration, path::PathBuf, collections::HashMap, env};

use eframe::{CreationContext, emath, Theme};
use eframe::egui;
mod submodules;
use egui_notify::{Toasts, Anchor};
use serde::{Serialize, Deserialize};
use tinyjson::JsonValue;
use poll_promise::Promise;

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
    vers: Vec<String>,
    promise: Option<Promise<ehttp::Result<Vec<String>>>>,
    promise_2: Option<Promise<ehttp::Result<Vec<String>>>>,
    promise_3: Option<Promise<ehttp::Result<Vec<String>>>>,
    download_metadata: bool,
    download: bool,
    picked_path: Option<String>,
    download_notify: bool, 
    download_firm_notify: bool, 
    ready_to_flash: bool,
    logs: String,
    json: String,
    iron_connected: Option<String>,
    check_count: i32,
    flash: bool,
    flash_notified_count: i32, 
    v2_serial_path: Option<String>,
}

#[derive(Serialize, Deserialize)]
struct FlashSavedConfig {
    pub dark_mode: bool,
}

impl Default for FlashSavedConfig {
    fn default() -> Self {
        Self { 
            dark_mode: true, 
        }
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
            promise: None,
            promise_2: None,
            promise_3: None,
            download_metadata: false,
            download: false,
            download_notify: true,
            download_firm_notify: true, 
            picked_path: None,
            ready_to_flash: false,
            logs: format!("Pineflash v{}\n", env!("CARGO_PKG_VERSION")),
            json: "".to_string(),
            iron_connected: None,
            check_count: 0,
            flash: false,
            flash_notified_count: 0,
            v2_serial_path: None
        }
        
     }
}

impl Flasher {
    fn new(cc: &CreationContext) -> Flasher {
        let config: FlasherConfig = FlasherConfig::default();
        // Flasher::configure_fonts(&cc.egui_ctx);
        let saved_config: FlashSavedConfig = confy::load("PineFlash", None).unwrap_or_default();
        let toasts = Toasts::default().with_anchor(Anchor::TopRight).with_margin(emath::vec2(0.0, 40.0));


        if saved_config.dark_mode {
            cc.egui_ctx.set_visuals(egui::Visuals::dark());
        } else {
            cc.egui_ctx.set_visuals(egui::Visuals::light());
        }

        Flasher::configure_fonts(cc.egui_ctx.clone());

        Flasher { config, toasts, saved_config }
    }
}

impl eframe::App for Flasher {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // always repaint to have accurate pinecil detection
        ctx.request_repaint();
        ctx.set_pixels_per_point(1.80);

        if self.config.check_count < 180 {
            self.config.check_count += 1
        } else {
            self.config.iron_connected = Flasher::check_connections(self);
            self.config.check_count = 0
        }


        
        let promise = self.config.promise.get_or_insert_with(|| {
                let ctx = ctx.clone();
                self.toasts.info("Fetching versions").set_duration(None).set_closable(false);
                let (sender, promise) = Promise::new();
                let request = ehttp::Request::get("https://api.github.com/repos/Ralim/IronOS/releases");
                ehttp::fetch(request, move | result: ehttp::Result<ehttp::Response>|{
                    let mut results = vec![];
                    match result {
                        Ok(_) => {
                            let json_string = String::from_utf8(result.unwrap().bytes).unwrap();
                            let json: JsonValue = json_string.parse().unwrap();
                            for i in 0..3 {
                                let version = json[i]["tag_name"].stringify().unwrap();
                                let version = &version[1..version.len()-1];
                                results.push(version.to_string());
                            }
                            sender.send(Ok(results));
                        },
                        Err(_) => {
                            sender.send(Err("Error".to_string()));
                        },
                    }

                    ctx.request_repaint(); // wake up UI thread
                });
                promise
            });
        if !self.config.versions_checked {
            match promise.ready() {
                Some(Ok(vers)) => {
                    self.toasts.dismiss_all_toasts();
                    self.toasts.info("Versions Found").set_duration(Some(Duration::from_secs(5))).set_closable(false);
                    self.config.vers = vers.clone();
                    self.config.logs.push_str("PineFlash: Versions successfully fetched.\n");
                    self.config.versions_checked = true;
                    self.config.download_metadata = true;
                },
                Some(Err(_)) => {
                    self.toasts.dismiss_latest_toast();
                    self.toasts.info("Could not find versions online,\ncheck your internet and try again").set_duration(Some(Duration::from_secs(5))).set_closable(false);
                    self.config.logs.push_str("PineFlash: Error fetching versions.\n");
                    self.config.versions_checked = true;
                },
                None => (),
            }   
        }


        Flasher::render_header(self, ctx);
        Flasher::render_main_windows(self, ctx);

        if self.config.download {
            let ctx = ctx.clone();
            let url = format!("https://github.com/Ralim/IronOS/releases/download/{}/{}.zip", self.config.version, self.config.int_name);
            let path: PathBuf = [ std::env::temp_dir(), format!("{}-{}.zip", self.config.version, self.config.int_name).into() ].iter().collect();
            if self.config.download_firm_notify {
                self.toasts.info("Downloading").set_duration(None).set_closable(false);
                self.config.download_firm_notify = false
            }

            let promise = self.config.promise_2.get_or_insert_with(|| {
                let (sender, promise) = Promise::new();
                let request = ehttp::Request::get(url);
                ehttp::fetch(request, move | result: ehttp::Result<ehttp::Response>|{
                    let data = result.unwrap().bytes;
                    let mut file = File::create(path).unwrap();

                    if file.write_all(data.as_slice()).is_err() {
                        println!("Could not write bytes to zip file");
                    }
                    let results = vec![];
                    // results.push(string);

                    sender.send(Ok(results));
                    ctx.request_repaint(); // wake up UI thread
                });
                promise                                    
            });                                            

            match promise.ready() {                        
                Some(Ok(_)) => {                               
                    self.toasts.dismiss_all_toasts();
                    self.config.logs.push_str("PineFlash: Download Complete.\n");
                    self.toasts.info("Flashing.").set_duration(None).set_closable(false);
                    self.config.download = false;
                    // Flasher::flash(self)
                    self.config.flash = true;
                },
                Some(Err(_)) => {
                    self.toasts.dismiss_all_toasts();
                    self.toasts.info("Something went wrong with the download, check your internet and try again.").set_duration(Some(Duration::from_secs(5))).set_closable(false);
                    self.config.logs.push_str("PineFlash: Error downloading firmware.\n");
                    self.config.download = false;
                },
                None => {
                },
            }
        }
            if !self.config.version.contains("Select") && !self.config.version.contains("Custom") && self.config.download_metadata {
                let ctx = ctx.clone();
                let url = format!("https://github.com/Ralim/IronOS/releases/download/{}/metadata.zip", self.config.version);
                let path: PathBuf = [ std::env::temp_dir(), "metadata.zip".into() ].iter().collect();
                if self.config.download_notify {
                    self.toasts.info("Downloading Language information.").set_duration(None).set_closable(false);
                    self.config.download_notify = false
                }

                let promise = self.config.promise_3.get_or_insert_with(|| {
                    let (sender, promise) = Promise::new();
                    let request = ehttp::Request::get(url);
                    ehttp::fetch(request, move | result: ehttp::Result<ehttp::Response>|{
                        let data = result.unwrap().bytes;
                        let mut file = File::create(path).unwrap();

                        if file.write_all(data.as_slice()).is_err() {
                            println!("Could not write bytes to zip file");
                        }
                        let results = vec![];

                        sender.send(Ok(results));
                        ctx.request_repaint(); // wake up UI thread
                    });
                    promise                                    
                });
            match promise.ready() {                        
                Some(Ok(_)) => {                               
                    self.toasts.dismiss_all_toasts();
                    self.config.logs.push_str("PineFlash: Download of Language Info Complete.\n");
                    self.toasts.info("Download Complete.").set_duration(Some(Duration::from_secs(3))).set_closable(false);
                    let path: PathBuf = [ std::env::temp_dir(), "metadata.zip".into() ].iter().collect();
                    let mut file = File::open(path).unwrap();
                    let mut data = Vec::new();
                    file.read_to_end(&mut data).unwrap();
                    let target_dir: PathBuf = [ std::env::temp_dir(), "metadata".into() ].iter().collect();

                    zip_extract::extract(Cursor::new(data), &target_dir, false).unwrap();

                    // let json_path = format!("/tmp/metadata/{}.json", self.config.int_name);
                    let json_path: PathBuf = [ std::env::temp_dir(), "metadata".into(), format!("{}.json", self.config.int_name ).into() ].iter().collect();
                    self.config.json = fs::read_to_string(json_path).unwrap();


                    let value = serde_json::from_str::<YourValue>(self.config.json.as_str()).unwrap();
                    self.config.logs.push_str("PineFlash: Extraction of Language Info Successful.\n");
                    self.config.download_metadata = false;
                    for i in value.contents {
                        if !i.0.contains(".hex") {
                            let a = i.1;
                            self.config.fancy_names.push(a.language_name);
                            self.config.code_names.push(a.language_code);
                        }
                    }
                },
                Some(Err(_)) => {
                    self.toasts.dismiss_all_toasts();
                    self.toasts.info("Something went wrong with the download, check your internet and try again.").set_duration(Some(Duration::from_secs(5))).set_closable(false);
                    self.config.logs.push_str("PineFlash: Error downloading metadata.\n");
                    self.config.download_metadata = false
                },
                None => {
                },
            }


        }

        if self.config.flash {
            if self.config.flash_notified_count < 60 {
                self.config.flash_notified_count += 1
            } else {
                Flasher::flash(self);
                self.config.flash_notified_count = 0
            }
        }
    }
}

fn main() {
    let options = eframe::NativeOptions { 
            decorated: true, 
            follow_system_theme: false, 
            default_theme: Theme::Dark, 
            icon_data: Some(eframe::IconData { rgba: (ICON.to_vec()), 
            width: (32), height: (32) }), 
            resizable: true, 
            initial_window_size: Some(emath::Vec2{ x: 590., y: 500. }), 
            min_window_size: Some(emath::Vec2{ x: 590., y: 500. }), 
            ..Default::default() 
        };

    eframe::run_native(
        "PineFlash",
        options,
        Box::new(|cc| Box::new(Flasher::new(cc))));
}
