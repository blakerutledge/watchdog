use crate::app::ui::*;

pub fn draw_separator(ui: &mut egui::Ui) {
    ui.add_space(8.0);
    ui.horizontal(|ui| {
        let (rect, _resp) = ui.allocate_exact_size(
            egui::Vec2::new(ui.available_width(), 1.0),
            egui::Sense::hover(),
        );
        ui.painter_at(rect).rect_filled(rect, 0.0, COLOR_LIGHT_GREY);
    });
    ui.add_space(14.0);
}

//
// Helper for drawing a standard label and text entry field
//
pub fn draw_row(
    ui: &mut egui::Ui,
    state: &mut State,
    label: &str,
    prop: &mut crate::app::config::ConfigData,
) {
    ui.horizontal(|ui| {
        // Override some styles if the element is invalid
        if !prop.valid {
            let visuals = ui.visuals_mut();
            visuals.selection.stroke = egui::Stroke {
                width: 1.0,
                color: COLOR_RED,
            };
            visuals.widgets.hovered.bg_stroke = egui::Stroke {
                width: 1.0,
                color: COLOR_RED,
            };
        }

        //
        // Label with minimum width
        //
        ui.allocate_ui_with_layout(
            egui::Vec2 {
                x: ROW_LABEL_WIDTH,
                y: ROW_HEIGHT,
            },
            egui::Layout {
                main_dir: egui::Direction::LeftToRight,
                main_wrap: false,
                main_align: egui::Align::LEFT,
                main_justify: true,
                cross_align: egui::Align::Center,
                cross_justify: true,
            },
            |ui| {
                //
                // Draw the label
                ui.label(egui::RichText::new(label).color(COLOR_OFFWHITE));

                // Add margin between label and text entry field
                ui.add_space(ROW_GUTTER_SPACE);
            },
        );

        //
        // Text entry field takes rest of available space
        //
        let r = ui.allocate_ui_with_layout(
            egui::Vec2 {
                x: ui.available_width(),
                y: ROW_HEIGHT,
            },
            egui::Layout {
                main_dir: egui::Direction::LeftToRight,
                main_wrap: false,
                main_align: egui::Align::LEFT,
                main_justify: true,
                cross_align: egui::Align::Center,
                cross_justify: true,
            },
            |ui| {
                //
                // Draw the text entry field
                let text_edit = egui::TextEdit::singleline(&mut prop.str)
                    .margin(egui::Vec2::new(16.0, 0.0))
                    .text_color(if prop.valid {
                        COLOR_TEXT_WHITE
                    } else {
                        COLOR_RED
                    })
                    .vertical_align(egui::Align::Center)
                    .interactive(true)
                    .frame(true)
                    .cursor_at_end(true);

                // Update the state to act on any changes this frame
                let r = ui.add(text_edit);
                if r.changed() {
                    state.actions.config_edited = true;
                    prop.dirty = true;
                };
            },
        );

        if r.response.hovered() && !prop.error.is_empty() {
            let mut anchor = r.response.rect.left_bottom();
            anchor.y += 1.0;

            let a = egui::Area::new("error_tooltip")
                .movable(false)
                .fixed_pos(anchor)
                .pivot(egui::Align2::LEFT_TOP)
                .interactable(false);

            a.show(ui.ctx(), |ui: &mut egui::Ui| {
                egui::Frame::none()
                    .outer_margin(egui::Margin {
                        left: 16.0,
                        right: 30.0 + 16.0,
                        top: 0.0,
                        bottom: 0.0,
                    })
                    .inner_margin(egui::Margin {
                        left: 18.0,
                        right: 18.0,
                        top: 14.0,
                        bottom: 14.0,
                    })
                    .fill(COLOR_DARKER_GREY)
                    .stroke(egui::Stroke {
                        color: COLOR_RED,
                        width: 1.0,
                    })
                    .rounding(egui::Rounding {
                        nw: 0.0,
                        ne: 0.0,
                        se: 4.0,
                        sw: 4.0,
                    })
                    .show(ui, |ui| {
                        ui.with_layout(
                            egui::Layout {
                                main_dir: egui::Direction::LeftToRight,
                                main_wrap: false,
                                main_align: egui::Align::LEFT,
                                main_justify: true,
                                cross_align: egui::Align::TOP,
                                cross_justify: false,
                            },
                            |ui| {
                                ui.add(
                                    egui::Label::new(
                                        egui::RichText::new(&prop.error).color(COLOR_RED), // .background_color(COLOR_MED_GREY),
                                    )
                                    .wrap(true),
                                )
                            },
                        )
                    });
            });
        }
    });

    // Add space below the row
    ui.add_space(ROW_MARGIN);
}

//
// Helper for drawing a standard label and text entry field
//
pub fn draw_row_basic(ui: &mut egui::Ui, label: &str, prop: &mut String) {
    ui.horizontal(|ui| {
        //
        // Label with minimum width
        //
        ui.allocate_ui_with_layout(
            egui::Vec2 {
                x: ROW_LABEL_WIDTH,
                y: ROW_HEIGHT,
            },
            egui::Layout {
                main_dir: egui::Direction::LeftToRight,
                main_wrap: false,
                main_align: egui::Align::LEFT,
                main_justify: true,
                cross_align: egui::Align::Center,
                cross_justify: true,
            },
            |ui| {
                //
                // Draw the label
                ui.label(egui::RichText::new(label).color(COLOR_OFFWHITE));

                // Add margin between label and text entry field
                ui.add_space(ROW_GUTTER_SPACE);
            },
        );

        //
        // Text entry field takes rest of available space
        //
        ui.allocate_ui_with_layout(
            egui::Vec2 {
                x: ui.available_width(),
                y: ROW_HEIGHT,
            },
            egui::Layout {
                main_dir: egui::Direction::LeftToRight,
                main_wrap: false,
                main_align: egui::Align::LEFT,
                main_justify: true,
                cross_align: egui::Align::Center,
                cross_justify: true,
            },
            |ui| {
                //
                // Draw the text entry field
                let text_edit = egui::TextEdit::singleline(prop)
                    .margin(egui::Vec2::new(16.0, 0.0))
                    .text_color(COLOR_TEXT_WHITE)
                    .vertical_align(egui::Align::Center)
                    .interactive(true)
                    .frame(false);

                // Update the state to act on any changes this frame
                ui.add(text_edit);
            },
        );
    });

    // Add space below the row
    ui.add_space(ROW_MARGIN);
}

// Draw row for password entry
//
// Helper for drawing a standard label and text entry field
//
pub fn draw_row_password(
    ui: &mut egui::Ui,
    state: &mut State,
    label: &str,
    prop: &mut crate::app::config::ConfigData,
) {
    ui.horizontal(|ui| {
        // Override some styles if the element is invalid
        if !prop.valid {
            let visuals = ui.visuals_mut();
            visuals.selection.stroke = egui::Stroke {
                width: 1.0,
                color: COLOR_RED,
            };
            visuals.widgets.hovered.bg_stroke = egui::Stroke {
                width: 1.0,
                color: COLOR_RED,
            };
        }

        //
        // Label with minimum width
        //
        ui.allocate_ui_with_layout(
            egui::Vec2 {
                x: ROW_LABEL_WIDTH,
                y: ROW_HEIGHT,
            },
            egui::Layout {
                main_dir: egui::Direction::LeftToRight,
                main_wrap: false,
                main_align: egui::Align::LEFT,
                main_justify: true,
                cross_align: egui::Align::Center,
                cross_justify: true,
            },
            |ui| {
                //
                // Draw the label
                ui.label(egui::RichText::new(label).color(COLOR_OFFWHITE));

                // Add margin between label and text entry field
                ui.add_space(ROW_GUTTER_SPACE);
            },
        );

        //
        // Text entry field takes rest of available space
        //
        let r = ui.allocate_ui_with_layout(
            egui::Vec2 {
                x: ui.available_width(),
                y: ROW_HEIGHT,
            },
            egui::Layout {
                main_dir: egui::Direction::LeftToRight,
                main_wrap: false,
                main_align: egui::Align::LEFT,
                main_justify: true,
                cross_align: egui::Align::Center,
                cross_justify: true,
            },
            |ui| {
                //
                // Draw the text entry field
                let text_edit = egui::TextEdit::singleline(&mut prop.str)
                    .margin(egui::Vec2::new(16.0, 0.0))
                    .text_color(if prop.valid {
                        COLOR_TEXT_WHITE
                    } else {
                        COLOR_RED
                    })
                    .vertical_align(egui::Align::Center)
                    .interactive(true)
                    .frame(true)
                    .password(true);

                // Update the state to act on any changes this frame
                let r = ui.add(text_edit);
                if r.changed() {
                    state.actions.config_edited = true;
                    prop.dirty = true;
                };
            },
        );

        if r.response.hovered() && !prop.error.is_empty() {
            let mut anchor = r.response.rect.left_bottom();
            anchor.y += 1.0;

            let a = egui::Area::new("error_tooltip")
                .movable(false)
                .fixed_pos(anchor)
                .pivot(egui::Align2::LEFT_TOP)
                .interactable(false);

            a.show(ui.ctx(), |ui: &mut egui::Ui| {
                egui::Frame::none()
                    .outer_margin(egui::Margin {
                        left: 16.0,
                        right: 30.0 + 16.0,
                        top: 0.0,
                        bottom: 0.0,
                    })
                    .inner_margin(egui::Margin {
                        left: 18.0,
                        right: 18.0,
                        top: 14.0,
                        bottom: 14.0,
                    })
                    .fill(COLOR_DARKER_GREY)
                    .stroke(egui::Stroke {
                        color: COLOR_RED,
                        width: 1.0,
                    })
                    .rounding(egui::Rounding {
                        nw: 0.0,
                        ne: 0.0,
                        se: 4.0,
                        sw: 4.0,
                    })
                    .show(ui, |ui| {
                        ui.set_row_height(18.0);
                        ui.with_layout(
                            egui::Layout {
                                main_dir: egui::Direction::LeftToRight,
                                main_wrap: false,
                                main_align: egui::Align::LEFT,
                                main_justify: true,
                                cross_align: egui::Align::TOP,
                                cross_justify: false,
                            },
                            |ui| {
                                ui.add(
                                    egui::Label::new(
                                        egui::RichText::new(&prop.error).color(COLOR_RED), // .background_color(COLOR_MED_GREY),
                                    )
                                    .wrap(true),
                                )
                            },
                        )
                    });
            });
        }
    });

    // Add space below the row
    ui.add_space(ROW_MARGIN);
}

pub fn format_ms(f: f32) -> String {
    format!("{:06.3}", f)
}

pub fn format_imagebuttons(ui: &mut egui::Ui) {
    let theme = ui.visuals_mut();

    // baseline
    theme.widgets.inactive.weak_bg_fill = COLOR_DARKER_GREY;

    // clicking
    theme.widgets.active.weak_bg_fill = COLOR_YELLOW;
    theme.widgets.active.bg_stroke = egui::Stroke {
        width: 0.0,
        color: COLOR_TRANSPARENT,
    };
    theme.widgets.active.rounding = egui::Rounding {
        nw: 4.0,
        sw: 4.0,
        se: 4.0,
        ne: 4.0,
    };

    // hovering
    theme.widgets.hovered.weak_bg_fill = COLOR_DARKER_GREY;
    theme.widgets.hovered.bg_stroke = egui::Stroke {
        width: 1.0,
        color: COLOR_OFFWHITE,
    };
    theme.widgets.hovered.rounding = egui::Rounding {
        nw: 4.0,
        sw: 4.0,
        se: 4.0,
        ne: 4.0,
    };

    // disabled
    theme.widgets.noninteractive.weak_bg_fill = COLOR_TRANSPARENT;

    let style = ui.style_mut();

    style.spacing.button_padding = Vec2::new(0.0, 0.0);
    style.spacing.item_spacing = Vec2::new(0.0, 0.0);
}
