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
            ui.label(format!("FPS: {}", state.frames_per_second));

            if state.frames.len() >= 2 {
                // Render previous frame's stats, this frame
                let i = state.frames.len() - 2;
                let diff = state.frames[i].stop.checked_sub(state.frames[i].start);
                match diff {
                    Some(diff) => {
                        let ft_f = diff.as_nanos() as f32 / 1e6;
                        ui.label(format!("Frame Time: {} ms", format_ms(ft_f)));
                    }
                    _ => {
                        ui.label(format!("Frame Time: x.x ms"));
                    }
                }
            }

            ui.label(format!(
                "Avg Frame Time: {}ms",
                format_ms(state.avg_frame_time)
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
