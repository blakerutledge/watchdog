use super::perf::Frame;
use std::collections::HashMap;
use std::collections::VecDeque;
use std::time::Duration;
use winit::window::ResizeDirection;

//
// Define all state object properties, nested into categories
//
pub struct State {
    // Action flags, can be set to true by any consumer of the state object,
    // and will be actioned by the apply fn in app.rs
    pub actions: Actions,

    // Stats for host Window rendering
    pub perf: Perf,

    // Ui
    pub ui: UiState,

    // Watched App heartbeats

    // Config status
    pub json: Json,
}

pub struct Actions {
    pub app_exit: bool,
    pub window_close: bool,
    pub window_open: bool,
    pub window_minimize: bool,
    pub window_maximize: bool,
    pub window_unmaximize: bool,
}

pub struct Perf {
    pub start_time: Duration,
    pub frames: VecDeque<Frame>,
    pub fps: u32,
    pub avg_frame_time: f32,
    pub monitor_refresh_rate: u32,
}

pub struct UiState {
    pub textures: HashMap<String, (egui::Vec2, egui::TextureHandle)>,
    pub custom_fonts: bool,
    pub active_tab: TabState,
    pub show_exit_tooltip: bool,
    pub overlay_exit: bool,
    pub title_bar_time_last_click: Duration,
    pub cursor_location: Option<ResizeDirection>,
    pub cursor_icon: egui::CursorIcon,
    pub resizing: bool,
    pub resize_start_mouse: (u8, u8),
    pub resize_start_corner: (u8, u8),
}

pub struct Json {
    pub exists: bool,
    pub parsed: bool,
    pub dirty: bool,
    pub filepath: Option<String>,
}

pub enum TabState {
    Config,
    Play,
    Stats,
}

//
// Create state object, & initialize with defaults
//
pub fn init() -> State {
    //
    // all action flags are initialized as false
    let actions = Actions {
        app_exit: false,
        window_close: false,
        window_open: false,
        window_minimize: false,
        window_maximize: false,
        window_unmaximize: false,
    };

    let perf = Perf {
        start_time: Duration::from_secs(0),
        fps: 0 as u32,
        frames: VecDeque::new(),
        avg_frame_time: 0 as f32,
        monitor_refresh_rate: 60,
    };

    let json = Json {
        exists: false,
        parsed: false,
        dirty: false,
        filepath: None,
    };

    let ui = UiState {
        textures: HashMap::new(),
        custom_fonts: false,
        // tab_config: true,
        // tab_play: false,
        // tab_stats: false,
        active_tab: TabState::Config,
        show_exit_tooltip: false,
        overlay_exit: false,
        title_bar_time_last_click: Duration::new(0, 0),
        cursor_location: None,
        cursor_icon: egui::CursorIcon::Default,
        resizing: false,
        resize_start_mouse: (0, 0),
        resize_start_corner: (0, 0),
    };

    State {
        actions,
        perf,
        ui,
        json,
    }
}
