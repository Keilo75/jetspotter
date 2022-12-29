use eframe::egui;

use crate::jetspotter::PersistentData;

pub struct TopPanelView;

impl Default for TopPanelView {
    fn default() -> Self {
        Self {}
    }
}

impl super::View for TopPanelView {
    fn ui(&mut self, persistent: &mut PersistentData, ui: &mut eframe::egui::Ui) {
        ui.add_space(5.0);
        egui::menu::bar(ui, |ui| {
            ui.heading("Jetspotter");

            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                let theme_btn = ui.button(if persistent.dark_mode { "🌞" } else { "🌙" });

                if theme_btn.clicked() {
                    persistent.dark_mode = !persistent.dark_mode;
                    persistent.save();
                }
            });
        });
        ui.add_space(5.0);
    }
}
