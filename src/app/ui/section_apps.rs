use crate::app::ui::*;
pub fn draw(ui: &mut egui::Ui, state: &mut State, _config: &mut Config) {
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
            ui.heading(egui::RichText::new("Apps").color(COLOR_TEXT_WHITE));
        },
    );

    ui.add_space(SECTION_HEADING_MARGIN);

    ui.add_space(5.0);
    let r = ui.button("build listeners");
    if r.clicked() {
        state.actions.build_listeners = true;
    }

    ui.add_space(5.0);
    let r = ui.button("destroy listeners");
    if r.clicked() {
        state.actions.destroy_listeners = true;
    }
}
