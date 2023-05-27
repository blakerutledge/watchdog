use super::state::State;

pub fn init() -> Box<dyn FnMut(&egui::Context, &mut State)> {
    let draw_ui = Box::new(|context: &egui::Context, state: &mut State| {
        egui::CentralPanel::default().show(context, |ui| {
            ui.heading("My egui Application Yo yo");
            ui.horizontal(|ui| {
                let name_label = ui.label("Your name: ");
                ui.text_edit_singleline(&mut state.name)
                    .labelled_by(name_label.id);
            });
            ui.add(egui::Slider::new(&mut state.age, 0..=120).text("age"));
            if ui.button("Click each year").clicked() {
                state.age += 1;
            }
            ui.label(format!("Hello '{}', age {}", &state.name, &state.age,));
        });
    });

    draw_ui
}
