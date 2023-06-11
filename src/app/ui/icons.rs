use crate::app::ui::*;
use std::collections::HashMap;

// Logo
const ASSET_ICON_LOGO: &[u8] = include_bytes!("../../../assets/icons/icon-logo.png");

// Window Buttons
const ASSET_ICON_MIN: &[u8] = include_bytes!("../../../assets/icons/icon-min.png");
const ASSET_ICON_MAX: &[u8] = include_bytes!("../../../assets/icons/icon-max.png");
const ASSET_ICON_UNMAX: &[u8] = include_bytes!("../../../assets/icons/icon-unmax.png");
const ASSET_ICON_CLOSE: &[u8] = include_bytes!("../../../assets/icons/icon-close.png");

// Nav Bar
const ASSET_ICON_CONFIG: &[u8] = include_bytes!("../../../assets/icons/icon-config.png");
const ASSET_ICON_APPS: &[u8] = include_bytes!("../../../assets/icons/icon-apps.png");
const ASSET_ICON_STATS: &[u8] = include_bytes!("../../../assets/icons/icon-stats.png");
const ASSET_ICON_EXIT: &[u8] = include_bytes!("../../../assets/icons/icon-exit.png");

// Config section
const ASSET_ICON_LOAD: &[u8] = include_bytes!("../../../assets/icons/icon-load.png");
const ASSET_ICON_SAVE: &[u8] = include_bytes!("../../../assets/icons/icon-save.png");
const ASSET_ICON_RESET: &[u8] = include_bytes!("../../../assets/icons/icon-reset.png");
const ASSET_ICON_CREATE: &[u8] = include_bytes!("../../../assets/icons/icon-create.png");
const ASSET_ICON_DELETE: &[u8] = include_bytes!("../../../assets/icons/icon-delete.png");

//
// add more here...
//

pub fn init(state: &mut State) {
    // Create map of keys to binary assets
    state.ui.asset_map = HashMap::from([
        // Logo
        ("icon_logo", ASSET_ICON_LOGO),
        // Window Buttons
        ("icon_min", ASSET_ICON_MIN),
        ("icon_max", ASSET_ICON_MAX),
        ("icon_unmax", ASSET_ICON_UNMAX),
        ("icon_close", ASSET_ICON_CLOSE),
        // Nav Bar
        ("icon_config", ASSET_ICON_CONFIG),
        ("icon_apps", ASSET_ICON_APPS),
        ("icon_stats", ASSET_ICON_STATS),
        ("icon_exit", ASSET_ICON_EXIT),
        // Config
        ("icon_load", ASSET_ICON_LOAD),
        ("icon_save", ASSET_ICON_SAVE),
        ("icon_reset", ASSET_ICON_RESET),
        ("icon_create", ASSET_ICON_CREATE),
        ("icon_delete", ASSET_ICON_DELETE),
        //
        // ... and here
        //
    ]);
}

pub fn load(state: &mut State, context: &egui::Context) {
    //
    // On the first frame, load each image binary into a texture for egui to use
    //
    if !state.ui.loaded_textures {
        for (slug, data) in state.ui.asset_map.iter() {
            if !state.ui.textures.contains_key(slug) {
                create_tex(data, slug, context, &mut state.ui.textures);
                println!("created {slug} texture");
            }
        }
    }
    state.ui.loaded_textures = true;
}

fn create_tex(
    image_data: &[u8],
    slug: &'static str,
    context: &egui::Context,
    textures: &mut HashMap<&str, (Vec2, egui::TextureHandle)>,
) {
    let image = image::load_from_memory(image_data)
        .expect(format!("Failed to load image {}", slug).as_str());
    let size = [image.width() as _, image.height() as _];
    let image_buffer = image.to_rgba8();
    let pixels = image_buffer.as_flat_samples();

    let i = egui::ColorImage::from_rgba_unmultiplied(size, pixels.as_slice());

    let raw_size = egui::Vec2::new(size[0] as f32, size[1] as f32);
    let tex = context.load_texture(slug, i, Default::default());

    textures.insert(slug, (raw_size, tex));
}
