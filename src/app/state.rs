//
// Define all state object properties
//
pub struct State {
    // Action flags, can be set to true by any consumer of the state object,
    // and will be actioned by the apply fn in app.rs
    pub actions: Actions,
    //
    // Other stuff coming soon
    //
}

pub struct Actions {
    pub app_exit: bool,
    pub window_close: bool,
    pub window_open: bool,
}

//
// Create state object, & initialize with defaults
//
pub fn init() -> State {
    let actions = Actions {
        app_exit: false,
        window_close: false,
        window_open: false,
    };

    State { actions }
}
