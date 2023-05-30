use super::config::Config;
use super::state::State;

pub fn init() -> Box<dyn FnMut(&egui::Context, &mut State, &Config)> {
    Box::new(
        |context: &egui::Context, state: &mut State, config: &Config| {
            egui::CentralPanel::default().show(context, |ui| {
                //
                // User Interface defined here
                //
                ui.heading("Watchdog");

                //
                // host perf
                //
                ui.add_space(6.0);
                ui.strong("Host Performance");
                ui.label(format!("FPS: {}", state.perf.fps));

                // If there is only one frame in the list, it is partially completed,
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

                //
                // config
                //
                ui.add_space(6.0);
                ui.strong("Json");

                if ui.button("Open file…").clicked() {
                    if let Some(path) = rfd::FileDialog::new().pick_file() {
                        state.json.filepath = Some(path.display().to_string());
                    }
                }

                if let Some(picked_path) = &state.json.filepath {
                    ui.horizontal(|ui| {
                        ui.label("Picked file:");
                        ui.monospace(picked_path);
                    });
                }

                /*
                    // Show dropped files (if any):
                    if !self.dropped_files.is_empty() {
                        ui.group(|ui| {
                            ui.label("Dropped files:");

                            for file in &self.dropped_files {
                                let mut info = if let Some(path) = &file.path {
                                    path.display().to_string()
                                } else if !file.name.is_empty() {
                                    file.name.clone()
                                } else {
                                    "???".to_owned()
                                };
                                if let Some(bytes) = &file.bytes {
                                    use std::fmt::Write as _;
                                    write!(info, " ({} bytes)", bytes.len()).ok();
                                }
                                ui.label(info);
                            }
                        });
                    }
                });

                */

                ui.checkbox(&mut state.json.parsed, "Parsed");

                if ui.button("Quit Watchdog").clicked() {
                    state.actions.app_exit = true
                }

                if ui.button("Abort Watched Task").clicked() {
                    state.actions.app_exit = true
                }

                if ui.button("Quit Watchdog and Abort Watched Task").clicked() {
                    state.actions.app_exit = true
                }
            });
        },
    )
}

fn format_ms(f: f32) -> String {
    format!("{:06.3}", f)
}
