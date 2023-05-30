use winit::event_loop::EventLoop;
use winit::window::{Window, WindowBuilder};

use super::state::State;

// Set bounds for window dimensions
const INITIAL_WIDTH: u32 = 720;
const INITIAL_HEIGHT: u32 = 1400;
const MIN_WIDTH: u32 = 480;
const MIN_HEIGHT: u32 = 800;

pub fn init(event_loop: &EventLoop<()>, state: &mut State) -> Window {
    //
    // Ingest raw image file, store in binary
    const ICON_IMAGE_DATA: &[u8] = include_bytes!("../../assets/icons/watchdog-logo.png");

    // Read, parse, and convert image file to RGBA data
    let (icon_rgba, icon_width, icon_height) = {
        let image = image::load_from_memory(ICON_IMAGE_DATA)
            .expect("Failed to open icon path")
            .into_rgba8();
        let (width, height) = image.dimensions();
        let rgba = image.into_raw();
        (rgba, width, height)
    };

    // Convert to Window Icon format
    let window_icon = winit::window::Icon::from_rgba(icon_rgba.clone(), icon_width, icon_height)
        .expect("Failed to open window icon");

    //
    // Build Winit Window
    //
    let w = WindowBuilder::new()
        .with_visible(true)
        .with_active(true)
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

    let mhz = w
        .current_monitor()
        .unwrap()
        .refresh_rate_millihertz()
        .unwrap_or(50000);

    state.monitor_refresh_rate = mhz / 1e3 as u32;

    w
}

// Update Loop, called from primary event loop in app.rs, test for user input
pub fn update(event: &winit::event::Event<'_, ()>, state: &mut super::state::State) {
    match event {
        winit::event::Event::WindowEvent { event, .. } => match event {
            //
            // Close Window Event
            winit::event::WindowEvent::CloseRequested => {
                // request window close via state object
                state.actions.window_close = true
            }
            //
            // Maybe other things here
            //
            _ => {}
        },
        _ => {}
    }
}

// Close the window
pub fn close(window: &Window) {
    window.set_visible(false);
}

// Open the window
pub fn open(window: &Window) {
    window.set_visible(true);
    window.focus_window();
}

// Clean up on app exit
pub fn on_exit(window: &Window) {
    window.set_visible(false);
}
