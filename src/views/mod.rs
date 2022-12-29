use crate::jetspotter::PersistentData;

pub mod statistics;

pub struct Views {
    pub statistics: statistics::Statistics,
}

impl Default for Views {
    fn default() -> Self {
        Views {
            statistics: statistics::Statistics::default(),
        }
    }
}

pub trait View {
    fn ui(&mut self, persistent: &mut PersistentData, ui: &mut eframe::egui::Ui);
}
