use crate::app::ui::*;

// Transparent exterior to visible window, for easier drag to resize hit area
const BORDER_COLOR: egui::Color32 = egui::Color32::from_rgba_premultiplied(32, 32, 32, 255);
const BORDER_THICK: f32 = 1.0;

// Visible line around visible window
const STROKE_COLOR: egui::Color32 = egui::Color32::from_rgb(40, 40, 40);
const STROKE_THICK: f32 = 2.0;

pub fn draw(context: &egui::Context, state: &mut State) {
    //
    // Add very thin Top/Bottom/Side Panels for the invisible space around the visible window
    //
    egui::TopBottomPanel::top("resize_border_top")
        .show_separator_line(false)
        .frame(egui::Frame::none())
        .exact_height(BORDER_THICK)
        .show(context, |ui| {
            let (rect, resp) = ui.allocate_exact_size(
                egui::Vec2::new(ui.available_width(), BORDER_THICK),
                egui::Sense::click_and_drag(),
            );
            resp.on_hover_and_drag_cursor(state.ui.cursor_icon);
            ui.painter_at(rect).rect_filled(rect, 0.0, BORDER_COLOR);
        });

    egui::TopBottomPanel::bottom("resize_border_bottom")
        .show_separator_line(false)
        .frame(egui::Frame::none())
        .exact_height(BORDER_THICK)
        .show(context, |ui| {
            let (rect, resp) = ui.allocate_exact_size(
                egui::Vec2::new(ui.available_width(), BORDER_THICK),
                egui::Sense::click_and_drag(),
            );
            resp.on_hover_and_drag_cursor(state.ui.cursor_icon);
            ui.painter_at(rect).rect_filled(rect, 0.0, BORDER_COLOR);
        });

    egui::SidePanel::left("resize_border_left")
        .show_separator_line(false)
        .frame(egui::Frame::none())
        .exact_width(BORDER_THICK)
        .resizable(false)
        .show(context, |ui| {
            let (rect, resp) = ui.allocate_exact_size(
                egui::Vec2::new(BORDER_THICK, ui.available_height()),
                egui::Sense::click_and_drag(),
            );
            resp.on_hover_and_drag_cursor(state.ui.cursor_icon);
            ui.painter_at(rect).rect_filled(rect, 0.0, BORDER_COLOR);
        });

    egui::SidePanel::right("resize_border_right")
        .show_separator_line(false)
        .frame(egui::Frame::none())
        .exact_width(BORDER_THICK)
        .resizable(false)
        .show(context, |ui| {
            let (rect, resp) = ui.allocate_exact_size(
                egui::Vec2::new(BORDER_THICK, ui.available_height()),
                egui::Sense::click_and_drag(),
            );
            resp.on_hover_and_drag_cursor(state.ui.cursor_icon);
            ui.painter_at(rect).rect_filled(rect, 0.0, BORDER_COLOR);
        });

    //
    // Add very thin Top/Bottom/Side Panels for the visible stroke around the visible window
    //

    egui::TopBottomPanel::top("resize_stroke_top")
        .show_separator_line(false)
        .frame(egui::Frame::none())
        .exact_height(STROKE_THICK)
        .show(context, |ui| {
            let (rect, resp) = ui.allocate_exact_size(
                egui::Vec2::new(ui.available_width(), STROKE_THICK),
                egui::Sense::click_and_drag(),
            );
            resp.on_hover_and_drag_cursor(state.ui.cursor_icon);
            ui.painter_at(rect).rect_filled(rect, 0.0, STROKE_COLOR);
        });

    egui::TopBottomPanel::bottom("resize_stroke_bottom")
        .show_separator_line(false)
        .frame(egui::Frame::none())
        .exact_height(STROKE_THICK)
        .show(context, |ui| {
            let (rect, resp) = ui.allocate_exact_size(
                egui::Vec2::new(ui.available_width(), STROKE_THICK),
                egui::Sense::click_and_drag(),
            );
            resp.on_hover_and_drag_cursor(state.ui.cursor_icon);
            ui.painter_at(rect).rect_filled(rect, 0.0, STROKE_COLOR);
        });

    egui::SidePanel::left("resize_stroke_left")
        .show_separator_line(false)
        .frame(egui::Frame::none())
        .exact_width(STROKE_THICK)
        .resizable(false)
        .show(context, |ui| {
            let (rect, resp) = ui.allocate_exact_size(
                egui::Vec2::new(STROKE_THICK, ui.available_height()),
                egui::Sense::click_and_drag(),
            );
            resp.on_hover_and_drag_cursor(state.ui.cursor_icon);
            ui.painter_at(rect).rect_filled(rect, 0.0, STROKE_COLOR);
        });

    egui::SidePanel::right("resize_stroke_right")
        .show_separator_line(false)
        .frame(egui::Frame::none())
        .exact_width(STROKE_THICK)
        .resizable(false)
        .show(context, |ui| {
            let (rect, resp) = ui.allocate_exact_size(
                egui::Vec2::new(STROKE_THICK, ui.available_height()),
                egui::Sense::click_and_drag(),
            );
            resp.on_hover_and_drag_cursor(state.ui.cursor_icon);
            ui.painter_at(rect).rect_filled(rect, 0.0, STROKE_COLOR);
        });
}
