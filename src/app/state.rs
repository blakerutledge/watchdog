pub struct State {
    pub age: u32,
    pub name: String,
    pub action_exit: bool,
    pub action_window_close: bool,
    pub action_window_open: bool,
}

pub fn init() -> State {
    State {
        name: String::from("Blake"),
        age: 31,
        action_exit: false,
        action_window_close: false,
        action_window_open: false,
    }
}
