use eframe::egui;
mod submodules;

struct Flasher {
    iron: String,
    version: String,
    versions_checked: bool,
    vers: Vec<String>,
    run_once: bool
}
impl Default for Flasher {
    fn default() -> Self {
        Self {
            iron: "Pinecil".to_string(),
            version: "v2.19".to_string(),
            versions_checked: false,
            vers: vec![],
            run_once: true
        }
    }
}
impl eframe::App for Flasher {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
         if self.run_once {
            // Flasher::grab_vers(self);
            // Flasher::configure_fonts(ctx);
            // Flasher::grab_urls(self);
            // for i in &self.urls {
                // println!("{}", i)
            // }
        }       
        Flasher::render_header(self, ctx, frame);
        Flasher::render_main_windows(self, ctx);
        // Flasher::grab_urls(self);

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
