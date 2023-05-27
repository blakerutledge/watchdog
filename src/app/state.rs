pub struct State {
    pub age: u32,
    pub name: String,
}

pub fn init() -> State {
    State {
        name: String::from("Blake"),
        age: 31,
    }
}
