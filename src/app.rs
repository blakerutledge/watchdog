mod config {
    // use super::*;
}

mod renderer;
mod state;
mod tray_manager;
mod ui;
mod window_manager;

//
// - - - Watchdog
//

use std::collections::HashMap;
use winit::event_loop::{ControlFlow, EventLoop, EventLoopBuilder};

pub fn init() {
    let event_loop: EventLoop<()> = EventLoopBuilder::with_user_event().build();
    let window = window_manager::init(&event_loop);
    let mut renderer = renderer::init(&window);
    let (mut tray, tray_menu) = tray_manager::init();

    let mut state = state::init();
    let mut ui_draw_call = ui::init();

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

    // Window Manager Tick
    window_manager::update(event, state);

    // Tray Event Tick
    tray_manager::update(tray_menu, state);

    // Draw UI
    renderer::update(event, window, renderer, ui_draw_call, state);

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
    if state.action_exit {
        state.action_exit = false;
        exit(control_flow, tray, window);
    }

    // Window Close has been requested
    if state.action_window_close {
        state.action_window_close = false;
        window_manager::close(window);
    }

    // Window Open has been requested
    if state.action_window_open {
        state.action_window_open = false;
        window_manager::open(window);
    }
}
