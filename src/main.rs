// HIDE console window on Windows in release
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
// WARN about clippy things and rusty things
#![warn(clippy::all, rust_2018_idioms)]

mod app;
mod utils;

fn main() {
    app::init();
}
