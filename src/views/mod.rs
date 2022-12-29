use crate::jetspotter::PersistentData;

pub mod fetch_panel;
pub mod play_panel;
pub mod statistics_panel;
pub mod top_panel;

pub struct Views {
    pub statistics_panel: statistics_panel::StatisticsView,
    pub play_panel: play_panel::PlayView,
    pub top_panel: top_panel::TopPanelView,
    pub fetch_panel: fetch_panel::FetchPanel,
}

impl Default for Views {
    fn default() -> Self {
        Views {
            statistics_panel: statistics_panel::StatisticsView::default(),
            play_panel: play_panel::PlayView::default(),
            top_panel: top_panel::TopPanelView::default(),
            fetch_panel: fetch_panel::FetchPanel::default(),
        }
    }
}

pub trait View {
    fn ui(&mut self, persistent: &mut PersistentData, ui: &mut eframe::egui::Ui);
}
