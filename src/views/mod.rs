use crate::jetspotter::AppState;

pub mod fetch_overlay;
pub mod fetch_panel;
pub mod play_panel;
pub mod statistics_panel;
pub mod top_panel;

pub struct Views {
    pub statistics_panel: statistics_panel::StatisticsPanel,
    pub play_panel: play_panel::PlayPanel,
    pub top_panel: top_panel::TopPanel,
    pub fetch_panel: fetch_panel::FetchPanel,
    pub fetch_overlay: fetch_overlay::FetchOverlay,
}

impl Default for Views {
    fn default() -> Self {
        Views {
            statistics_panel: statistics_panel::StatisticsPanel::default(),
            play_panel: play_panel::PlayPanel::default(),
            top_panel: top_panel::TopPanel::default(),
            fetch_panel: fetch_panel::FetchPanel::default(),
            fetch_overlay: fetch_overlay::FetchOverlay::default(),
        }
    }
}

pub trait View<T = ()> {
    fn ui(&mut self, ui: &mut eframe::egui::Ui, state: &mut AppState) -> T;
}
