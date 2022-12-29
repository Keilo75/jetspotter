use crate::jetspotter::AppState;

pub struct PlayPanel;

impl Default for PlayPanel {
    fn default() -> Self {
        Self {}
    }
}

#[derive(PartialEq)]
pub enum PlayPanelResult {
    None,
    StartGame,
}

impl super::View<PlayPanelResult> for PlayPanel {
    fn ui(&mut self, ui: &mut eframe::egui::Ui, state: &mut AppState) -> PlayPanelResult {
        let mut result = PlayPanelResult::None;

        if ui.button("Play").clicked() {
            result = PlayPanelResult::StartGame;
        }

        ui.label(format!(
            "{} aircraft cached.",
            state.persistent.aircraft.len()
        ));

        result
    }
}
