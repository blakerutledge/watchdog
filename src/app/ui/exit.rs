use crate::app::state::State;
// use crate::app::ui::*;

pub fn draw(context: &egui::Context, state: &mut State) {
    // Set to false at the beginning of every frame
    state.ui.exit_tooltip_clickout = false;

    // TO DO: totally change how this looks
    // TO DO: hook up to real actions
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
                    state.ui.exit_tooltip_clickout = true;
                    state.ui.show_exit_tooltip = false;
                }
            }
        });
}
