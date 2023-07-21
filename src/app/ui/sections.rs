use crate::app::ui::*;

//
// Global styles for all sections that use the central panel
//
pub fn style(ui: &mut egui::Ui) {
    let style = ui.style_mut();

    style.spacing.button_padding = Vec2::new(10.0, 10.0);
    style.spacing.scroll_bar_width = 7.0;
    style.spacing.scroll_bar_inner_margin = 2.0;
    style.spacing.scroll_bar_outer_margin = 0.0;
    style.spacing.item_spacing = Vec2::new(0.0, 0.0);
    let visuals = ui.visuals_mut();
    visuals.extreme_bg_color = COLOR_DARKER_GREY;
    visuals.selection.bg_fill = Color32::from_rgba_unmultiplied(255, 255, 255, 20); // non-const call

    visuals.selection.stroke = egui::Stroke {
        width: 1.0,
        color: COLOR_YELLOW,
    };
}

//
// Nested containers for proper layout of all sections, as well as
// state dependent draw calls to each section
//
pub fn draw(context: &egui::Context, state: &mut State, config: &mut Config) {
    // Draw main content area
    egui::CentralPanel::default()
        // Create a medium gray background color for whole frame,
        // same color as Nav bar and Title bar
        .frame(egui::Frame::none().fill(COLOR_MED_GREY))
        .show(context, |ui| {
            //
            // Create another frame, with darker background, that has the
            // top left corner rounded. Note if we didnt have the first background,
            // the space that is clipped by the corner radius would be transparent ha
            egui::Frame::none()
                .fill(COLOR_DARK_GREY)
                // add some margin for the scroll bar to look nice
                .inner_margin(egui::Margin {
                    left: 2.0,
                    top: 2.0,
                    bottom: 2.0,
                    right: 2.0,
                })
                // apply the rounding of top left corner only
                .rounding(egui::Rounding {
                    nw: 8.0,
                    sw: 0.0,
                    se: 0.0,
                    ne: 0.0,
                })
                .show(ui, |ui| {
                    //
                    // Set custom styles, shared across all sections
                    style(ui);

                    // Create a vertically scrolling area, that fills the window
                    egui::ScrollArea::vertical()
                        .auto_shrink([false, false])
                        .scroll_bar_visibility(
                            egui::scroll_area::ScrollBarVisibility::AlwaysVisible,
                        )
                        .show(ui, |ui| {
                            egui::Frame::none()
                                // Now we have the proper nested containers in place
                                // Define main content area outer margin here, and each section
                                // can just take up all available space
                                .outer_margin(egui::Margin {
                                    left: 20.0,
                                    top: 28.0,
                                    bottom: 20.0,
                                    right: 20.0,
                                })
                                .show(ui, |ui| {
                                    //
                                    // Draw one section at a time depending on which is selected
                                    match state.ui.active_tab {
                                        TabState::Config => {
                                            // Draw the Config section
                                            section_config::draw(ui, state, config)
                                        }
                                        TabState::Apps => {
                                            // Draw the Apps section
                                            section_apps::draw(ui, state, config)
                                        }
                                        TabState::Stats => {
                                            // Draw the Stats section
                                            section_stats::draw(ui, state, config)
                                        }
                                    }

                                    // Just make some more space at the bottom, so error messages are not clipped
                                    components::draw_separator(ui);
                                });
                        });
                });
        });
}
