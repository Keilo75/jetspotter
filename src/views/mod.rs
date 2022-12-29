use crate::jetspotter::PersistentData;

pub mod play;
pub mod statistics;
pub mod top_panel;

pub struct Views {
    pub statistics: statistics::StatisticsView,
    pub play: play::PlayView,
    pub top_panel: top_panel::TopPanelView,
}

impl Default for Views {
    fn default() -> Self {
        Views {
            statistics: statistics::StatisticsView::default(),
            play: play::PlayView::default(),
            top_panel: top_panel::TopPanelView::default(),
        }
    }
}

pub trait View {
    fn ui(&mut self, persistent: &mut PersistentData, ui: &mut eframe::egui::Ui);
}
