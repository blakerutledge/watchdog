use crate::app::ui::*;
pub fn draw(ui: &mut egui::Ui, _state: &mut State, _config: &mut Config) {
    // Header
    ui.heading(egui::RichText::new("Apps").color(COLOR_TEXT_WHITE));

    ui.add_space(SECTION_HEADING_MARGIN);
}
