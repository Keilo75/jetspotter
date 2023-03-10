use eframe::{egui::Layout, emath::Align, epaint::Color32};
use egui_extras::RetainedImage;
use poll_promise::Promise;
use strum::IntoEnumIterator;

use crate::{
    jetphotos::{
        self,
        aircraft_photo::{AircraftKind, AircraftPhoto},
    },
    jetspotter::AppState,
};

pub struct Game {
    photo: Option<AircraftPhoto>,
    promise: Option<Promise<RetainedImage>>,
    guess: Option<AircraftKind>,
}

impl Default for Game {
    fn default() -> Self {
        Self {
            photo: None,
            promise: None,
            guess: None,
        }
    }
}

#[derive(PartialEq)]
pub enum GameResult {
    None,
    Exit,
    NextPhoto,
}

impl Game {
    pub fn start_game(&mut self) {
        self.photo = None;
        self.promise = None;
        self.guess = None;
    }
}

impl super::View<GameResult> for Game {
    fn ui(&mut self, ui: &mut eframe::egui::Ui, state: &mut AppState) -> GameResult {
        let mut result = GameResult::None;
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
                let max_height = available_size.y / 1.5;

                let mut img_size = size * (max_width / size.x);
                if img_size.y > max_height {
                    img_size = size * (max_height / size.y)
                }

                ui.vertical_centered(|ui| {
                    ui.set_max_width(img_size.x.clone());
                    image.show_size(ui, img_size);
                    ui.add_space(5.0);

                    if let Some(guess) = &self.guess {
                        ui.label(&photo.full_kind);
                        ui.label(format!("You guessed: {}", guess.to_string()));

                        let is_correct_guess = guess == &photo.kind;
                        if is_correct_guess {
                            ui.colored_label(Color32::LIGHT_GREEN, "That's correct!");
                        } else {
                            ui.colored_label(Color32::LIGHT_RED, "That's incorrect!");
                        }
                    } else {
                        ui.horizontal_wrapped(|ui| {
                            for kind in AircraftKind::iter() {
                                if ui.button(kind.to_string()).clicked() {
                                    self.guess = Some(kind.clone());
                                    let is_correct_guess = kind == photo.kind;

                                    let index = state
                                        .persistent
                                        .results
                                        .aircraft_results
                                        .iter()
                                        .position(|r| r.aircraft == photo.kind)
                                        .unwrap();

                                    state.persistent.results.games_played += 1;
                                    let aircraft =
                                        &mut state.persistent.results.aircraft_results[index];
                                    aircraft.games_played += 1;

                                    if is_correct_guess {
                                        state.persistent.results.games_won += 1;
                                        aircraft.games_won += 1
                                    } else {
                                        let kind = kind.to_string();

                                        aircraft.misses.insert(
                                            kind.clone(),
                                            aircraft
                                                .misses
                                                .get(&kind)
                                                .and_then(|m| Some(m + 1))
                                                .unwrap_or(1),
                                        );
                                    }

                                    state.persistent.save();
                                }
                            }
                        });
                    }

                    ui.separator();

                    ui.horizontal(|ui| {
                        ui.label(format!(
                            "{} photos left",
                            state.persistent.aircraft.len() + 1
                        ));

                        if self.guess.is_some() {
                            ui.with_layout(Layout::right_to_left(Align::TOP), |ui| {
                                if ui.button("Exit").clicked() {
                                    result = GameResult::Exit;
                                }

                                if state.persistent.aircraft.len() > 0
                                    && ui.button("Next photo").clicked()
                                {
                                    result = GameResult::NextPhoto;
                                }
                            });
                        }
                    });
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
