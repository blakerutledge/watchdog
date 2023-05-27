use winit::event_loop::EventLoop;
use winit::window::{Window, WindowBuilder};

const INITIAL_WIDTH: u32 = 720;
const INITIAL_HEIGHT: u32 = 1400;
const MIN_WIDTH: u32 = 480;
const MIN_HEIGHT: u32 = 800;

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
        .with_min_inner_size(winit::dpi::PhysicalSize {
            width: MIN_WIDTH,
            height: MIN_HEIGHT,
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

pub fn update(event: &winit::event::Event<'_, ()>, state: &mut super::state::State) {
    // Close Window
    match event {
        winit::event::Event::WindowEvent { event, .. } => match event {
            winit::event::WindowEvent::CloseRequested => {
                // request window close via state object
                state.action_window_close = true
            }
            _ => {}
        },
        _ => {}
    }
}

pub fn on_exit(window: &Window) {
    window.set_visible(false);
}
