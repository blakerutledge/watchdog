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
    *control_flow = winit::event_loop::ControlFlow::Poll;

    // Window update
    window_manager::update();

    // Draw UI
    renderer::update(event, window, renderer, ui_draw_call, state);

    // Close Window
    match event {
        winit::event::Event::WindowEvent { event, .. } => match event {
            winit::event::WindowEvent::CloseRequested => {
                window_manager::close(window);
            }
            _ => {}
        },
        _ => {}
    }

    // Tray Event (Left click icon in tray)
    if let Ok(event) = tray_icon::TrayEvent::receiver().try_recv() {
        if event.event == tray_icon::ClickEvent::Left {
            window_manager::open(window);
        }
    }
    // Tray Menu Event (left click menu item, after right clicking to open menu)
    else if let Ok(event) = tray_icon::menu::MenuEvent::receiver().try_recv() {
        let instruction = tray_manager::test_handlers(tray_menu, event.id);
        match instruction.as_str() {
            "app/exit" => {
                exit(control_flow, tray, window);
            }
            "app/instruction_not_mapped" => {
                println!(
                    "App Error: Menu Item with handle {:?} is not mapped to any action",
                    instruction
                );
            }
            _ => {}
        }
    }
}
