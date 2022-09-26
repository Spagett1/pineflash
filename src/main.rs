use std::{fs::File, io::Write};

use eframe::egui::{self};
mod submodules;
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
    download: bool,
}
struct Flasher {
    config: FlasherConfig,
}

impl Default for FlasherConfig {
    fn default() -> Self {
        Self {
            iron: "Pinecil V1".to_string(),
            int_name: "Pinecil".to_string(),
            version: "v2.19".to_string(),
            langs: vec!["EN".to_string(),"BE".to_string(),"BG".to_string(),"CS".to_string(),"DA".to_string(),"DE".to_string(),"EL".to_string(),"ES".to_string(),"FI".to_string(),"FR".to_string(),"HR".to_string(),"HU".to_string(),"IT".to_string(),"JA".to_string(),"LT".to_string(),"NL".to_string(),"NO".to_string(),"PL".to_string(),"PT".to_string(),"RO".to_string(),"RU".to_string(),"SK".to_string(),"SL".to_string(),"SR".to_string(),"SV".to_string(),"TR".to_string(),"UK".to_string(),"VI".to_string(),"YUE".to_string(),"ZH".to_string()],
            lang: "EN".to_string(),
            versions_checked: false,
            vers: vec![],
            promise: None,
            promise_2: None,
            download: false,
        }
        
    }
}

impl Flasher {
    fn new() -> Flasher {
        let config: FlasherConfig = FlasherConfig::default();


        Flasher { config }
    }
}

impl eframe::App for Flasher {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        let promise = self.config.promise.get_or_insert_with(|| {
                let ctx = ctx.clone();
                let (sender, promise) = Promise::new();
                let request = ehttp::Request::get("https://api.github.com/repos/Ralim/IronOS/releases");
                ehttp::fetch(request, move | result: ehttp::Result<ehttp::Response>|{
                    let json_string = String::from_utf8(result.unwrap().bytes).unwrap();
                    let json: JsonValue = json_string.parse().unwrap();
                    let mut results = vec![];
                    for i in 0..5 {
                        let version = json[i]["tag_name"].stringify().unwrap();
                        let version = &version[1..version.len()-1];
                        results.push(version.to_string());
                    }
                    sender.send(Ok(results));
                    ctx.request_repaint(); // wake up UI thread
                });
                promise
            });
        if !self.config.versions_checked {
            match promise.ready() {
                Some(Ok(vers)) => {
                    self.config.vers = vers.clone();
                    self.config.versions_checked = true;
                },
                Some(Err(_)) => (),
                None => (),
            }   
        }
        // if self.config.run_once {
            // Fonts will be used for styling later
            // Flasher::configure_fonts(ctx);
        // }       
        // println!("{:?}", Flasher::default().vers);
        Flasher::render_header(self, ctx, frame);
        Flasher::render_main_windows(self, ctx);

        if self.config.download {
            let url = format!("https://github.com/Ralim/IronOS/releases/download/{}/{}.zip", self.config.version, self.config.int_name);
            let path = format!("/tmp/{}-{}.zip", self.config.version, self.config.int_name);

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
                });
                promise                                    
            });                                            

            match promise.ready() {                        
                Some(_) => {                               
                    self.config.download = false;
                    Flasher::flash(self)
                },
                None => (),
            }
        }
        // self.config.run_once = false;
    }
}

fn main() {

    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "Tangello Music",
        options,
        Box::new(|_cc| Box::new(Flasher::new())));



}
