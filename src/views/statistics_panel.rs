use crate::jetspotter::{AppState, Results};

pub struct StatisticsView;

impl Default for StatisticsView {
    fn default() -> Self {
        Self {}
    }
}

impl super::View for StatisticsView {
    fn ui(&mut self, ui: &mut eframe::egui::Ui, state: &mut AppState) {
        ui.heading("Statistics");

        ui.horizontal(|ui| {
            let Results {
                games_played,
                games_won,
                ..
            } = state.persistent.results;

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
