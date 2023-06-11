// use crate::app::config;
use crate::app::ui::*;
//
// Config section all ui elements
//

pub fn draw(ui: &mut egui::Ui, state: &mut State, config: &mut Config) {
    // Header
    ui.heading(egui::RichText::new("Config").color(COLOR_TEXT_WHITE));

    ui.add_space(SECTION_HEADING_MARGIN);

    /*
    if ui.button("Save config to:").clicked() {
        if let Some(path) = rfd::FileDialog::new()
            .add_filter("json", &["json"])
            .save_file()
        {
            config::move_config(path, state, config);
        }
    }

    if ui.button("Load config from:").clicked() {
        if let Some(file) = rfd::FileDialog::new()
            .add_filter("json", &["json"])
            .pick_file()
        {
            config::replace_from_file(file, state, config);
        }
    }

    if ui.button("Reset config to defaults:").clicked() {
        config::reset_config(state, config);
    }

    if ui.button("Reset to default file:").clicked() {
        config::reinit_config(state, config);
    }

    ui.horizontal(|ui| {
        ui.label("Config Filepath");
        let config_filepath_label = state.json.filepath.to_str().unwrap();
        // ui.monospace(config_filepath_label);
    });
    */

    ui.add_space(20.0);

    components::draw_row(
        ui,
        state,
        "JSON Filepath",
        &mut state.json.filepath.to_str().unwrap().to_string(),
        false,
    );

    components::draw_separator(ui);

    // App Index Selector
    ui.horizontal(|ui| {
        let visuals = ui.visuals_mut();

        visuals.widgets.noninteractive.bg_stroke = egui::Stroke {
            width: 0.0,
            color: COLOR_TRANSPARENT,
        };

        // baseline
        visuals.widgets.inactive.weak_bg_fill = COLOR_TRANSPARENT;

        // clicking
        visuals.widgets.active.weak_bg_fill = COLOR_DARKER_GREY;
        visuals.widgets.active.bg_stroke = egui::Stroke {
            width: 0.0,
            color: COLOR_TRANSPARENT,
        };
        visuals.widgets.active.rounding = egui::Rounding {
            nw: 0.0,
            sw: 0.0,
            se: 0.0,
            ne: 0.0,
        };

        // hovering
        visuals.widgets.hovered.weak_bg_fill =
            egui::Color32::from_rgba_unmultiplied(255, 255, 255, 120);
        visuals.widgets.hovered.bg_stroke = egui::Stroke {
            width: 0.0,
            color: COLOR_TRANSPARENT,
        };
        visuals.widgets.hovered.rounding = egui::Rounding {
            nw: 0.0,
            sw: 0.0,
            se: 0.0,
            ne: 0.0,
        };

        // disabled
        visuals.widgets.noninteractive.weak_bg_fill = COLOR_TRANSPARENT;

        let style = ui.style_mut();
        style.spacing.button_padding = egui::Vec2::new(4.0, 4.0);
        style.spacing.window_margin = egui::Margin {
            left: 0.0,
            right: 0.0,
            top: 0.0,
            bottom: 0.0,
        };
        style.spacing.item_spacing = egui::Vec2::new(4.0, 0.0);

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
                ui.add_space(10.0);
                for i in 0..5 {
                    let r = ui.add(
                        egui::Button::new(
                            egui::RichText::new((i + 1).to_string())
                                .text_style(egui::TextStyle::Name("TextButton".into()))
                                .color(COLOR_TEXT_WHITE),
                        ), // .min_size(egui::Vec2::new(ROW_HEIGHT, ROW_HEIGHT)),
                    );
                    if r.clicked() {
                        println!("clicked {:?}", i);
                    }
                    r.on_hover_cursor(egui::CursorIcon::PointingHand);
                }
            },
        );

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
                cross_justify: true,
            },
            |ui| {
                // Delete
                let r = ui.add(egui::Button::new(
                    egui::RichText::new("Delete".to_string())
                        .text_style(egui::TextStyle::Body)
                        .color(COLOR_TEXT_WHITE),
                ));
                if r.clicked() {
                    println!("clicked delete");
                }
                r.on_hover_cursor(egui::CursorIcon::PointingHand);

                // New
                let r = ui.add(egui::Button::new(
                    egui::RichText::new("Add".to_string())
                        .text_style(egui::TextStyle::Body)
                        .color(COLOR_TEXT_WHITE),
                ));
                if r.clicked() {
                    println!("clicked add");
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
