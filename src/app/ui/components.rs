use crate::app::ui::*;

pub fn draw_separator(ui: &mut egui::Ui) {
    ui.add_space(8.0);
    ui.horizontal(|ui| {
        let (rect, _resp) = ui.allocate_exact_size(
            egui::Vec2::new(ui.available_width(), 1.0),
            egui::Sense::hover(),
        );
        ui.painter_at(rect)
            .rect_filled(rect, 0.0, egui::Color32::from_rgb(63, 63, 63));
    });
    ui.add_space(14.0);
}

pub fn draw_row(
    ui: &mut egui::Ui,
    state: &mut State,
    label: &str,
    prop: &mut String,
    interactive: bool,
) {
    ui.horizontal(|ui| {
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
                ui.label(egui::RichText::new(label).color(COLOR_TEXT_WHITE));
            },
        );

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
                ui.add_space(10.0);
                let text_edit = egui::TextEdit::singleline(prop)
                    .margin(egui::Vec2::new(if interactive { 16.0 } else { 0.0 }, 0.0))
                    .text_color(egui::Color32::from_rgb(163, 163, 163))
                    .vertical_align(egui::Align::Center)
                    .interactive(interactive)
                    .frame(interactive);

                let r = ui.add(text_edit);
                if r.changed() {
                    // config.write(&state.json.filepath);
                    state.actions.config_edited = true
                };
            },
        );
    });

    ui.add_space(ROW_MARGIN);
}

pub fn format_ms(f: f32) -> String {
    format!("{:06.3}", f)
}
