use eframe::{egui::{self, FontDefinitions, FontData}, epaint::{FontFamily}};

use crate::Flasher;

impl Flasher {
    pub fn configure_fonts(ctx: &egui::Context) {
        let mut fonts = FontDefinitions::default();
        // Imports the MesloLGS font from its ttf file in order to enable support for other characters
        fonts.font_data.insert(
            "MesloLGS".to_owned(),
            FontData::from_static(include_bytes!("../../assets/SFMono_Nerd_Font_Complete.otf")),
        );
        fonts
            .families
            .get_mut(&FontFamily::Proportional)
            .unwrap()
            .push("MesloLGS".to_owned());
    
        ctx.set_fonts(fonts);
    
    }
}