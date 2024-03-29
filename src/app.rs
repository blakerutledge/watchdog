use std::collections::HashMap;
use winit::event_loop::{ControlFlow, EventLoop, EventLoopBuilder};

mod apps;
mod config;
mod osc;
mod perf;
mod renderer;
mod state;
mod stats;
mod tray_manager;
mod ui;
mod window_manager;

///
/// Initializes all sub components and begins the Watchdog Application
///
pub fn init() {
    //
    // Create the shared state object
    let mut state = state::init();

    let mut config = config::init(&mut state);
    let mut apps = apps::init();
    let mut stats = stats::init();

    // Create shared event loop for winit + egui + tray-icon events
    // winit::event_loop::EventLoopBuilder::<Event>::with_user_event().build();
    let event_loop: EventLoop<()> = EventLoopBuilder::with_user_event().build();

    // Create winit window
    let window = window_manager::init(&event_loop, &mut state);

    // Create renderer pipeline using WGPU backend, Winit, & Egui
    let mut renderer = renderer::init(&window);

    // Create system tray element, and (useable) list of menu items in the tray
    let (mut tray, tray_menu) = tray_manager::init();

    // Create the UI
    let mut ui_draw_call = ui::init(&mut state);

    // Initialize the performance tracker
    perf::start_app(&mut state);

    // Begin the event loop, adding in top level
    event_loop.run(move |event, _, control_flow| {
        update(
            &event,
            control_flow,
            &window,
            &tray_menu,
            &mut tray,
            &mut renderer,
            &mut ui_draw_call,
            &mut config,
            &mut apps,
            &mut stats,
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
    window: &winit::window::Window,
    tray_menu: &HashMap<String, tray_manager::MenuElement>,
    tray: &mut tray_icon::TrayIcon,
    renderer: &mut renderer::Renderer,
    ui_draw_call: &mut Box<
        dyn FnMut(&egui::Context, &mut state::State, &mut config::Config, &winit::window::Window),
    >,
    config: &mut config::Config,
    apps: &mut apps::Apps,
    stats: &mut stats::Stats,
    state: &mut state::State,
) {
    // Renderer handles a few various winit events outside of redrawing
    renderer::update(event, renderer);

    // Window Manager update step, parse events and affect state
    window_manager::update(event, window, state);

    // Tray Event update step, parse events and affect state
    tray_manager::update(tray_menu, state);

    // Only draw as fast as the GPU says we should
    let redraw = renderer::test_redraw(event, window);
    if redraw {
        // Start the timer
        perf::start_frame(state);

        // Draw Window UI + affect state (immediate mode)
        renderer::render(window, renderer, ui_draw_call, state, config);
        //
        // Note: timer is finished within the above render call
    }

    // If window is visible, the display will throttle the event loop to the display refresh rate
    if window.is_visible().unwrap_or(false) {
        *control_flow = winit::event_loop::ControlFlow::Poll;
    }
    // If the window is closed, and the app is running in the background,
    // manually throttle the event loop to ~60Hz
    else {
        *control_flow = winit::event_loop::ControlFlow::WaitUntil(
            std::time::Instant::now() + std::time::Duration::from_millis(14),
        );
    }

    // Apply any changes to the state
    apply(control_flow, window, tray, state, config, apps, stats);
}

///
/// Step through any changed state flags, apply actions as necessary
///
fn apply(
    control_flow: &mut ControlFlow,
    window: &winit::window::Window,
    tray: &mut tray_icon::TrayIcon,
    state: &mut state::State,
    config: &mut config::Config,
    apps: &mut apps::Apps,
    _stats: &mut stats::Stats,
    // renderer: &mut renderer::Renderer,
    // tray_menu: &HashMap<String, tray_manager::MenuElement>,
    // ui_draw_call: &mut Box<dyn FnMut(&egui::Context, &mut state::State)>,
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

    // Window Minimize has been requested
    if state.actions.window_minimize {
        state.actions.window_minimize = false;
        window_manager::minimize(window);
    }

    // Window Maximize has been requested
    if state.actions.window_maximize {
        state.actions.window_maximize = false;
        window_manager::maximize(window);
    }

    // Window Un-Maximize has been requested
    if state.actions.window_unmaximize {
        state.actions.window_unmaximize = false;
        window_manager::unmaximize(window);
    }

    // Config has been edited
    if state.actions.config_edited {
        state.actions.config_edited = false;
        config.validate_all();
        config.write(&state.json.filepath);
    }

    // Build Apps Listeners has been requested
    if state.actions.build_listeners {
        state.actions.build_listeners = false;
        apps.build_listeners(&config);
    }

    // Destroy Apps Listeners has been requested
    if state.actions.destroy_listeners {
        state.actions.destroy_listeners = false;
        apps.destroy_listeners();
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
