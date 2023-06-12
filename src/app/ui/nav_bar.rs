use egui::Pos2;

use crate::app::ui::*;

const COLUMN_WIDTH: f32 = 64.0;
const ICON_SIZE: f32 = 44.0;
const CORNER_ROUND: f32 = 4.0;
const MARGIN: f32 = 10.0;

pub fn draw(context: &egui::Context, state: &mut State) {
    egui::SidePanel::left("nav_bar")
        .exact_width(COLUMN_WIDTH)
        .frame(egui::Frame::none().fill(COLOR_MED_GREY))
        .resizable(false)
        .show_separator_line(false)
        .show(context, |ui| {
            ui.with_layout(egui::Layout::top_down(egui::Align::Center), |ui| {
                // Get Icon TextureHandles
                let icon_config = state.ui.textures.get("icon_config").unwrap();
                let icon_apps = state.ui.textures.get("icon_apps").unwrap();
                let icon_stats = state.ui.textures.get("icon_stats").unwrap();
                let icon_exit = state.ui.textures.get("icon_exit").unwrap();

                let icon_w2 = Vec2::new(ICON_SIZE, ICON_SIZE);

                let theme = ui.visuals_mut();

                // baseline
                theme.widgets.inactive.weak_bg_fill = COLOR_TRANSPARENT;

                // clicking
                theme.widgets.active.weak_bg_fill = COLOR_DARKER_GREY;
                theme.widgets.active.bg_stroke = egui::Stroke {
                    width: 0.0,
                    color: COLOR_TRANSPARENT,
                };
                theme.widgets.active.rounding = egui::Rounding {
                    nw: CORNER_ROUND,
                    sw: CORNER_ROUND,
                    se: CORNER_ROUND,
                    ne: CORNER_ROUND,
                };

                // hovering
                theme.widgets.hovered.weak_bg_fill = COLOR_DARK_GREY;
                theme.widgets.hovered.bg_stroke = egui::Stroke {
                    width: 0.0,
                    color: COLOR_TRANSPARENT,
                };
                theme.widgets.hovered.rounding = egui::Rounding {
                    nw: CORNER_ROUND,
                    sw: CORNER_ROUND,
                    se: CORNER_ROUND,
                    ne: CORNER_ROUND,
                };

                // disabled
                theme.widgets.noninteractive.weak_bg_fill = COLOR_TRANSPARENT;

                let style = ui.style_mut();
                style.spacing.button_padding = egui::Vec2::new(0.0, 0.0);
                style.spacing.item_spacing = egui::Vec2::new(0.0, MARGIN - 1.0); // ???

                // let r = ui.add(egui::ImageButton::new(&icon_config.1, icon_w2));
                let r = draw_nav_button(
                    ui,
                    &icon_config.1,
                    state.ui.active_tab == TabState::Config,
                    true,
                );
                if r.clicked() {
                    state.ui.active_tab = TabState::Config;
                }
                r.on_hover_cursor(egui::CursorIcon::PointingHand);

                //
                //
                // Apps
                let r = draw_nav_button(
                    ui,
                    &icon_apps.1,
                    state.ui.active_tab == TabState::Apps,
                    false,
                );
                if r.clicked() {
                    state.ui.active_tab = TabState::Apps;
                }
                r.on_hover_cursor(egui::CursorIcon::PointingHand);

                // Stats
                let r = draw_nav_button(
                    ui,
                    &icon_stats.1,
                    state.ui.active_tab == TabState::Stats,
                    false,
                );
                if r.clicked() {
                    state.ui.active_tab = TabState::Stats;
                }
                r.on_hover_cursor(egui::CursorIcon::PointingHand);

                // Float to bottom
                ui.add_space(ui.available_height() - ICON_SIZE - MARGIN);

                // Exit
                let r = ui.add(egui::ImageButton::new(&icon_exit.1, icon_w2));
                if r.clicked() {
                    if !state.ui.exit_tooltip_clickout && !state.ui.show_exit_tooltip {
                        state.ui.show_exit_tooltip = true;
                    }
                }
                r.on_hover_cursor(egui::CursorIcon::PointingHand);
            })
        });
}

fn draw_nav_button(
    ui: &mut egui::Ui,
    texture_id: &egui::TextureHandle,
    is_selected: bool,
    is_healthy: bool,
) -> egui::Response {
    if is_selected {
        ui.visuals_mut().widgets.inactive.weak_bg_fill = COLOR_DARK_GREY;
    } else {
        ui.visuals_mut().widgets.inactive.weak_bg_fill = COLOR_TRANSPARENT;
    }
    //
    // Start with the Image Button
    let r = ui.add(egui::ImageButton::new(
        texture_id,
        Vec2::new(ICON_SIZE, ICON_SIZE),
    ));

    // Then draw the left hand bar
    let mut bar = r.rect.clone();
    bar.set_width(3.0);
    ui.painter_at(bar).rect_filled(
        bar,
        egui::Rounding {
            nw: 3.0,
            sw: 3.0,
            ne: 0.0,
            se: 0.0,
        },
        if is_selected {
            COLOR_YELLOW
        } else {
            COLOR_TRANSPARENT
        },
    );

    // Then draw the status circle
    let mut circ = egui::Rect {
        min: Pos2::new(0.0, 0.0),
        max: Pos2::new(6.0, 6.0),
    };
    circ.set_center(Pos2::new(r.rect.right() - 6.0, r.rect.top() + 6.0));
    ui.painter_at(circ).circle_filled(
        circ.center(),
        3.0,
        if is_healthy { COLOR_GREEN } else { COLOR_RED },
    );

    r
}
