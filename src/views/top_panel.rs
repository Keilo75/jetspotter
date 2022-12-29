use eframe::egui::{self, Button};

use crate::jetspotter::{AppPanel, AppState};

pub struct TopPanelView;

impl Default for TopPanelView {
    fn default() -> Self {
        Self {}
    }
}

impl super::View for TopPanelView {
    fn ui(&mut self, ui: &mut eframe::egui::Ui, state: &mut AppState) {
        ui.add_space(5.0);
        egui::menu::bar(ui, |ui| {
            ui.heading("Jetspotter");

            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                let theme_btn = ui.add_enabled(
                    state.app_panel != AppPanel::Fetching,
                    Button::new(if state.persistent.dark_mode {
                        "🌞"
                    } else {
                        "🌙"
                    }),
                );

                if theme_btn.clicked() {
                    state.persistent.dark_mode = !state.persistent.dark_mode;
                    state.persistent.save();
                }
            });
        });
        ui.add_space(5.0);
    }
}
