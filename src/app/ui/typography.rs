use crate::app::ui::*;

const ASSET_FONT_MONOLISA: &[u8] = include_bytes!("../../../assets/fonts/monolisa/MonoLisa.otf");

pub fn load(state: &mut State, context: &egui::Context) {
    // load fonts
    if !state.ui.custom_fonts {
        // init font
        let mut fonts = egui::FontDefinitions::default();

        // Install my own font (maybe supporting non-latin characters).
        // .ttf and .otf files supported.
        fonts.font_data.insert(
            "monolisa".to_owned(),
            egui::FontData::from_static(ASSET_FONT_MONOLISA),
        );

        // Put my font first (highest priority) forboth monospace and proportional text:
        fonts
            .families
            .entry(egui::FontFamily::Proportional)
            .or_default()
            .insert(0, "monolisa".to_owned());

        fonts
            .families
            .entry(egui::FontFamily::Monospace)
            .or_default()
            .insert(0, "monolisa".to_owned());

        // Tell egui to use these fonts:
        context.set_fonts(fonts);

        use egui::FontFamily::Monospace;
        use egui::{FontId, TextStyle};

        let mut style = (*context.style()).clone();
        style.text_styles = [
            (
                TextStyle::Name("Title".into()),
                FontId::new(14.0, Monospace),
            ),
            (
                TextStyle::Name("Subheading".into()),
                FontId::new(16.0, Monospace),
            ),
            (
                TextStyle::Name("TextButton".into()),
                FontId::new(14.0, Monospace),
            ),
            (
                TextStyle::Name("TextButtonSmall".into()),
                FontId::new(12.0, Monospace),
            ),
            (TextStyle::Heading, FontId::new(20.0, Monospace)),
            (TextStyle::Body, FontId::new(12.0, Monospace)),
            (TextStyle::Monospace, FontId::new(12.0, Monospace)),
            (TextStyle::Button, FontId::new(12.0, Monospace)),
            (TextStyle::Small, FontId::new(8.0, Monospace)),
        ]
        .into();
        context.set_style(style);

        // set flag to true so we only do this once
        state.ui.custom_fonts = true;

        println!("Loaded custom fonts");
    }
}
