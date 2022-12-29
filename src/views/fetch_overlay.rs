use eframe::egui::ProgressBar;
use poll_promise::Promise;

use crate::{
    jetphotos::{self, aircraft_photo::AircraftPhoto},
    jetspotter::{AppPanel, AppState},
};

pub struct FetchOverlay {
    pub promise: Option<Promise<Vec<AircraftPhoto>>>,
    pub page: i32,
}

impl Default for FetchOverlay {
    fn default() -> Self {
        Self {
            promise: None,
            page: 1,
        }
    }
}

impl super::View for FetchOverlay {
    fn ui(&mut self, ui: &mut eframe::egui::Ui, state: &mut AppState) {
        if self.promise.is_none() {
            let (sender, promise) = Promise::new();
            jetphotos::fetch_photos(
                sender,
                state.persistent.aircraft.len(),
                state.persistent.fetch_amount,
                self.page,
            );

            state.app_panel = AppPanel::Fetching;
            self.promise = Some(promise);
        }

        if let Some(promise) = &self.promise {
            if let Some(result) = promise.ready() {
                let mut photos = result.clone();
                state.persistent.aircraft.append(&mut photos);
                self.promise = None;
                self.page += 1;

                if state.persistent.aircraft.len() as i32 == state.persistent.fetch_amount {
                    state.persistent.save();
                    state.app_panel = AppPanel::Menu;
                }
            }

            ui.horizontal(|ui| {
                ui.spinner();
                ui.label("Fetching photos...")
            });
            let progress =
                state.persistent.aircraft.len() as f32 / state.persistent.fetch_amount as f32;

            ui.add(ProgressBar::new(progress).text(format!(
                "{}/{}",
                state.persistent.aircraft.len(),
                state.persistent.fetch_amount
            )));
        }
    }
}
