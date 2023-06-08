use std::time::Duration;

use egui::Vec2;

use super::config::Config;
use super::state::State;

use super::utils;

pub fn init() -> Box<dyn FnMut(&egui::Context, &mut State, &mut Config, &winit::window::Window)> {
    // all images embedded in binary
    let icon_config = include_bytes!("../../assets/icons/icon-config.png");
    let icon_play = include_bytes!("../../assets/icons/icon-play.png");
    let icon_stats = include_bytes!("../../assets/icons/icon-stats.png");
    let icon_exit = include_bytes!("../../assets/icons/icon-exit.png");

    let logo = include_bytes!("../../assets/icons/watchdog-logo.png");
    let icon_min = include_bytes!("../../assets/icons/icon-min.png");
    let icon_max = include_bytes!("../../assets/icons/icon-max.png");
    let icon_unmax = include_bytes!("../../assets/icons/icon-unmax.png");
    let icon_close = include_bytes!("../../assets/icons/icon-close.png");

    let font_monolisa = include_bytes!("../../assets/fonts/monolisa/MonoLisa.otf");

    Box::new(
        |context: &egui::Context,
         state: &mut State,
         config: &mut Config,
         window: &winit::window::Window| {
            {
                // load any missing images
                if !state.ui.textures.contains_key("icon_config") {
                    create_tex(icon_config, "icon_config", context, state);
                    println!("created icon_config texture");
                }

                if !state.ui.textures.contains_key("icon_play") {
                    create_tex(icon_play, "icon_play", context, state);
                    println!("created icon_play texture");
                }

                if !state.ui.textures.contains_key("icon_stats") {
                    create_tex(icon_stats, "icon_stats", context, state);
                    println!("created icon_stats texture");
                }

                if !state.ui.textures.contains_key("icon_exit") {
                    create_tex(icon_exit, "icon_exit", context, state);
                    println!("created icon_exit texture");
                };

                if !state.ui.textures.contains_key("logo") {
                    create_tex(logo, "logo", context, state);
                    println!("created logo texture");
                }

                if !state.ui.textures.contains_key("icon_min") {
                    create_tex(icon_min, "icon_min", context, state);
                    println!("created icon_min texture");
                }

                if !state.ui.textures.contains_key("icon_max") {
                    create_tex(icon_max, "icon_max", context, state);
                    println!("created icon_max texture");
                }

                if !state.ui.textures.contains_key("icon_unmax") {
                    create_tex(icon_unmax, "icon_unmax", context, state);
                    println!("created icon_unmax texture");
                }

                if !state.ui.textures.contains_key("icon_close") {
                    create_tex(icon_close, "icon_close", context, state);
                    println!("created icon_close texture");
                }
                /*
                   add more here
                */

                // load fonts
                if !state.ui.custom_fonts {
                    // init font
                    let mut fonts = egui::FontDefinitions::default();

                    // Install my own font (maybe supporting non-latin characters).
                    // .ttf and .otf files supported.
                    fonts.font_data.insert(
                        "monolisa".to_owned(),
                        egui::FontData::from_static(font_monolisa),
                    );

                    // Put my font first (highest priority) forboth monospace and proportional text:
                    fonts
                        .families
                        .entry(egui::FontFamily::Proportional)
                        .or_default()
                        .insert(0, "monolisa".to_owned());

                    fonts
                        .families
                        .entry(egui::FontFamily::Monospace)
                        .or_default()
                        .insert(0, "monolisa".to_owned());

                    // Tell egui to use these fonts:
                    context.set_fonts(fonts);

                    use egui::FontFamily::Monospace;
                    use egui::{FontId, TextStyle};

                    let mut style = (*context.style()).clone();
                    style.text_styles = [
                        (
                            TextStyle::Name("Title".into()),
                            FontId::new(14.0, Monospace),
                        ),
                        (TextStyle::Heading, FontId::new(10.0, Monospace)),
                        (TextStyle::Body, FontId::new(10.0, Monospace)),
                        (TextStyle::Monospace, FontId::new(10.0, Monospace)),
                        (TextStyle::Button, FontId::new(10.0, Monospace)),
                        (TextStyle::Small, FontId::new(8.0, Monospace)),
                    ]
                    .into();
                    context.set_style(style);

                    // set flag to true so we only do this once
                    state.ui.custom_fonts = true
                }
            }

            if !window.is_maximized() {
                draw_resize_borders(context, state);
            }

            egui::TopBottomPanel::top("title_bar")
                .exact_height(40.0)
                .frame(
                    egui::Frame::none().fill(egui::Color32::from_rgb(40, 40, 40)), // .inner_margin(egui::style::Margin {
                                                                                   //     top: 2.0,
                                                                                   //     right: 2.0,
                                                                                   //     left: 2.0,
                                                                                   //     bottom: 0.0,
                                                                                   // }),
                )
                .resizable(false)
                .show_separator_line(false)
                .show(context, |ui| {
                    // ui.with_layout(egui::Layout::top_down(egui::Align::Center), |ui| {
                    //     // ui.add_space(4.0);

                    // ui.group(|ui| {
                    ui.with_layout(egui::Layout::left_to_right(egui::Align::Center), |ui| {
                        let theme = ui.visuals_mut();

                        theme.widgets.noninteractive.bg_stroke = egui::Stroke {
                            width: 0.0,
                            color: egui::Color32::TRANSPARENT,
                        };

                        // baseline
                        theme.widgets.inactive.weak_bg_fill = egui::Color32::TRANSPARENT;

                        // clicking
                        theme.widgets.active.weak_bg_fill = egui::Color32::from_rgb(22, 22, 22);
                        theme.widgets.active.bg_stroke = egui::Stroke {
                            width: 0.0,
                            color: egui::Color32::TRANSPARENT,
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
                            color: egui::Color32::TRANSPARENT,
                        };
                        theme.widgets.hovered.rounding = egui::Rounding {
                            nw: 0.0,
                            sw: 0.0,
                            se: 0.0,
                            ne: 0.0,
                        };

                        // disabled
                        theme.widgets.noninteractive.weak_bg_fill = egui::Color32::TRANSPARENT;

                        let style = ui.style_mut();
                        style.spacing.button_padding = egui::Vec2::new(0.0, 0.0);
                        style.spacing.window_margin = egui::Margin {
                            left: 0.0,
                            right: 0.0,
                            top: 0.0,
                            bottom: 0.0,
                        };
                        style.spacing.item_spacing = egui::Vec2::new(0.0, 0.0);
                        let logo = state.ui.textures.get("logo").unwrap();
                        let logo_size = egui::Vec2::new(25.0, 25.0);

                        let icon_min = state.ui.textures.get("icon_min").unwrap();
                        let icon_max = state.ui.textures.get("icon_max").unwrap();
                        let icon_unmax = state.ui.textures.get("icon_unmax").unwrap();
                        let icon_close = state.ui.textures.get("icon_close").unwrap();

                        let icon_w2 = Vec2::new(48.0, 40.0);

                        // Titlebar
                        let group = ui.group(|ui| {
                            ui.add_space(12.0);
                            ui.image(&logo.1, logo_size);
                            ui.add_space(24.0);
                            ui.label(
                                egui::RichText::new("Watchdog")
                                    .text_style(egui::TextStyle::Name("Title".into()))
                                    .color(egui::Color32::from_rgb(238, 238, 238)),
                            );
                            ui.add_space(ui.available_width() - icon_w2.x * 3.0);
                        });

                        // Title bar draggable to move window
                        let r = ui.interact(
                            group.response.rect,
                            egui::Id::new("title_bar_content"),
                            egui::Sense::click_and_drag(),
                        );
                        if r.drag_started() {
                            // TO DO, but have 8px unselectable resize area at the top
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

                        // Window Minimize
                        let r = ui.add(egui::ImageButton::new(&icon_min.1, icon_w2));
                        if r.clicked() {
                            state.actions.window_minimize = true;
                        }

                        // Window Maximize
                        let r = ui.add(egui::ImageButton::new(
                            if window.is_maximized() {
                                &icon_unmax.1
                            } else {
                                &icon_max.1
                            },
                            icon_w2,
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
                        theme.widgets.hovered.weak_bg_fill = egui::Color32::from_rgb(251, 81, 48);
                        theme.widgets.active.weak_bg_fill = egui::Color32::from_rgb(238, 58, 23);

                        // Window Close
                        let r = ui.add(egui::ImageButton::new(&icon_close.1, icon_w2));
                        if r.clicked() {
                            state.actions.window_close = true;
                        }
                    });
                    // });
                });

            let mut exit_tooltip_clickout = false;

            egui::Area::new("Exit")
                .order(egui::Order::Foreground)
                // .resizable(false)
                .movable(false)
                // .collapsible(false)
                // .title_bar(true)
                .pivot(egui::Align2::LEFT_BOTTOM)
                .default_pos(egui::Pos2::new(50.0, 700.0))
                .show(context, |ui| {
                    if state.ui.show_exit_tooltip {
                        let group = ui.group(|ui| {
                            let r = ui.button("Close Watchdog");
                            if r.clicked() {
                                state.actions.window_close = true;
                                state.ui.show_exit_tooltip = false;
                            }
                            let r = ui.button("Quit Watchdog");
                            if r.clicked() {
                                state.actions.app_exit = true;
                                state.ui.show_exit_tooltip = false;
                            }
                            let r = ui.button("Quit Watchdog and Quit Watched Apps");
                            if r.clicked() {
                                // state.actions.app_exit = true;
                                state.ui.show_exit_tooltip = false;
                            }
                        });

                        if group.response.clicked_elsewhere() {
                            println!("clicked elsewhere");
                            exit_tooltip_clickout = true;
                            state.ui.show_exit_tooltip = false;
                        }
                    }
                });

            egui::SidePanel::left("nav_bar")
                .exact_width(64.0)
                .frame(egui::Frame::none().fill(egui::Color32::from_rgb(40, 40, 40)))
                .resizable(false)
                .show_separator_line(false)
                .show(context, |ui| {
                    ui.with_layout(egui::Layout::top_down(egui::Align::Center), |ui| {
                        // Get Icon TextureHandles
                        let icon_config = state.ui.textures.get("icon_config").unwrap();
                        let icon_play = state.ui.textures.get("icon_play").unwrap();
                        let icon_stats = state.ui.textures.get("icon_stats").unwrap();
                        let icon_exit = state.ui.textures.get("icon_exit").unwrap();

                        let icon_w = 44.0;
                        let icon_w2 = Vec2::new(icon_w, icon_w);

                        let theme = ui.visuals_mut();

                        // baseline
                        theme.widgets.inactive.weak_bg_fill = egui::Color32::TRANSPARENT;

                        // clicking
                        theme.widgets.active.weak_bg_fill = egui::Color32::from_rgb(22, 22, 22);
                        theme.widgets.active.bg_stroke = egui::Stroke {
                            width: 0.0,
                            color: egui::Color32::TRANSPARENT,
                        };
                        theme.widgets.active.rounding = egui::Rounding {
                            nw: 8.0,
                            sw: 8.0,
                            se: 8.0,
                            ne: 8.0,
                        };

                        // hovering
                        theme.widgets.hovered.weak_bg_fill = egui::Color32::from_rgb(29, 29, 29);
                        theme.widgets.hovered.bg_stroke = egui::Stroke {
                            width: 0.0,
                            color: egui::Color32::TRANSPARENT,
                        };
                        theme.widgets.hovered.rounding = egui::Rounding {
                            nw: 8.0,
                            sw: 8.0,
                            se: 8.0,
                            ne: 8.0,
                        };

                        // disabled
                        theme.widgets.noninteractive.weak_bg_fill = egui::Color32::TRANSPARENT;

                        let style = ui.style_mut();
                        style.spacing.button_padding = egui::Vec2::new(0.0, 0.0);
                        style.spacing.item_spacing = egui::Vec2::new(0.0, 10.0);

                        // Config
                        let r = ui.add(egui::ImageButton::new(&icon_config.1, icon_w2));
                        if r.clicked() {
                            state.ui.active_tab = super::state::TabState::Config;
                        }
                        r.on_hover_cursor(egui::CursorIcon::PointingHand);

                        // Play
                        let r = ui.add(egui::ImageButton::new(&icon_play.1, icon_w2));
                        if r.clicked() {
                            state.ui.active_tab = super::state::TabState::Play;
                        }
                        r.on_hover_cursor(egui::CursorIcon::PointingHand);

                        // Stats
                        let r = ui.add(egui::ImageButton::new(&icon_stats.1, icon_w2));
                        if r.clicked() {
                            state.ui.active_tab = super::state::TabState::Stats;
                        }
                        r.on_hover_cursor(egui::CursorIcon::PointingHand);

                        // Float to bottom
                        ui.add_space(ui.available_height() - icon_w - 10.0);

                        // Exit
                        let r = ui.add(egui::ImageButton::new(&icon_exit.1, icon_w2));
                        if r.clicked() {
                            if !exit_tooltip_clickout && !state.ui.show_exit_tooltip {
                                state.ui.show_exit_tooltip = true;
                            }
                        }
                        r.on_hover_cursor(egui::CursorIcon::PointingHand);
                    })
                });

            /*
            egui::Window::new("Host Performance")
                .resizable(false)
                .movable(true)
                .collapsible(false)
                .title_bar(true)
                .pivot(egui::Align2::RIGHT_BOTTOM)
                .default_pos(egui::Pos2::new(99999.0, 9999999.0))
                .show(context, |ui| {
                    //
                    // host perf
                    //
                    ui.add_space(6.0);
                    // ui.strong("Host Performance");
                    ui.label(format!("FPS: {}", state.perf.fps));

                    // If there is only one frame in the list, it is partially completed,
                    // and does not yet have a frame.stop time that we can use. The first frame is
                    // currently being rendered!
                    let mut frametime = format_ms(0.0);
                    if state.perf.frames.len() >= 2 {
                        // Render the previous frame's stats, this frame
                        let f = &state.perf.frames[state.perf.frames.len() - 2];
                        let diff = f.stop.checked_sub(f.start);
                        match diff {
                            Some(diff) => {
                                frametime = format_ms(diff.as_nanos() as f32 / 1e6);
                            }
                            _ => {}
                        }
                    }
                    ui.label(format!("Frame Time: {} ms", frametime));

                    ui.label(format!(
                        "Avg Frame Time: {}ms",
                        format_ms(state.perf.avg_frame_time)
                    ));
                });
            */

            egui::CentralPanel::default().show(context, |ui| {
                match state.ui.active_tab {
                    super::state::TabState::Config => draw_config(ui, state, config),
                    super::state::TabState::Play => {
                        ui.label("Play");
                    }
                    super::state::TabState::Stats => {
                        ui.label("Stats");
                    }
                }

                // if ui.button("Abort Watched Task").clicked() {
                //     state.actions.app_exit = true
                // }

                // if ui.button("Quit Watchdog and Abort Watched Task").clicked() {
                //     state.actions.app_exit = true
                // }
            });
        },
    )
}

fn draw_config(ui: &mut egui::Ui, state: &mut State, config: &mut Config) {
    ui.label("Config");

    //
    // config
    //

    if ui.button("Save config to:").clicked() {
        if let Some(path) = rfd::FileDialog::new()
            .add_filter("json", &["json"])
            .save_file()
        {
            super::config::move_config(path, state, config);
        }
    }

    if ui.button("Load config from:").clicked() {
        if let Some(file) = rfd::FileDialog::new()
            .add_filter("json", &["json"])
            .pick_file()
        {
            super::config::replace_from_file(file, state, config);
        }
    }

    if ui.button("Reset config to defaults:").clicked() {
        super::config::reset_config(state, config);
    }

    if ui.button("Reset to default file:").clicked() {
        super::config::reinit_config(state, config);
    }

    ui.horizontal(|ui| {
        ui.label("Config Filepath");
        let config_filepath_label = state.json.filepath.to_str().unwrap();
        ui.monospace(config_filepath_label);
    });

    ui.add_space(20.0);

    let r = ui.text_edit_singleline(&mut config.hello);
    if r.changed() {
        config.write(&state.json.filepath);
    };
}

fn format_ms(f: f32) -> String {
    format!("{:06.3}", f)
}

fn create_tex(image_data: &[u8], slug: &str, context: &egui::Context, state: &mut State) {
    let image = image::load_from_memory(image_data)
        .expect(format!("Failed to load image {}", slug).as_str());
    let size = [image.width() as _, image.height() as _];
    let image_buffer = image.to_rgba8();
    let pixels = image_buffer.as_flat_samples();

    let i = egui::ColorImage::from_rgba_unmultiplied(size, pixels.as_slice());

    let raw_size = egui::Vec2::new(size[0] as f32, size[1] as f32);
    let tex = context.load_texture(slug, i, Default::default());

    state.ui.textures.insert(slug.to_string(), (raw_size, tex));
}

fn draw_resize_borders(context: &egui::Context, state: &mut super::state::State) {
    // transparent exterior to visible window, for easier drag to resize hit area
    let border_color = egui::Color32::from_rgba_unmultiplied(0, 0, 0, 20);
    let border_thick = 3.0;

    // visual line around visible window
    let stroke_color = egui::Color32::from_rgb(40, 40, 40);
    let stroke_thick = 2.0;

    egui::TopBottomPanel::top("resize_border_top")
        .show_separator_line(false)
        .frame(egui::Frame::none())
        .exact_height(border_thick)
        .show(context, |ui| {
            let (rect, resp) = ui.allocate_exact_size(
                egui::Vec2::new(ui.available_width(), border_thick),
                egui::Sense::click_and_drag(),
            );
            resp.on_hover_and_drag_cursor(state.ui.cursor_icon);
            ui.painter_at(rect).rect_filled(rect, 0.0, border_color);
        });

    egui::TopBottomPanel::bottom("resize_border_bottom")
        .show_separator_line(false)
        .frame(egui::Frame::none())
        .exact_height(border_thick)
        .show(context, |ui| {
            let (rect, resp) = ui.allocate_exact_size(
                egui::Vec2::new(ui.available_width(), border_thick),
                egui::Sense::click_and_drag(),
            );
            resp.on_hover_and_drag_cursor(state.ui.cursor_icon);
            ui.painter_at(rect).rect_filled(rect, 0.0, border_color);
        });

    egui::SidePanel::left("resize_border_left")
        .show_separator_line(false)
        .frame(egui::Frame::none())
        .exact_width(border_thick)
        .resizable(false)
        .show(context, |ui| {
            let (rect, resp) = ui.allocate_exact_size(
                egui::Vec2::new(border_thick, ui.available_height()),
                egui::Sense::click_and_drag(),
            );
            resp.on_hover_and_drag_cursor(state.ui.cursor_icon);
            ui.painter_at(rect).rect_filled(rect, 0.0, border_color);
        });

    egui::SidePanel::right("resize_border_right")
        .show_separator_line(false)
        .frame(egui::Frame::none())
        .exact_width(border_thick)
        .resizable(false)
        .show(context, |ui| {
            let (rect, resp) = ui.allocate_exact_size(
                egui::Vec2::new(border_thick, ui.available_height()),
                egui::Sense::click_and_drag(),
            );
            resp.on_hover_and_drag_cursor(state.ui.cursor_icon);
            ui.painter_at(rect).rect_filled(rect, 0.0, border_color);
        });

    egui::TopBottomPanel::top("resize_stroke_top")
        .show_separator_line(false)
        .frame(egui::Frame::none())
        .exact_height(stroke_thick)
        .show(context, |ui| {
            let (rect, resp) = ui.allocate_exact_size(
                egui::Vec2::new(ui.available_width(), stroke_thick),
                egui::Sense::click_and_drag(),
            );
            resp.on_hover_and_drag_cursor(state.ui.cursor_icon);
            ui.painter_at(rect).rect_filled(rect, 0.0, stroke_color);
        });

    egui::TopBottomPanel::bottom("resize_stroke_bottom")
        .show_separator_line(false)
        .frame(egui::Frame::none())
        .exact_height(stroke_thick)
        .show(context, |ui| {
            let (rect, resp) = ui.allocate_exact_size(
                egui::Vec2::new(ui.available_width(), stroke_thick),
                egui::Sense::click_and_drag(),
            );
            resp.on_hover_and_drag_cursor(state.ui.cursor_icon);
            ui.painter_at(rect).rect_filled(rect, 0.0, stroke_color);
        });

    egui::SidePanel::left("resize_stroke_left")
        .show_separator_line(false)
        .frame(egui::Frame::none())
        .exact_width(stroke_thick)
        .resizable(false)
        .show(context, |ui| {
            let (rect, resp) = ui.allocate_exact_size(
                egui::Vec2::new(stroke_thick, ui.available_height()),
                egui::Sense::click_and_drag(),
            );
            resp.on_hover_and_drag_cursor(state.ui.cursor_icon);
            ui.painter_at(rect).rect_filled(rect, 0.0, stroke_color);
        });

    egui::SidePanel::right("resize_stroke_right")
        .show_separator_line(false)
        .frame(egui::Frame::none())
        .exact_width(stroke_thick)
        .resizable(false)
        .show(context, |ui| {
            let (rect, resp) = ui.allocate_exact_size(
                egui::Vec2::new(stroke_thick, ui.available_height()),
                egui::Sense::click_and_drag(),
            );
            resp.on_hover_and_drag_cursor(state.ui.cursor_icon);
            ui.painter_at(rect).rect_filled(rect, 0.0, stroke_color);
        });
}
