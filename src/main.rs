use std::{fs::File, io::Write, time::Duration};

use eframe::{egui::{self}, CreationContext, emath, Theme};
mod submodules;
use egui_notify::{Toasts, Anchor};
use tinyjson::JsonValue;
use poll_promise::Promise;

struct FlasherConfig {
    iron: String,
    int_name: String,
    version: String,
    langs: Vec<String>,
    lang: String,
    versions_checked: bool,
    vers: Vec<String>,
    promise: Option<Promise<ehttp::Result<Vec<String>>>>,
    promise_2: Option<Promise<ehttp::Result<Vec<String>>>>,
    promise_3: Option<Promise<ehttp::Result<Vec<String>>>>,
    metadata_fetched: bool,
    download: bool,
    download_notify: bool, 
    picked_path: Option<String>,
    ready_to_flash: bool,
    logs: String
}
struct Flasher {
    config: FlasherConfig,
    toasts: Toasts,
}

impl Default for FlasherConfig {
    fn default() -> Self {
        Self {
            iron: "Pinecil V1".to_string(),
            int_name: "Pinecil".to_string(),
            version: "Select".to_string(),
            langs: vec!["EN".to_string(),"BE".to_string(),"BG".to_string(),"CS".to_string(),"DA".to_string(),"DE".to_string(),"EL".to_string(),"ES".to_string(),"FI".to_string(),"FR".to_string(),"HR".to_string(),"HU".to_string(),"IT".to_string(),"JA".to_string(),"LT".to_string(),"NL".to_string(),"NO".to_string(),"PL".to_string(),"PT".to_string(),"RO".to_string(),"RU".to_string(),"SK".to_string(),"SL".to_string(),"SR".to_string(),"SV".to_string(),"TR".to_string(),"UK".to_string(),"VI".to_string(),"YUE".to_string(),"ZH".to_string()],
            lang: "EN".to_string(),
            versions_checked: false,
            vers: vec![],
            promise: None,
            promise_2: None,
            promise_3: None,
            metadata_fetched: false,
            download: false,
            download_notify: true,
            picked_path: None,
            ready_to_flash: false,
            logs: "".to_string()
        }
        
    }
}

impl Flasher {
    fn new(cc: &CreationContext) -> Flasher {
        let config: FlasherConfig = FlasherConfig::default();
        // Flasher::configure_fonts(&cc.egui_ctx);

        let toasts = Toasts::default().with_anchor(Anchor::BottomRight);

        Flasher { config, toasts }
    }
}

impl eframe::App for Flasher {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
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
                            for i in 0..5 {
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
                    self.config.logs.push_str("Versions successfully fetched.\n");
                    self.config.versions_checked = true;
                },
                Some(Err(_)) => {
                    self.toasts.dismiss_latest_toast();
                    self.toasts.info("Could not find versions online,\ncheck your internet and try again").set_duration(Some(Duration::from_secs(5))).set_closable(false);
                    self.config.logs.push_str("Error fetching versions.\n");
                    self.config.versions_checked = true;
                },
                None => (),
            }   
        }


        Flasher::render_header(self, ctx, frame);
        Flasher::render_main_windows(self, ctx);

        if self.config.download {
            let ctx = ctx.clone();
            let url = format!("https://github.com/Ralim/IronOS/releases/download/{}/{}.zip", self.config.version, self.config.int_name);
            let path = format!("/tmp/{}-{}.zip", self.config.version, self.config.int_name);
            if self.config.download_notify {
                self.toasts.info("Downloading").set_duration(None).set_closable(false);
                self.config.download_notify = false
            }

            let promise = self.config.promise_2.get_or_insert_with(|| {
                let (sender, promise) = Promise::new();
                let request = ehttp::Request::get(url);
                ehttp::fetch(request, move | result: ehttp::Result<ehttp::Response>|{
                    let data = result.unwrap().bytes;
                    let mut file = File::create(path).unwrap();

                    // println!("{:?}", string);
                    // let json: JsonValue = json_string.parse().unwrap();
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
                    self.toasts.info("Download Complete.").set_duration(Some(Duration::from_secs(3))).set_closable(false);
                    self.config.logs.push_str("Download Complete.\n");
                    self.toasts.info("Flashing.").set_duration(None).set_closable(false);
                    self.config.download = false;
                    Flasher::flash(self)
                },
                Some(Err(_)) => {
                    self.toasts.dismiss_all_toasts();
                    self.toasts.info("Something went wrong with the download, check your internet and try again.").set_duration(Some(Duration::from_secs(5))).set_closable(false);
                    self.config.logs.push_str("Error downloading firmware.\n");
                    self.config.download = false;
                },
                None => {
                },
            }
        }
    }
}

fn main() {

    let mut options = eframe::NativeOptions::default();
    options.decorated = false;
    options.resizable = true;
    options.follow_system_theme = false;
    options.default_theme = Theme::Dark;
    options.initial_window_size = Some(emath::Vec2{ x: 300., y: 275. });
    options.max_window_size = Some(emath::Vec2{ x: 300., y: 275. });
    options.min_window_size = Some(emath::Vec2{ x: 300., y: 275. });
    eframe::run_native(
        "PineFlash",
        options,
        Box::new(|cc| Box::new(Flasher::new(cc))));
}
