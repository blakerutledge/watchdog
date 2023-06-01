use egui::Vec2;

use super::config::Config;
use super::state::State;

pub fn init() -> Box<dyn FnMut(&egui::Context, &mut State, &Config)> {
    // all images embedded in binary
    let icon_config = include_bytes!("../../assets/icons/icon-config.png");
    let icon_play = include_bytes!("../../assets/icons/icon-play.png");
    let icon_stats = include_bytes!("../../assets/icons/icon-stats.png");
    let icon_exit = include_bytes!("../../assets/icons/icon-exit.png");

    Box::new(
        |context: &egui::Context, state: &mut State, config: &Config| {
            {
                // load any missing images
                if !state.ui.textures.contains_key("icon_config") {
                    create_tex(icon_config, "icon_config", context, state);
                    println!("created icon_config texture");
                }

                if !state.ui.textures.contains_key("icon_play") {
                    create_tex(icon_play, "icon_play", context, state);
                    println!("created icon_play texture");
                }

                if !state.ui.textures.contains_key("icon_stats") {
                    create_tex(icon_stats, "icon_stats", context, state);
                    println!("created icon_stats texture");
                }

                if !state.ui.textures.contains_key("icon_exit") {
                    create_tex(icon_exit, "icon_exit", context, state);
                    println!("created icon_exit texture");
                };

                /*
                   add more here
                */
            }

            egui::TopBottomPanel::top("my_panel").show(context, |ui| {
                let icon_config = state.ui.textures.get("icon_config").unwrap();
                let icon_play = state.ui.textures.get("icon_play").unwrap();
                let icon_stats = state.ui.textures.get("icon_stats").unwrap();
                let icon_exit = state.ui.textures.get("icon_exit").unwrap();

                ui.add(egui::ImageButton::new(
                    &icon_config.1,
                    Vec2::new(40.0, 40.0),
                ));
                ui.add(egui::ImageButton::new(&icon_play.1, Vec2::new(40.0, 40.0)));
                ui.add(egui::ImageButton::new(&icon_stats.1, Vec2::new(40.0, 40.0)));
                ui.add(egui::ImageButton::new(&icon_exit.1, Vec2::new(40.0, 40.0)));
            });

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

fn draw_header(ui: &mut egui::Ui, state: &mut State, config: &Config) {}

fn create_tex(image_data: &[u8], slug: &str, context: &egui::Context, state: &mut State) {
    let image = image::load_from_memory(image_data)
        .expect(format!("Failed to load image {}", slug).as_str());
    let size = [image.width() as _, image.height() as _];
    let image_buffer = image.to_rgba8();
    let pixels = image_buffer.as_flat_samples();

    let i = egui::ColorImage::from_rgba_unmultiplied(size, pixels.as_slice());

    state.ui.textures.insert(
        slug.to_string(),
        (
            egui::Vec2::new(size[0] as f32, size[1] as f32),
            context.load_texture(slug, i, Default::default()),
        ),
    );
}
