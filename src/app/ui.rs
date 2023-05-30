use super::state::State;

pub fn init() -> Box<dyn FnMut(&egui::Context, &mut State)> {
    Box::new(|context: &egui::Context, state: &mut State| {
        egui::CentralPanel::default().show(context, |ui| {
            // println!("WAT");
            //
            // User Interface defined here
            //
            ui.heading("Watchdog");

            /*
            ui.horizontal(|ui| {
                let name_label = ui.label("Your name: ");
                ui.text_edit_singleline(&mut state.name)
                    .labelled_by(name_label.id);
            });

            ui.add(egui::Slider::new(&mut state.age, 0..=120).text("age"));

            if ui.button("Click each year").clicked() {
                state.age += 1;
            }

            */

            ui.label(format!("FPS: {}", state.perf.fps));

            // If there is only one frame in the list, it is partilly completed,
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

            if ui.button("Exit").clicked() {
                state.actions.app_exit = true
            }
        });
    })
}

fn format_ms(f: f32) -> String {
    format!("{:06.3}", f)
}
