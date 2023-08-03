use eframe::{
    egui::{self, FontData, FontDefinitions},
    epaint::FontFamily,
};

use crate::Flasher;

impl Flasher {
    pub fn configure_fonts(cc: egui::Context) {
        let mut fonts = FontDefinitions::default();
        fonts.font_data.insert(
            "Chinese".to_owned(),
            FontData::from_static(include_bytes!("../../assets/XiaolaiSC-Regular.ttf")),
        );
        fonts
            .families
            .get_mut(&FontFamily::Proportional)
            .unwrap()
            .push("Chinese".to_owned());

        // Use meslolgs for icons
        fonts.font_data.insert(
            "MesloLGS".to_owned(),
            FontData::from_static(include_bytes!("../../assets/SFMono_Nerd_Font_Complete.otf")),
        );
        fonts
            .families
            .get_mut(&FontFamily::Proportional)
            .unwrap()
            .push("MesloLGS".to_owned());

        cc.set_fonts(fonts);
    }
}
