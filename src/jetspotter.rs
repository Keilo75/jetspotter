use eframe::egui::{self, DragValue, Ui};
use eframe::egui::{Context, TopBottomPanel};
use poll_promise::Promise;
use serde::{Deserialize, Serialize};

use crate::jetphotos::AircraftPhoto;

#[derive(Serialize, Deserialize)]
pub struct PersistentData {
    pub dark_mode: bool,
    pub fetch_amount: i32,
    pub aircraft: Vec<AircraftPhoto>,
}

impl PersistentData {
    fn load() -> Self {
        let config: Self = confy::load("jetspotter", None).unwrap_or_default();
        config
    }

    pub fn save(&mut self) {
        confy::store("jetspotter", None, self).unwrap();
    }
}

impl Default for PersistentData {
    fn default() -> Self {
        Self {
            dark_mode: true,
            aircraft: Vec::new(),
            fetch_amount: 100,
        }
    }
}

#[derive(PartialEq, Debug)]
pub enum AppState {
    Menu,
    Fetching,
}

pub struct Jetspotter {
    pub persistent: PersistentData,
    pub state: AppState,
    pub promise: Option<Promise<Vec<AircraftPhoto>>>,
    pub page: i32,
}

impl Jetspotter {
    pub fn new() -> Self {
        let persistent = PersistentData::load();

        Jetspotter {
            persistent,
            state: AppState::Menu,
            promise: None,
            page: 1,
        }
    }

    pub fn render_top_panel(&mut self, ctx: &Context) {
        TopBottomPanel::top("top").show(ctx, |ui| {
            ui.add_space(5.0);
            egui::menu::bar(ui, |ui| {
                ui.heading("Jetspotter");

                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    let theme_btn = ui.button(if self.persistent.dark_mode {
                        "🌞"
                    } else {
                        "🌙"
                    });

                    if theme_btn.clicked() {
                        self.persistent.dark_mode = !self.persistent.dark_mode;
                        self.persistent.save();
                    }
                });
            });
            ui.add_space(5.0);
        });
    }

    pub fn render_play_panel(&mut self, ui: &mut Ui) {
        ui.heading("Play");
        if ui.button("Play").clicked() {
            println!("Playing");
        }

        ui.label(format!(
            "{} aircraft cached.",
            self.persistent.aircraft.len()
        ));
    }

    pub fn render_fetch_aircraft_panel(&mut self, ui: &mut Ui) {
        ui.label("Fetch Amount");
        let fetch_amount_input = ui.add(DragValue::new(&mut self.persistent.fetch_amount));
        if fetch_amount_input.lost_focus() || fetch_amount_input.drag_released() {
            self.persistent.save();
        }
        ui.horizontal(|ui| {
            ui.set_enabled(self.persistent.fetch_amount >= 0);

            let fetch_photos_btn = ui.button("Fetch photos");
            if fetch_photos_btn.clicked() {
                self.state = AppState::Fetching;
                self.page = 1;

                self.persistent.aircraft.clear();
                self.persistent.save();
            }

            ui.label("This may take a while.");
        });
    }

    pub fn render_statistics_panel(&mut self, ui: &mut Ui) {
        ui.heading("Statistics");
    }
}
