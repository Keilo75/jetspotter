use crate::jetspotter::AppState;

pub struct PlayView;

impl Default for PlayView {
    fn default() -> Self {
        Self {}
    }
}

impl super::View for PlayView {
    fn ui(&mut self, ui: &mut eframe::egui::Ui, state: &mut AppState) {
        ui.heading("Play");
        if ui.button("Play").clicked() {
            println!("Playing");
        }

        ui.label(format!(
            "{} aircraft cached.",
            state.persistent.aircraft.len()
        ));
    }
}
