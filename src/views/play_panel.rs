use crate::jetspotter::AppState;

pub struct PlayPanel;

impl Default for PlayPanel {
    fn default() -> Self {
        Self {}
    }
}

impl super::View for PlayPanel {
    fn ui(&mut self, ui: &mut eframe::egui::Ui, state: &mut AppState) {
        if ui.button("Play").clicked() {
            println!("Playing");
        }

        ui.label(format!(
            "{} aircraft cached.",
            state.persistent.aircraft.len()
        ));
    }
}
