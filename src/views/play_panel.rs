use crate::jetspotter::PersistentData;

pub struct PlayView;

impl Default for PlayView {
    fn default() -> Self {
        Self {}
    }
}

impl super::View for PlayView {
    fn ui(&mut self, persistent: &mut PersistentData, ui: &mut eframe::egui::Ui) {
        ui.heading("Play");
        if ui.button("Play").clicked() {
            println!("Playing");
        }

        ui.label(format!("{} aircraft cached.", persistent.aircraft.len()));
    }
}
