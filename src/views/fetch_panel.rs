use eframe::egui::DragValue;

use crate::jetspotter::AppState;

pub struct FetchPanel {}

impl Default for FetchPanel {
    fn default() -> Self {
        Self {}
    }
}

impl super::View for FetchPanel {
    fn ui(&mut self, ui: &mut eframe::egui::Ui, state: &mut AppState) {
        ui.label("Fetch Amount");
        let fetch_amount_input = ui.add(DragValue::new(&mut state.persistent.fetch_amount));
        if fetch_amount_input.lost_focus() || fetch_amount_input.drag_released() {
            state.persistent.save();
        }

        ui.horizontal(|ui| {
            ui.set_enabled(state.persistent.fetch_amount >= 0);

            let fetch_photos_btn = ui.button("Fetch photos");
            if fetch_photos_btn.clicked() {
                // self.state = AppState::Fetching;
                // self.page = 1;

                state.persistent.aircraft.clear();
                state.persistent.save();
            }

            ui.label("This may take a while.");
        });
    }
}
