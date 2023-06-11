use egui::Button;

use crate::app::config;
use crate::app::ui::*;
//
// Config section all ui elements
//

#[derive(PartialEq)]
enum ButtonState {
    Disabled,
    Normal,
    Selected,
    Error,
}

pub fn draw(ui: &mut egui::Ui, state: &mut State, config: &mut Config) {
    //
    // Header and JSON File Buttons
    ui.allocate_ui_with_layout(
        egui::Vec2 {
            x: ui.available_width(),
            y: ROW_HEIGHT,
        },
        egui::Layout {
            main_dir: egui::Direction::LeftToRight,
            main_wrap: false,
            main_align: egui::Align::LEFT,
            main_justify: false,
            cross_align: egui::Align::Center,
            cross_justify: false,
        },
        |ui| {
            //
            // Header
            ui.heading(egui::RichText::new("Config").color(COLOR_TEXT_WHITE));

            // Watched Apps Create / Delete Buttons
            ui.allocate_ui_with_layout(
                egui::Vec2 {
                    x: ui.available_width(),
                    y: ROW_HEIGHT,
                },
                egui::Layout {
                    main_dir: egui::Direction::RightToLeft,
                    main_wrap: false,
                    main_align: egui::Align::RIGHT,
                    main_justify: false,
                    cross_align: egui::Align::Center,
                    cross_justify: false,
                },
                |ui| {
                    components::format_imagebuttons(ui);

                    let icon_w = egui::Vec2::new(28.0, 28.0);

                    let icon_load = state.ui.textures.get("icon_load").unwrap();
                    let icon_save = state.ui.textures.get("icon_save").unwrap();
                    let icon_reset = state.ui.textures.get("icon_reset").unwrap();

                    // Reset Button UI
                    let r_reset = ui.add(egui::ImageButton::new(&icon_reset.1, icon_w));
                    ui.add_space(8.0);
                    // Save Button UI
                    let r_save = ui.add(egui::ImageButton::new(&icon_save.1, icon_w));
                    ui.add_space(8.0);
                    // Load Button UI
                    let r_load = ui.add(egui::ImageButton::new(&icon_load.1, icon_w));

                    // Interaction for Load Button
                    if r_load.clicked() {
                        if let Some(file) = rfd::FileDialog::new()
                            .add_filter("json", &["json"])
                            .pick_file()
                        {
                            config::replace_from_file(file, state, config);
                        }
                    }
                    r_load.on_hover_cursor(egui::CursorIcon::PointingHand);

                    // Interaction for Save Button
                    if r_save.clicked() {
                        if let Some(path) = rfd::FileDialog::new()
                            .add_filter("json", &["json"])
                            .save_file()
                        {
                            config::move_config(path, state, config);
                        }
                    }
                    r_save.on_hover_cursor(egui::CursorIcon::PointingHand);

                    // interaction for Reset Button
                    if r_reset.clicked() {
                        config::reinit_config(state, config);
                    }
                    r_reset.on_hover_cursor(egui::CursorIcon::PointingHand);
                },
            );
        },
    );

    ui.add_space(4.0);

    components::draw_row(
        ui,
        state,
        "JSON Filepath",
        &mut state.json.filepath.to_str().unwrap().to_string(),
        false,
    );

    ui.add_space(SECTION_HEADING_MARGIN);

    components::draw_separator(ui);

    //
    // Watched App - Index Selector
    //
    ui.horizontal(|ui| {
        let visuals = ui.visuals_mut();
        //
        // Clear button styles
        //

        // disabled
        visuals.widgets.noninteractive.weak_bg_fill = COLOR_TRANSPARENT;
        visuals.widgets.noninteractive.bg_stroke = egui::Stroke {
            width: 0.0,
            color: COLOR_TRANSPARENT,
        };

        // not selected
        visuals.widgets.inactive.weak_bg_fill = COLOR_TRANSPARENT;
        visuals.widgets.inactive.bg_stroke = egui::Stroke {
            width: 0.0,
            color: COLOR_TRANSPARENT,
        };

        // hovered
        visuals.widgets.hovered.weak_bg_fill = COLOR_TRANSPARENT;
        visuals.widgets.hovered.bg_stroke = egui::Stroke {
            width: 0.0,
            color: COLOR_TRANSPARENT,
        };

        // selected
        visuals.widgets.active.weak_bg_fill = COLOR_TRANSPARENT;
        visuals.widgets.active.bg_stroke = egui::Stroke {
            width: 0.0,
            color: COLOR_TRANSPARENT,
        };

        let style = ui.style_mut();
        style.spacing.button_padding = egui::Vec2::new(0.0, 0.0);
        style.spacing.item_spacing = egui::Vec2::new(0.0, 0.0);

        // Watched Apps Label
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
                ui.label(
                    egui::RichText::new("Watched Apps")
                        .text_style(egui::TextStyle::Name("Subheading".into()))
                        .color(COLOR_TEXT_WHITE),
                );

                ui.add_space(ROW_GUTTER_SPACE);
            },
        );

        // Watched Apps Index Buttons
        ui.allocate_ui_with_layout(
            egui::Vec2 {
                x: 50.0, // does not allocate enough space, but well get more automatically
                y: ROW_HEIGHT,
            },
            egui::Layout {
                main_dir: egui::Direction::LeftToRight,
                main_wrap: false,
                main_align: egui::Align::LEFT,
                main_justify: false,
                cross_align: egui::Align::Center,
                cross_justify: true,
            },
            |ui| {
                let button_width = 18.0;
                let num_apps = config.watched_apps.len();
                for i in 0..5 {
                    //
                    // Button State
                    let b_state = if false {
                        // i >= num_apps {
                        ButtonState::Disabled
                    } else if i == state.ui.config_watched_app_index {
                        ButtonState::Selected
                    } else {
                        ButtonState::Normal
                    };
                    // To Do: Error state

                    // Button Wrapper
                    let (rect, response) =
                        ui.allocate_exact_size(Vec2::new(button_width, 14.0), egui::Sense::click());

                    let hover = response.hovered();

                    let mut child_ui = ui.child_ui(
                        rect,
                        egui::Layout {
                            main_dir: egui::Direction::TopDown,
                            main_wrap: false,
                            main_align: egui::Align::BOTTOM,
                            main_justify: true,
                            cross_align: egui::Align::Center,
                            cross_justify: true,
                        },
                    );

                    // Label
                    let _b = child_ui.label(
                        egui::RichText::new((i + 1).to_string())
                            .text_style(egui::TextStyle::Name("TextButton".into()))
                            .color(match b_state {
                                ButtonState::Disabled => COLOR_MED_GREY,
                                ButtonState::Selected => COLOR_TEXT_WHITE,
                                ButtonState::Normal => COLOR_OFFWHITE,
                                ButtonState::Error => COLOR_RED,
                            }),
                    );

                    child_ui.add_space(4.0);

                    // Underline
                    let (underline_rect, _resp) = child_ui
                        .allocate_exact_size(Vec2::new(button_width, 2.0), egui::Sense::hover());

                    child_ui.painter().rect(
                        underline_rect,
                        0.0,
                        if hover || b_state == ButtonState::Selected {
                            COLOR_YELLOW
                        } else {
                            COLOR_TRANSPARENT
                        },
                        egui::Stroke::new(0.0, COLOR_TRANSPARENT),
                    );

                    if response.clicked() {
                        println!("clicked {:?}", i);
                    }
                    response.on_hover_cursor(egui::CursorIcon::PointingHand);

                    ui.add_space(6.0);
                }
            },
        );

        // Watched Apps Create / Delete Buttons
        ui.allocate_ui_with_layout(
            egui::Vec2 {
                x: ui.available_width(),
                y: ROW_HEIGHT,
            },
            egui::Layout {
                main_dir: egui::Direction::RightToLeft,
                main_wrap: false,
                main_align: egui::Align::RIGHT,
                main_justify: false,
                cross_align: egui::Align::Center,
                cross_justify: false,
            },
            |ui| {
                components::format_imagebuttons(ui);

                let icon_w2 = egui::Vec2::new(24.0, 24.0);
                let icon_create = state.ui.textures.get("icon_create").unwrap();
                let icon_delete = state.ui.textures.get("icon_delete").unwrap();

                // New
                let r = ui.add(egui::ImageButton::new(&icon_delete.1, icon_w2));
                if r.clicked() {
                    println!("clicked delete");
                }
                r.on_hover_cursor(egui::CursorIcon::PointingHand);

                ui.add_space(8.0);

                // Delete
                let r = ui.add(egui::ImageButton::new(&icon_create.1, icon_w2));
                if r.clicked() {
                    println!("clicked create");
                }
                r.on_hover_cursor(egui::CursorIcon::PointingHand);
            },
        );
    });

    ui.add_space(12.0);

    let current_app_index = 0;

    components::draw_row(
        ui,
        state,
        "Name",
        &mut config.watched_apps[current_app_index].name,
        true,
    );

    components::draw_row(
        ui,
        state,
        "Run",
        &mut config.watched_apps[current_app_index].run,
        true,
    );

    components::draw_row(
        ui,
        state,
        "OSC Port In (Client)",
        &mut config.watched_apps[current_app_index].osc_in_port,
        true,
    );

    components::draw_row(
        ui,
        state,
        "OSC Port Out (Client)",
        &mut config.watched_apps[current_app_index].osc_out_port,
        true,
    );

    components::draw_row(
        ui,
        state,
        "Hearbeat OSC Channel",
        &mut config.watched_apps[current_app_index].heartbeat_channel,
        true,
    );

    components::draw_row(
        ui,
        state,
        "Heartbeat Interval (sec)",
        &mut config.watched_apps[current_app_index].heartbeat_interval,
        true,
    );

    components::draw_row(
        ui,
        state,
        "Heartbeat Timeout (sec)",
        &mut config.watched_apps[current_app_index].heartbeat_timeout,
        true,
    );

    components::draw_row(
        ui,
        state,
        "Startup Timeout (sec)",
        &mut config.watched_apps[current_app_index].startup_timeout,
        true,
    );

    components::draw_row(
        ui,
        state,
        "Restart Delay (sec)",
        &mut config.watched_apps[current_app_index].restart_delay,
        true,
    );

    // filler for scroll testing
    components::draw_row(
        ui,
        state,
        "Restart Delay (sec)",
        &mut config.watched_apps[current_app_index].restart_delay,
        true,
    );
    components::draw_row(
        ui,
        state,
        "Restart Delay (sec)",
        &mut config.watched_apps[current_app_index].restart_delay,
        true,
    );
    components::draw_row(
        ui,
        state,
        "Restart Delay (sec)",
        &mut config.watched_apps[current_app_index].restart_delay,
        true,
    );
}
