use std::collections::HashMap;
use winit::event_loop::{ControlFlow, EventLoop, EventLoopBuilder};

mod config;
mod renderer;
mod state;
mod tray_manager;
mod ui;
mod window_manager;

///
/// Initializes all sub components and begins the Watchdog Application
///
pub fn init() {
    //
    // Create shared event loop for winit + egui + tray-icon events
    let event_loop: EventLoop<()> = EventLoopBuilder::with_user_event().build();

    // Create winit window
    let window = window_manager::init(&event_loop);

    // Create renderer pipeline using WGPU backend, Winit, & Egui
    let mut renderer = renderer::init(&window);

    // Create system tray element, and (useable) list of menu items in the tray
    let (mut tray, tray_menu) = tray_manager::init();

    // Create the shared state object
    let mut state = state::init();

    // Create the UI
    let mut ui_draw_call = ui::init();

    // Begin the event loop, adding in top level
    event_loop.run(move |event, _, control_flow| {
        update(
            &event,
            control_flow,
            &mut renderer,
            &mut tray,
            &window,
            &tray_menu,
            &mut ui_draw_call,
            &mut state,
        )
    });
}

///
/// Primary Event Loop Handler, delegates work to various components
///
fn update(
    event: &winit::event::Event<'_, ()>,
    control_flow: &mut ControlFlow,
    renderer: &mut renderer::Renderer,
    tray: &mut tray_icon::TrayIcon,
    window: &winit::window::Window,
    tray_menu: &HashMap<String, tray_manager::MenuElement>,
    ui_draw_call: &mut Box<dyn FnMut(&egui::Context, &mut state::State)>,
    state: &mut state::State,
) {
    // Set to Poll instead of Wait on Windows so we can actually
    // capture the tray left click event when it happens
    *control_flow = winit::event_loop::ControlFlow::Poll;

    // Window Manager update step, parse events and affect state
    window_manager::update(event, state);

    // Tray Event update step, parse events and affect state
    tray_manager::update(tray_menu, state);

    // Draw Window UI + affect state (immediate mode)
    renderer::update(event, window, renderer, ui_draw_call, state);

    // Apply any changes to the state
    apply(
        control_flow,
        // renderer,
        tray,
        window,
        // tray_menu,
        // ui_draw_call,
        state,
    );
}

///
/// Step through any changed state flags, apply actions as necessary
///
fn apply(
    control_flow: &mut ControlFlow,
    // renderer: &mut renderer::Renderer,
    tray: &mut tray_icon::TrayIcon,
    window: &winit::window::Window,
    // tray_menu: &HashMap<String, tray_manager::MenuElement>,
    // ui_draw_call: &mut Box<dyn FnMut(&egui::Context, &mut state::State)>,
    state: &mut state::State,
) {
    // Application Exit has been requested
    if state.actions.app_exit {
        state.actions.app_exit = false;
        exit(control_flow, tray, window);
    }

    // Window Close has been requested
    if state.actions.window_close {
        state.actions.window_close = false;
        window_manager::close(window);
    }

    // Window Open has been requested
    if state.actions.window_open {
        state.actions.window_open = false;
        window_manager::open(window);
    }
}

///
/// Handles Application Exit
///
fn exit(
    control_flow: &mut ControlFlow,
    tray: &mut tray_icon::TrayIcon,
    window: &winit::window::Window,
) {
    // Do some clean up
    window_manager::on_exit(window);
    tray_manager::on_exit(tray);
    *control_flow = ControlFlow::Exit;
}
