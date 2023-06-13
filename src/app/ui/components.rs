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
    let r = ui.horizontal(|ui| {
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
                    .frame(true);
                // .text_color(text_color)

                // Update the state to act on any changes this frame
                let r = ui.add(text_edit);
                if r.changed() {
                    state.actions.config_edited = true;
                    prop.dirty = true;
                };
            },
        );

        if r.response.hovered() && !prop.error.is_empty() {
            // r.response.rect
            let a = egui::Area::new("error_tooltip")
                .movable(false)
                // .anchor(egui::Align2::RIGHT_CENTER, Vec2::new(0.0, 0.0))
                .fixed_pos(r.response.rect.left_bottom())
                // .constrain(constrain)
                .pivot(egui::Align2::LEFT_TOP)
                .interactable(false);

            a.show(ui.ctx(), |ui: &mut egui::Ui| {
                ui.add_space(ROW_MARGIN);

                let (rect, _response) = ui.allocate_exact_size(
                    Vec2::new(r.response.rect.width(), r.response.rect.height()),
                    egui::Sense::hover(),
                );

                ui.painter_at(rect)
                    .rect_filled(rect, 4.0, COLOR_DARKER_GREY);

                ui.put(rect, |ui: &mut egui::Ui| {
                    ui.horizontal(|ui| {
                        ui.add_space(16.0); // same as indent for text input
                        ui.label(egui::RichText::new(&prop.error).color(COLOR_RED));
                        //  TO DO add stroke as well ?
                        // TO DO make text wrap and expand
                        // TO DO everything was red on launch?
                    })
                    .response
                });
            });
            // a.
        }
    });

    // Add space below the row
    ui.add_space(ROW_MARGIN);
}

//
// Helper for drawing a standard label and text entry field
//
pub fn draw_row_non_interactive(
    ui: &mut egui::Ui,
    state: &mut State,
    label: &str,
    prop: &mut String,
) {
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
                    .frame(true);

                // Update the state to act on any changes this frame
                let r = ui.add(text_edit);
                if r.changed() {
                    state.actions.config_edited = true
                };
            },
        );
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
