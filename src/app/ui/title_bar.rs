use crate::app::ui::*;

use std::time::Duration;

const ROW_HEIGHT: f32 = 40.0;
const WINDOW_BUTTON_WIDTH: f32 = 48.0;
const WINDOW_BUTTON_HEIGHT: f32 = 40.0;

pub fn draw(context: &egui::Context, state: &mut State, window: &winit::window::Window) {
    egui::TopBottomPanel::top("title_bar")
        .exact_height(ROW_HEIGHT)
        .frame(egui::Frame::none().fill(COLOR_MED_GREY))
        .resizable(false)
        .show_separator_line(false)
        .show(context, |ui| {
            ui.with_layout(egui::Layout::left_to_right(egui::Align::Center), |ui| {
                let theme = ui.visuals_mut();

                theme.widgets.noninteractive.bg_stroke = egui::Stroke {
                    width: 0.0,
                    color: COLOR_TRANSPARENT,
                };

                // baseline
                theme.widgets.inactive.weak_bg_fill = COLOR_TRANSPARENT;

                // clicking
                theme.widgets.active.weak_bg_fill = COLOR_DARK_GREY;
                theme.widgets.active.bg_stroke = egui::Stroke {
                    width: 0.0,
                    color: COLOR_TRANSPARENT,
                };
                theme.widgets.active.rounding = egui::Rounding {
                    nw: 0.0,
                    sw: 0.0,
                    se: 0.0,
                    ne: 0.0,
                };

                // hovering
                theme.widgets.hovered.weak_bg_fill = egui::Color32::from_rgb(29, 29, 29);
                theme.widgets.hovered.bg_stroke = egui::Stroke {
                    width: 0.0,
                    color: COLOR_TRANSPARENT,
                };
                theme.widgets.hovered.rounding = egui::Rounding {
                    nw: 0.0,
                    sw: 0.0,
                    se: 0.0,
                    ne: 0.0,
                };

                // disabled
                theme.widgets.noninteractive.weak_bg_fill = COLOR_TRANSPARENT;

                let style = ui.style_mut();
                style.spacing.button_padding = egui::Vec2::new(0.0, 0.0);
                style.spacing.window_margin = egui::Margin {
                    left: 0.0,
                    right: 0.0,
                    top: 0.0,
                    bottom: 0.0,
                };
                style.spacing.item_spacing = egui::Vec2::new(0.0, 0.0);
                let logo = state.ui.textures.get("icon_logo").unwrap();
                let logo_size = egui::Vec2::new(25.0, 25.0);

                let icon_min = state.ui.textures.get("icon_min").unwrap();
                let icon_max = state.ui.textures.get("icon_max").unwrap();
                let icon_unmax = state.ui.textures.get("icon_unmax").unwrap();
                let icon_close = state.ui.textures.get("icon_close").unwrap();

                let window_button_dims = Vec2::new(WINDOW_BUTTON_WIDTH, WINDOW_BUTTON_HEIGHT);

                // Titlebar - All elements besides Window Buttons
                let group = ui.group(|ui| {
                    ui.add_space(12.0); // enough space left of logo to appear centered with separate nav_bar elements
                    ui.image(&logo.1, logo_size);
                    ui.add_space(20.0); // add space so text aligns with left end of central panel
                    ui.label(
                        egui::RichText::new("Watchdog")
                            .text_style(egui::TextStyle::Name("Title".into()))
                            .color(COLOR_TEXT_WHITE),
                    );
                    ui.add_space(ui.available_width() - window_button_dims.x * 3.0);
                });

                // Title bar draggable to move window
                let r = ui.interact(
                    group.response.rect,
                    egui::Id::new("title_bar_content"),
                    egui::Sense::click_and_drag(),
                );
                if r.drag_started() {
                    window.drag_window().unwrap();
                }

                // Title bar double clickable to max/unmax
                // NOTE: something is currently not working with egui doubleclick event dispatch,
                // i can double click once and it responds but then never aganin
                if r.clicked() {
                    let n = utils::now();
                    let diff = n.checked_sub(state.ui.title_bar_time_last_click);
                    match diff {
                        Some(diff) => {
                            if diff.as_millis() < 500 {
                                state.ui.title_bar_time_last_click = Duration::new(0, 0);

                                if window.is_maximized() {
                                    state.actions.window_unmaximize = true;
                                } else {
                                    state.actions.window_maximize = true;
                                }
                            } else {
                                state.ui.title_bar_time_last_click = n;
                            }
                        }
                        _ => {
                            state.ui.title_bar_time_last_click = n;
                        }
                    };
                }

                // Titlebar -- Window Buttons

                // Window Minimize
                let r = ui.add(egui::ImageButton::new(&icon_min.1, window_button_dims));
                if r.clicked() {
                    state.actions.window_minimize = true;
                }

                // Window Maximize / Un-Maximize
                let r = ui.add(egui::ImageButton::new(
                    if window.is_maximized() {
                        &icon_unmax.1
                    } else {
                        &icon_max.1
                    },
                    window_button_dims,
                ));
                if r.clicked() {
                    if window.is_maximized() {
                        state.actions.window_unmaximize = true;
                    } else {
                        state.actions.window_maximize = true;
                    }
                }

                // Set background of image button to red
                let theme = ui.visuals_mut();
                theme.widgets.hovered.weak_bg_fill = COLOR_RED;
                theme.widgets.active.weak_bg_fill = COLOR_DARK_RED;

                // Window Close
                let r = ui.add(egui::ImageButton::new(&icon_close.1, window_button_dims));
                if r.clicked() {
                    state.actions.window_close = true;
                }
            });
        });
}
