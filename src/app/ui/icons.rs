use crate::app::ui::*;

const ASSET_ICON_CONFIG: &[u8] = include_bytes!("../../../assets/icons/icon-config.png");
const ASSET_ICON_APPS: &[u8] = include_bytes!("../../../assets/icons/icon-apps.png");
const ASSET_ICON_STATS: &[u8] = include_bytes!("../../../assets/icons/icon-stats.png");
const ASSET_ICON_EXIT: &[u8] = include_bytes!("../../../assets/icons/icon-exit.png");

const ASSET_ICON_LOGO: &[u8] = include_bytes!("../../../assets/icons/icon-logo.png");

const ASSET_ICON_MIN: &[u8] = include_bytes!("../../../assets/icons/icon-min.png");
const ASSET_ICON_MAX: &[u8] = include_bytes!("../../../assets/icons/icon-max.png");
const ASSET_ICON_UNMAX: &[u8] = include_bytes!("../../../assets/icons/icon-unmax.png");
const ASSET_ICON_CLOSE: &[u8] = include_bytes!("../../../assets/icons/icon-close.png");

const ASSET_FONT_MONOLISA: &[u8] = include_bytes!("../../../assets/fonts/monolisa/MonoLisa.otf");

pub fn load(state: &mut State, context: &egui::Context) {
    // load any missing images
    if !state.ui.textures.contains_key("icon_config") {
        create_tex(ASSET_ICON_CONFIG, "icon_config", context, state);
        println!("created icon_config texture");
    }

    if !state.ui.textures.contains_key("icon_apps") {
        create_tex(ASSET_ICON_APPS, "icon_apps", context, state);
        println!("created icon_apps texture");
    }

    if !state.ui.textures.contains_key("icon_stats") {
        create_tex(ASSET_ICON_STATS, "icon_stats", context, state);
        println!("created icon_stats texture");
    }

    if !state.ui.textures.contains_key("icon_exit") {
        create_tex(ASSET_ICON_EXIT, "icon_exit", context, state);
        println!("created icon_exit texture");
    };

    if !state.ui.textures.contains_key("icon_logo") {
        create_tex(ASSET_ICON_LOGO, "icon_logo", context, state);
        println!("created logo texture");
    }

    if !state.ui.textures.contains_key("icon_min") {
        create_tex(ASSET_ICON_MIN, "icon_min", context, state);
        println!("created icon_min texture");
    }

    if !state.ui.textures.contains_key("icon_max") {
        create_tex(ASSET_ICON_MAX, "icon_max", context, state);
        println!("created icon_max texture");
    }

    if !state.ui.textures.contains_key("icon_unmax") {
        create_tex(ASSET_ICON_UNMAX, "icon_unmax", context, state);
        println!("created icon_unmax texture");
    }

    if !state.ui.textures.contains_key("icon_close") {
        create_tex(ASSET_ICON_CLOSE, "icon_close", context, state);
        println!("created icon_close texture");
    }
    /*
       add more here
    */

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
            (TextStyle::Heading, FontId::new(20.0, Monospace)),
            (TextStyle::Body, FontId::new(12.0, Monospace)),
            (TextStyle::Monospace, FontId::new(12.0, Monospace)),
            (TextStyle::Button, FontId::new(12.0, Monospace)),
            (TextStyle::Small, FontId::new(8.0, Monospace)),
        ]
        .into();
        context.set_style(style);

        // set flag to true so we only do this once
        state.ui.custom_fonts = true
    }
}

fn create_tex(image_data: &[u8], slug: &str, context: &egui::Context, state: &mut State) {
    let image = image::load_from_memory(image_data)
        .expect(format!("Failed to load image {}", slug).as_str());
    let size = [image.width() as _, image.height() as _];
    let image_buffer = image.to_rgba8();
    let pixels = image_buffer.as_flat_samples();

    let i = egui::ColorImage::from_rgba_unmultiplied(size, pixels.as_slice());

    let raw_size = egui::Vec2::new(size[0] as f32, size[1] as f32);
    let tex = context.load_texture(slug, i, Default::default());

    state.ui.textures.insert(slug.to_string(), (raw_size, tex));
}
