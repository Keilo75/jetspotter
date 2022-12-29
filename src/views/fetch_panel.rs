use eframe::egui::DragValue;

use crate::jetspotter::PersistentData;

pub struct FetchPanel;

impl Default for FetchPanel {
    fn default() -> Self {
        Self {}
    }
}

impl super::View for FetchPanel {
    fn ui(&mut self, persistent: &mut PersistentData, ui: &mut eframe::egui::Ui) {
        ui.label("Fetch Amount");
        let fetch_amount_input = ui.add(DragValue::new(&mut persistent.fetch_amount));
        if fetch_amount_input.lost_focus() || fetch_amount_input.drag_released() {
            persistent.save();
        }

        ui.horizontal(|ui| {
            ui.set_enabled(persistent.fetch_amount >= 0);

            let fetch_photos_btn = ui.button("Fetch photos");
            if fetch_photos_btn.clicked() {
                // self.state = AppState::Fetching;
                // self.page = 1;

                persistent.aircraft.clear();
                persistent.save();
            }

            ui.label("This may take a while.");
        });
    }
}
