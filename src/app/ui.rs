use egui::Vec2;

use crate::app::config::Config;
use crate::app::state::{State, TabState};
use crate::utils;

// Import submodules
mod components;
mod exit;
mod icons;
mod nav_bar;
mod perf;
mod resize_borders;
mod section_apps;
mod section_config;
mod section_stats;
mod sections;
mod title_bar;
mod typography;

// TEMP perf overlay
const SHOW_HOST_DEV: bool = false;

// GLOBAL constants across submodules
pub const ROW_LABEL_WIDTH: f32 = 200.0;
pub const ROW_HEIGHT: f32 = 36.0;
pub const ROW_MARGIN: f32 = 6.0;

pub const SECTION_HEADING_MARGIN: f32 = 12.0;

pub const COLOR_TEXT_WHITE: egui::Color32 = egui::Color32::from_rgb(238, 238, 238);
pub const COLOR_MED_GREY: egui::Color32 = egui::Color32::from_rgb(40, 40, 40);
pub const COLOR_DARK_GREY: egui::Color32 = egui::Color32::from_rgb(29, 29, 29);
pub const COLOR_DARKER_GREY: egui::Color32 = egui::Color32::from_rgb(22, 22, 22);
pub const COLOR_TRANSPARENT: egui::Color32 = egui::Color32::TRANSPARENT;
pub const COLOR_DARK_RED: egui::Color32 = egui::Color32::from_rgb(238, 58, 23);
pub const COLOR_RED: egui::Color32 = egui::Color32::from_rgb(251, 81, 48);

// Init is called once, and returns the reference to the closure that is the primary draw function,
// that will be called from the primary event loop, and renderer module
pub fn init() -> Box<dyn FnMut(&egui::Context, &mut State, &mut Config, &winit::window::Window)> {
    Box::new(
        |context: &egui::Context,
         state: &mut State,
         config: &mut Config,
         window: &winit::window::Window| {
            // Ensure all icon image files are loaded
            // Called every frame, but guarded with boolean flags so
            // that the actual work is only done once
            icons::load(state, context);

            // Ensure all font files are loaded, and custom global type styles defined
            // Called every frame, but guarded with boolean flags so
            // that the actual work is only done once
            typography::load(state, context);

            // Draw special border around window and listen for interaction to resize the window
            // Do not draw these borders if the window is maximized
            if !window.is_maximized() {
                resize_borders::draw(context, state);
            }

            // Draw the window Title bar area
            title_bar::draw(context, state, window);

            // Draw the tool tip to allow various behaviors when exiting the app
            exit::draw(context, state);

            // Draw the left side vertical navigation bar, with tabs for each section of the app
            nav_bar::draw(context, state);

            // Draw performance debug overlay
            perf::draw(context, state);

            // Draw the main content
            sections::draw(context, state, config);
        },
    )
}
