use crate::jetspotter::PersistentData;

pub mod play;
pub mod statistics;

pub struct Views {
    pub statistics: statistics::StatisticsView,
    pub play: play::PlayView,
}

impl Default for Views {
    fn default() -> Self {
        Views {
            statistics: statistics::StatisticsView::default(),
            play: play::PlayView::default(),
        }
    }
}

pub trait View {
    fn ui(&mut self, persistent: &mut PersistentData, ui: &mut eframe::egui::Ui);
}
