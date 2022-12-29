use eframe::egui::{menu, Align, Button, Layout};

use crate::jetspotter::{AppPanel, AppState};

pub struct TopPanel;

impl Default for TopPanel {
    fn default() -> Self {
        Self {}
    }
}

impl super::View for TopPanel {
    fn ui(&mut self, ui: &mut eframe::egui::Ui, state: &mut AppState) {
        ui.add_space(5.0);
        menu::bar(ui, |ui| {
            ui.heading("Jetspotter");

            ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
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
