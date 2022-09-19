use eframe::egui;
mod submodules;

struct Flasher {
    iron: String,
    version: String,
    langs: Vec<String>,
    lang: String,
    versions_checked: bool,
    vers: Vec<String>,
    run_once: bool
}
impl Default for Flasher {
    fn default() -> Self {
        Self {
            iron: "Pinecil V1".to_string(),
            version: "v2.19".to_string(),
            langs: vec!["EN".to_string(),"BE".to_string(),"BG".to_string(),"CS".to_string(),"DA".to_string(),"DE".to_string(),"EL".to_string(),"ES".to_string(),"FI".to_string(),"FR".to_string(),"HR".to_string(),"HU".to_string(),"IT".to_string(),"JA".to_string(),"LT".to_string(),"NL".to_string(),"NO".to_string(),"PL".to_string(),"PT".to_string(),"RO".to_string(),"RU".to_string(),"SK".to_string(),"SL".to_string(),"SR".to_string(),"SV".to_string(),"TR".to_string(),"UK".to_string(),"VI".to_string(),"YUE".to_string(),"ZH".to_string()],
            lang: "EN".to_string(),
            versions_checked: false,
            vers: vec![],
            run_once: true
        }
    }
}
impl eframe::App for Flasher {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
         if self.run_once {
            // Fonts will be used for styling later
            // Flasher::configure_fonts(ctx);
        }       
        Flasher::render_header(self, ctx, frame);
        Flasher::render_main_windows(self, ctx);

        self.run_once = false;
    }
}

fn main() {

    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "Tangello Music",
        options,
        Box::new(|_cc| Box::new(Flasher::default())),
    );

}
