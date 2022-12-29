use crate::jetspotter::{PersistentData, Results};

pub struct Statistics;

impl Default for Statistics {
    fn default() -> Self {
        Self {}
    }
}

impl super::View for Statistics {
    fn ui(&mut self, persistent: &mut PersistentData, ui: &mut eframe::egui::Ui) {
        ui.heading("Statistics");

        ui.horizontal(|ui| {
            let Results {
                games_played,
                games_won,
                ..
            } = persistent.results;

            let win_rate = if games_played == 0 {
                0.0
            } else {
                games_won as f32 / games_played as f32
            };

            ui.label(format!(
                "Games played: {} | Games won: {} | Win rate: {}%",
                games_played, games_won, win_rate
            ));
        });

        ui.separator();
    }
}
