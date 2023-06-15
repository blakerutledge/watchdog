use crate::app::state::State;

use crate::app::ui::*;

pub fn draw(context: &egui::Context, state: &mut State, mut anchor: egui::Pos2) {
    // Set to false at the beginning of every frame,
    // See below useage on clicked_elsewhere for explanation
    state.ui.exit_tooltip_clickout = false;

    if state.ui.show_exit_tooltip {
        // Account for nav bar padding, anchor flush with bottom of window and nav bar
        anchor.x += 9.0;
        anchor.y += 13.0;

        let a = egui::Area::new("Exit")
            .order(egui::Order::Foreground)
            .movable(false)
            .pivot(egui::Align2::LEFT_BOTTOM)
            .fixed_pos(anchor);

        let r = a
            .show(context, |ui| {
                egui::Frame::none()
                    .inner_margin(egui::Margin {
                        left: 22.0,
                        top: 34.0,
                        right: 12.0,
                        bottom: 28.0,
                    })
                    .fill(COLOR_MED_GREY)
                    .rounding(egui::Rounding {
                        ne: 8.0,
                        se: 0.0,
                        sw: 0.0,
                        nw: 0.0,
                    })
                    .show(ui, |ui| {
                        ui.with_layout(
                            egui::Layout {
                                main_dir: egui::Direction::TopDown,
                                main_wrap: false,
                                main_align: egui::Align::LEFT,
                                main_justify: false,
                                cross_align: egui::Align::LEFT,
                                cross_justify: false,
                            },
                            |ui| {
                                // ui.add(

                                // Remove border of group element
                                ui.visuals_mut().widgets.noninteractive.bg_stroke = egui::Stroke {
                                    color: COLOR_TRANSPARENT,
                                    width: 0.0,
                                };

                                ui.horizontal_wrapped(|ui| {
                                    //
                                    // Close the watchdog window, leave it running in the background
                                    let r = draw_text_button(
                                        ui,
                                        "Close",
                                        "Watchdog will run in the background",
                                    );
                                    if r.clicked() {
                                        // Hide the exit tooltip
                                        state.ui.show_exit_tooltip = false;
                                        // Close the window
                                        state.actions.window_close = true;
                                    }
                                    r.on_hover_cursor(egui::CursorIcon::PointingHand);

                                    // Quit, but leave any watched apps running
                                    // let r = ui.button("Quit Watchdog");
                                    let r = draw_text_button(
                                        ui,
                                        "Exit & Abandon",
                                        "Exit, and leave Watched Apps running",
                                    );
                                    if r.clicked() {
                                        // Hide the exit tooltip
                                        state.ui.show_exit_tooltip = false;
                                        // Exit watchdog
                                        state.actions.app_exit = true;
                                    }
                                    r.on_hover_cursor(egui::CursorIcon::PointingHand);

                                    // Quit and Kill
                                    let r = draw_text_button(
                                        ui,
                                        "Exit & Kill",
                                        "Exit, and kill Watched Apps",
                                    );
                                    if r.clicked() {
                                        //
                                        // TO DO: kill watched apps
                                        //

                                        // Hide the exit tooltip
                                        state.ui.show_exit_tooltip = false;
                                        // Exit watchdog
                                        state.actions.app_exit = true;
                                    }
                                    r.on_hover_cursor(egui::CursorIcon::PointingHand);
                                });
                            },
                        );
                    });
            })
            .response;

        // Clicked out, close the tooltip
        if state.ui.show_exit_tooltip && r.clicked_elsewhere() {
            // Use a state flag to mark the clickout event,
            // so that if you click on the Exit nav bar button while this
            // tool tip is open, it will close it.
            // Otherwise, it will close and re-open it in same frame.
            state.ui.exit_tooltip_clickout = true;
            // Hide the exit tool tip
            state.ui.show_exit_tooltip = false;
        }

        // Draw a fullscreen scrim
        let a = egui::Area::new("ExitScrim")
            .order(egui::Order::Middle)
            .movable(false)
            .pivot(egui::Align2::LEFT_TOP)
            .interactable(true)
            .fixed_pos(egui::Pos2::new(66.0, 42.0));

        a.show(context, |ui| {
            egui::Frame::none()
                .rounding(egui::Rounding {
                    nw: 8.0,
                    ne: 0.0,
                    se: 0.0,
                    sw: 0.0,
                })
                .outer_margin(egui::Margin {
                    top: 0.0,
                    left: 0.0,
                    right: 1.0,
                    bottom: 1.0,
                })
                .fill(egui::Color32::from_rgba_unmultiplied(18, 18, 18, 240))
                .show(ui, |ui| {
                    ui.with_layout(
                        egui::Layout {
                            main_dir: egui::Direction::TopDown,
                            main_wrap: false,
                            main_align: egui::Align::Center,
                            main_justify: true,
                            cross_align: egui::Align::Center,
                            cross_justify: true,
                        },
                        |ui| {
                            //  necessary?
                            ui.label("");
                        },
                    );
                });
        });
    };
}

fn draw_text_button(ui: &mut egui::Ui, header: &str, subheader: &str) -> egui::Response {
    let button_height = 62.0;
    let button_width = 160.0;

    let (rect, response) =
        ui.allocate_exact_size(Vec2::new(button_width, button_height), egui::Sense::click());

    let hover = response.hovered();

    let mut child_ui = ui.child_ui(
        rect,
        egui::Layout {
            main_dir: egui::Direction::TopDown,
            main_wrap: false,
            main_align: egui::Align::TOP,
            main_justify: false,
            cross_align: egui::Align::LEFT,
            cross_justify: false,
        },
    );

    // Label
    let _b = child_ui.label(
        egui::RichText::new((header).to_string())
            .text_style(egui::TextStyle::Name("TextButton".into()))
            .color(match hover {
                false => COLOR_OFFWHITE,
                true => COLOR_TEXT_WHITE,
            }),
    );

    child_ui.add_space(4.0);

    // Underline
    let (underline_rect, _resp) =
        child_ui.allocate_exact_size(Vec2::new(button_width, 1.0), egui::Sense::hover());

    child_ui.painter().rect(
        underline_rect,
        0.0,
        if hover {
            COLOR_YELLOW
        } else {
            COLOR_TRANSPARENT
        },
        egui::Stroke::new(0.0, COLOR_TRANSPARENT),
    );

    child_ui.add_space(8.0);

    ui.set_row_height(20.0);

    // Label
    let _b = child_ui.label(
        egui::RichText::new((subheader).to_string())
            .text_style(egui::TextStyle::Name("TextButtonSmall".into()))
            .color(match hover {
                false => COLOR_GRAY_TINT,
                true => COLOR_OFFWHITE,
            }),
    );

    ui.add_space(16.0);

    response
}
