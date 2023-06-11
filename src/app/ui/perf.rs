use crate::app::ui::*;

pub fn draw(context: &egui::Context, state: &mut State) {
    //
    // Show debug statistics relevant to the watchdog host application performance
    //
    if SHOW_HOST_DEV {
        egui::Window::new("Host Performance")
            .resizable(false)
            .movable(true)
            .collapsible(false)
            .title_bar(true)
            .pivot(egui::Align2::RIGHT_BOTTOM)
            .default_pos(egui::Pos2::new(99999.0, 9999999.0))
            .show(context, |ui| {
                //
                // Frames per second
                ui.add_space(6.0);
                ui.label(format!("FPS: {}", state.perf.fps));

                //
                // Frametime
                //
                // If there is only one frame in the list, it is partially completed,
                // and does not yet have a frame.stop time that we can use. The first frame is
                // currently being rendered. We can only show stats for the previous frame!
                let mut frametime = components::format_ms(0.0);
                if state.perf.frames.len() >= 2 {
                    let f = &state.perf.frames[state.perf.frames.len() - 2];
                    let diff = f.stop.checked_sub(f.start);
                    match diff {
                        Some(diff) => {
                            frametime = components::format_ms(diff.as_nanos() as f32 / 1e6);
                        }
                        _ => {}
                    }
                }
                ui.label(format!("Frame Time: {} ms", frametime));

                //
                // Average frame time
                // Calculated in the app/perf.rs module
                ui.label(format!(
                    "Avg Frame Time: {}ms",
                    components::format_ms(state.perf.avg_frame_time)
                ));
            });
    }
}
