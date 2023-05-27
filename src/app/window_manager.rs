use winit::event_loop::EventLoop;
use winit::window::{Window, WindowBuilder};

const INITIAL_WIDTH: u32 = 1920;
const INITIAL_HEIGHT: u32 = 1080;

pub fn init(event_loop: &EventLoop<()>) -> Window {
    //
    // - - - ICON
    //
    const ICON_IMAGE_DATA: &[u8] = include_bytes!("../../assets/icons/watchdog-logo.png");
    let (icon_rgba, icon_width, icon_height) = {
        let image = image::load_from_memory(ICON_IMAGE_DATA)
            .expect("Failed to open icon path")
            .into_rgba8();
        let (width, height) = image.dimensions();
        let rgba = image.into_raw();
        (rgba, width, height)
    };

    let window_icon = winit::window::Icon::from_rgba(icon_rgba.clone(), icon_width, icon_height)
        .expect("Failed to open window icon");

    let window = WindowBuilder::new()
        .with_visible(false)
        .with_title("Watchdog")
        .with_window_icon(Some(window_icon))
        .with_decorations(true)
        .with_resizable(true)
        .with_transparent(false)
        .with_inner_size(winit::dpi::PhysicalSize {
            width: INITIAL_WIDTH,
            height: INITIAL_HEIGHT,
        })
        .build(event_loop)
        .unwrap();

    window
}
pub fn close(window: &Window) {
    window.set_visible(false);
}
pub fn open(window: &Window) {
    window.set_visible(true);
}

pub fn update() {}

pub fn on_exit(window: &Window) {
    window.set_visible(false);
}
