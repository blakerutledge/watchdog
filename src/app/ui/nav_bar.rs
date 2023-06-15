use egui::Pos2;

use crate::app::ui::*;

const COLUMN_WIDTH: f32 = 64.0;
const ICON_SIZE: f32 = 44.0;
const CORNER_ROUND: f32 = 4.0;
const MARGIN: f32 = 10.0;
const STATUS_DOT_RADIUS: f32 = 3.0;

pub fn draw(context: &egui::Context, state: &mut State, config: &Config) {
    egui::SidePanel::left("nav_bar")
        .exact_width(COLUMN_WIDTH)
        .frame(egui::Frame::none().fill(COLOR_MED_GREY))
        .resizable(false)
        .show_separator_line(false)
        .show(context, |ui| {
            ui.with_layout(egui::Layout::top_down(egui::Align::Center), |ui| {
                //
                // Reset the default Egui style
                style_imagebuttons(ui);

                // Draw Config Button
                draw_nav_button(ui, state, TabState::Config, config.valid);

                // Draw Apps Button
                let apps_valid = false; // TO DO: make real valid / invalid status
                draw_nav_button(ui, state, TabState::Apps, apps_valid);

                // Draw Stats Button
                let stats_valid = false; // TO DO: make real valid / invalid status
                draw_nav_button(ui, state, TabState::Stats, stats_valid);

                // Float to bottom
                ui.add_space(ui.available_height() - ICON_SIZE - MARGIN);

                // Draw Exit Button
                // This one does not actually have an associated tab in TabState,
                // so we cant draw it with the helper funtion, but a modded one
                let r = draw_exit_button(ui, state);

                // Draw the tool tip to allow various behaviors when exiting the app
                let anchor = r.rect.right_bottom();
                exit::draw(context, state, anchor);

                // Exit button click interactivity
                if r.clicked() {
                    if !state.ui.exit_tooltip_clickout && !state.ui.show_exit_tooltip {
                        state.ui.show_exit_tooltip = true;
                    }
                }
                r.on_hover_cursor(egui::CursorIcon::PointingHand);
            })
        });
}

// Helper to draw a nav button, complete with styles for various states and interactivity
fn draw_nav_button(ui: &mut egui::Ui, state: &mut State, tab_state: TabState, is_healthy: bool) {
    //
    // Flag if this is the button for the currently active tab
    let is_selected = state.ui.active_tab == tab_state && !state.ui.show_exit_tooltip;

    // Set background color
    ui.visuals_mut().widgets.inactive.weak_bg_fill = match is_selected {
        true => COLOR_DARK_GREY,
        false => COLOR_TRANSPARENT,
    };

    // Store color of left side vertical bar
    let bar_color = match is_selected {
        true => COLOR_YELLOW,
        false => COLOR_TRANSPARENT,
    };

    // Store color of top right status dot
    let dot_color = match is_healthy {
        true => COLOR_GREEN,
        false => COLOR_RED,
    };

    // Store the texture slug for lookup
    let texture_slug = match tab_state {
        TabState::Config => "icon_config",
        TabState::Apps => "icon_apps",
        TabState::Stats => "icon_stats",
    };

    // Lookup the texture handle with the slug
    let texture_handle = state.ui.textures.get(texture_slug).unwrap();

    // Draw the ImageButton
    let r = ui.add(egui::ImageButton::new(
        &texture_handle.1,
        Vec2::new(ICON_SIZE, ICON_SIZE),
    ));

    // Then draw the left hand bar
    let mut bar = r.rect.clone();
    bar.set_width(STATUS_DOT_RADIUS);
    ui.painter_at(bar).rect_filled(
        bar,
        egui::Rounding {
            nw: STATUS_DOT_RADIUS,
            sw: STATUS_DOT_RADIUS,
            ne: 0.0,
            se: 0.0,
        },
        bar_color,
    );

    // Then draw the status circle
    let mut circ = egui::Rect {
        min: Pos2::new(0.0, 0.0),
        max: Pos2::new(STATUS_DOT_RADIUS * 2.0, STATUS_DOT_RADIUS * 2.0),
    };
    circ.set_center(Pos2::new(
        r.rect.right() - STATUS_DOT_RADIUS * 2.0,
        r.rect.top() + STATUS_DOT_RADIUS * 2.0,
    ));
    ui.painter_at(circ)
        .circle_filled(circ.center(), STATUS_DOT_RADIUS, dot_color);

    // Click Interactivity, set the active tab to this type
    if r.clicked() {
        state.ui.active_tab = tab_state;
    }

    // Hover cursor icon
    r.on_hover_cursor(egui::CursorIcon::PointingHand);
}

// Helper to draw a nav button, complete with styles for various states and interactivity
fn draw_exit_button(ui: &mut egui::Ui, state: &mut State) -> egui::Response {
    let is_selected = state.ui.show_exit_tooltip;

    // Set background color
    ui.visuals_mut().widgets.inactive.weak_bg_fill = match is_selected {
        true => COLOR_DARK_GREY,
        false => COLOR_TRANSPARENT,
    };

    // Store color of left side vertical bar
    let bar_color = match is_selected {
        true => COLOR_YELLOW,
        false => COLOR_TRANSPARENT,
    };

    // Lookup the texture handle with the slug
    let texture_handle = state.ui.textures.get("icon_exit").unwrap();

    // Draw the ImageButton
    let r = ui.add(egui::ImageButton::new(
        &texture_handle.1,
        Vec2::new(ICON_SIZE, ICON_SIZE),
    ));

    // Then draw the left hand bar
    let mut bar = r.rect.clone();
    bar.set_width(STATUS_DOT_RADIUS);
    ui.painter_at(bar).rect_filled(
        bar,
        egui::Rounding {
            nw: STATUS_DOT_RADIUS,
            sw: STATUS_DOT_RADIUS,
            ne: 0.0,
            se: 0.0,
        },
        bar_color,
    );

    r
}

//
// Reset the default Egui ImageButton style to something more manageable
fn style_imagebuttons(ui: &mut egui::Ui) {
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
}
