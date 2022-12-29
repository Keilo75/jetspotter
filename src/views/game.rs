use egui_extras::RetainedImage;
use poll_promise::Promise;

use crate::{
    jetphotos::{self, aircraft_photo::AircraftPhoto},
    jetspotter::AppState,
};

pub struct Game {
    photo: Option<AircraftPhoto>,
    promise: Option<Promise<RetainedImage>>,
}

impl Default for Game {
    fn default() -> Self {
        Self {
            photo: None,
            promise: None,
        }
    }
}

#[derive(PartialEq)]
pub enum GameResult {
    None,
}

impl super::View<GameResult> for Game {
    fn ui(&mut self, ui: &mut eframe::egui::Ui, state: &mut AppState) -> GameResult {
        let result = GameResult::None;
        if self.photo.is_none() {
            let new_photo = state.persistent.aircraft.pop().unwrap();
            let (sender, promise) = Promise::new();

            jetphotos::fetch_photo(sender, new_photo.url.clone());
            self.promise = Some(promise);
            self.photo = Some(new_photo);
        }

        let photo = self.photo.as_ref().unwrap();

        if let Some(promise) = &self.promise {
            if let Some(image) = promise.ready() {
                let size = image.size_vec2();
                let available_size = ui.available_size();
                let max_width = available_size.x;
                let max_height = available_size.y / 2.0;

                let mut img_size = size * (max_width / size.x);
                if img_size.y > max_height {
                    img_size = size * (max_height / size.y)
                }

                ui.vertical_centered(|ui| {
                    image.show_size(ui, img_size);
                });
            } else {
                ui.horizontal(|ui| {
                    ui.spinner();
                    ui.label("Loading image...")
                });
            }
        }

        result
    }
}
