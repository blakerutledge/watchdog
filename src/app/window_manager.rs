use winit::event_loop::EventLoop;
use winit::window::{ResizeDirection, Window, WindowBuilder};

use std::f64;

use super::state::State;

// Set bounds for window dimensions
const INITIAL_WIDTH: u32 = 540;
const INITIAL_HEIGHT: u32 = 800;
const MIN_WIDTH: u32 = 480;
const MIN_HEIGHT: u32 = 600;
const BORDER: f64 = 10.0;

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
        .with_decorations(false)
        .with_transparent(true)
        .with_resizable(true)
        .with_inner_size(winit::dpi::LogicalSize {
            width: INITIAL_WIDTH,
            height: INITIAL_HEIGHT,
        })
        .with_min_inner_size(winit::dpi::LogicalSize {
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

    state.perf.monitor_refresh_rate = mhz / 1e3 as u32;

    w
}

// Update Loop, called from primary event loop in app.rs, test for user input
pub fn update(
    event: &winit::event::Event<'_, ()>,
    window: &winit::window::Window,
    state: &mut super::state::State,
) {
    match event {
        winit::event::Event::WindowEvent { event, .. } => match event {
            //
            // Close Window Event
            winit::event::WindowEvent::CloseRequested => {
                // request window close via state object
                state.actions.window_close = true
            }

            winit::event::WindowEvent::CursorMoved { position, .. } => {
                if !window.is_decorated() {
                    if state.ui.resizing {
                        let window_size = window.inner_size();
                        let window_position = window.outer_position().unwrap();

                        match state.ui.cursor_location.unwrap() {
                            ResizeDirection::East => {
                                window.set_inner_size(winit::dpi::PhysicalSize::new(
                                    f64::max(
                                        position.x as f64,
                                        MIN_WIDTH as f64 * window.scale_factor(),
                                    ),
                                    window_size.height as f64,
                                ));
                            }

                            // TO DO: finish manually enforcing min & max window size limits
                            ResizeDirection::North => {
                                // clamp
                                let (h, y) = if window_size.height as f64 - position.y
                                    < MIN_HEIGHT as f64 * window.scale_factor()
                                {
                                    let diff = window_size.height
                                        - (MIN_HEIGHT as f64 * window.scale_factor()) as u32;
                                    (
                                        MIN_HEIGHT as f64 * window.scale_factor(),
                                        window_position.y + diff as i32,
                                    )
                                } else {
                                    (
                                        window_size.height as f64 - position.y,
                                        window_position.y + position.y as i32,
                                    )
                                };
                                window.set_inner_size(winit::dpi::PhysicalSize::new(
                                    window_size.width as f64,
                                    h,
                                ));
                                window.set_outer_position(winit::dpi::PhysicalPosition::new(
                                    window_position.x,
                                    y,
                                ));
                            }
                            ResizeDirection::NorthEast => {
                                let (h, y) = if window_size.height as f64 - position.y
                                    < MIN_HEIGHT as f64 * window.scale_factor()
                                {
                                    let diff = window_size.height
                                        - (MIN_HEIGHT as f64 * window.scale_factor()) as u32;
                                    (
                                        MIN_HEIGHT as f64 * window.scale_factor(),
                                        window_position.y + diff as i32,
                                    )
                                } else {
                                    (
                                        window_size.height as f64 - position.y,
                                        window_position.y + position.y as i32,
                                    )
                                };
                                window.set_inner_size(winit::dpi::PhysicalSize::new(
                                    f64::max(
                                        position.x as f64,
                                        MIN_WIDTH as f64 * window.scale_factor(),
                                    ),
                                    h,
                                ));
                                window.set_outer_position(winit::dpi::PhysicalPosition::new(
                                    window_position.x,
                                    y,
                                ));
                            }
                            ResizeDirection::NorthWest => {
                                let (w, x) = if window_size.width as f64 - position.x
                                    < MIN_WIDTH as f64 * window.scale_factor()
                                {
                                    let diff = window_size.width
                                        - (MIN_WIDTH as f64 * window.scale_factor()) as u32;
                                    (
                                        MIN_WIDTH as f64 * window.scale_factor(),
                                        window_position.x + diff as i32,
                                    )
                                } else {
                                    (
                                        window_size.width as f64 - position.x,
                                        window_position.x + position.x as i32,
                                    )
                                };
                                let (h, y) = if window_size.height as f64 - position.y
                                    < MIN_HEIGHT as f64 * window.scale_factor()
                                {
                                    let diff = window_size.height
                                        - (MIN_HEIGHT as f64 * window.scale_factor()) as u32;
                                    (
                                        MIN_HEIGHT as f64 * window.scale_factor(),
                                        window_position.y + diff as i32,
                                    )
                                } else {
                                    (
                                        window_size.height as f64 - position.y,
                                        window_position.y + position.y as i32,
                                    )
                                };
                                window.set_inner_size(winit::dpi::PhysicalSize::new(w, h));
                                window.set_outer_position(winit::dpi::PhysicalPosition::new(x, y));
                            }
                            ResizeDirection::South => {
                                window.set_inner_size(winit::dpi::PhysicalSize::new(
                                    window_size.width as f64,
                                    f64::max(
                                        position.y as f64,
                                        MIN_HEIGHT as f64 * window.scale_factor(),
                                    ),
                                ));
                            }
                            ResizeDirection::SouthEast => {
                                window.set_inner_size(winit::dpi::PhysicalSize::new(
                                    f64::max(
                                        position.x as f64,
                                        MIN_WIDTH as f64 * window.scale_factor(),
                                    ),
                                    f64::max(
                                        position.y as f64,
                                        MIN_HEIGHT as f64 * window.scale_factor(),
                                    ),
                                ));
                            }
                            ResizeDirection::SouthWest => {
                                let (w, x) = if window_size.width as f64 - position.x
                                    < MIN_WIDTH as f64 * window.scale_factor()
                                {
                                    let diff = window_size.width
                                        - (MIN_WIDTH as f64 * window.scale_factor()) as u32;
                                    (
                                        MIN_WIDTH as f64 * window.scale_factor(),
                                        window_position.x + diff as i32,
                                    )
                                } else {
                                    (
                                        window_size.width as f64 - position.x,
                                        window_position.x + position.x as i32,
                                    )
                                };
                                window.set_inner_size(winit::dpi::PhysicalSize::new(
                                    w,
                                    f64::max(
                                        position.y as f64,
                                        MIN_HEIGHT as f64 * window.scale_factor(),
                                    ),
                                ));
                                window.set_outer_position(winit::dpi::PhysicalPosition::new(
                                    x,
                                    window_position.y,
                                ));
                            }
                            ResizeDirection::West => {
                                let (w, x) = if window_size.width as f64 - position.x
                                    < MIN_WIDTH as f64 * window.scale_factor()
                                {
                                    let diff = window_size.width
                                        - (MIN_WIDTH as f64 * window.scale_factor()) as u32;
                                    (
                                        MIN_WIDTH as f64 * window.scale_factor(),
                                        window_position.x + diff as i32,
                                    )
                                } else {
                                    (
                                        window_size.width as f64 - position.x,
                                        window_position.x + position.x as i32,
                                    )
                                };
                                window.set_inner_size(winit::dpi::PhysicalSize::new(
                                    w,
                                    window_size.height as f64,
                                ));
                                window.set_outer_position(winit::dpi::PhysicalPosition::new(
                                    x,
                                    window_position.y,
                                ));
                            }
                        }

                        window.request_redraw();
                    } else {
                        if true {
                            let new_location =
                                cursor_resize_direction(window.inner_size(), *position, BORDER);

                            if new_location != state.ui.cursor_location {
                                state.ui.cursor_location = new_location;
                                // window.set_cursor_icon(cursor_direction_icon(state.ui.cursor_location))
                                state.ui.cursor_icon =
                                    cursor_direction_icon(state.ui.cursor_location);
                            }
                        }
                    }
                }
            }

            // Start drag to resize custom event
            winit::event::WindowEvent::MouseInput {
                state: winit::event::ElementState::Pressed,
                button: winit::event::MouseButton::Left,
                ..
            } => {
                if let Some(_dir) = state.ui.cursor_location {
                    state.ui.resizing = true;
                }
            }

            // Stop drag to resize custom event
            winit::event::WindowEvent::MouseInput {
                state: winit::event::ElementState::Released,
                button: winit::event::MouseButton::Left,
                ..
            } => {
                if state.ui.resizing {
                    state.ui.resizing = false;
                }
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

// Minimize the window
pub fn minimize(window: &Window) {
    window.set_minimized(true);
}

// Maximize the window
pub fn maximize(window: &Window) {
    window.set_maximized(true);
}

// Un-Maximize the window
pub fn unmaximize(window: &Window) {
    window.set_maximized(false);
}

// Open the window
pub fn open(window: &Window) {
    window.set_visible(true);
    window.set_minimized(false);
    window.focus_window();
}

// Clean up on app exit
pub fn on_exit(window: &Window) {
    window.set_visible(false);
}

fn cursor_direction_icon(resize_direction: Option<ResizeDirection>) -> egui::CursorIcon {
    match resize_direction {
        Some(resize_direction) => match resize_direction {
            ResizeDirection::East => egui::CursorIcon::ResizeHorizontal,
            ResizeDirection::North => egui::CursorIcon::ResizeVertical,
            ResizeDirection::NorthEast => egui::CursorIcon::ResizeNeSw,
            ResizeDirection::NorthWest => egui::CursorIcon::ResizeNwSe,
            ResizeDirection::South => egui::CursorIcon::ResizeVertical,
            ResizeDirection::SouthEast => egui::CursorIcon::ResizeNwSe,
            ResizeDirection::SouthWest => egui::CursorIcon::ResizeNeSw,
            ResizeDirection::West => egui::CursorIcon::ResizeHorizontal,
        },
        None => egui::CursorIcon::Default,
    }
}

fn cursor_resize_direction(
    win_size: winit::dpi::PhysicalSize<u32>,
    position: winit::dpi::PhysicalPosition<f64>,
    border_size: f64,
) -> Option<ResizeDirection> {
    enum XDirection {
        West,
        East,
        Default,
    }

    enum YDirection {
        North,
        South,
        Default,
    }

    let xdir = if position.x < border_size {
        XDirection::West
    } else if position.x > (win_size.width as f64 - border_size) {
        XDirection::East
    } else {
        XDirection::Default
    };

    let ydir = if position.y < border_size {
        YDirection::North
    } else if position.y > (win_size.height as f64 - border_size) {
        YDirection::South
    } else {
        YDirection::Default
    };

    Some(match xdir {
        XDirection::West => match ydir {
            YDirection::North => ResizeDirection::NorthWest,
            YDirection::South => ResizeDirection::SouthWest,
            YDirection::Default => ResizeDirection::West,
        },

        XDirection::East => match ydir {
            YDirection::North => ResizeDirection::NorthEast,
            YDirection::South => ResizeDirection::SouthEast,
            YDirection::Default => ResizeDirection::East,
        },

        XDirection::Default => match ydir {
            YDirection::North => ResizeDirection::North,
            YDirection::South => ResizeDirection::South,
            YDirection::Default => return None,
        },
    })
}
