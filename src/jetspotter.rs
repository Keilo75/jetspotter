use crate::aircraft::Aircraft;
use eframe::egui::{self, DragValue, Ui};
use eframe::egui::{Context, TopBottomPanel};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Serialize, Deserialize)]
pub struct JetspotterConfig {
    pub dark_mode: bool,
    pub photo_dir: PathBuf,
    pub fetch_amount: i32,
}

impl JetspotterConfig {
    fn load() -> Self {
        let config: Self = confy::load("jetspotter", None).unwrap_or_default();
        config
    }

    fn save(&mut self) {
        confy::store("jetspotter", None, self).unwrap();
    }
}

impl Default for JetspotterConfig {
    fn default() -> Self {
        let config_path = confy::get_configuration_file_path("jetspotter", None).unwrap();
        let photo_dir = config_path.parent().unwrap().join("photos");

        Self {
            dark_mode: true,
            photo_dir,
            fetch_amount: 100,
        }
    }
}

pub struct Jetspotter {
    pub config: JetspotterConfig,
    pub aircraft: Option<Vec<Aircraft>>,
}

impl Jetspotter {
    pub fn new() -> Self {
        Jetspotter {
            config: JetspotterConfig::load(),
            aircraft: None,
        }
    }

    pub fn render_top_panel(&mut self, ctx: &Context) {
        TopBottomPanel::top("top").show(ctx, |ui| {
            ui.add_space(5.0);
            egui::menu::bar(ui, |ui| {
                ui.heading("Jetspotter");

                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    let theme_btn = ui.button(if self.config.dark_mode {
                        "🌞"
                    } else {
                        "🌙"
                    });

                    if theme_btn.clicked() {
                        self.config.dark_mode = !self.config.dark_mode;
                        self.config.save();
                    }
                });
            });
            ui.add_space(5.0);
        });
    }

    pub fn render_play_panel(&mut self, ui: &mut Ui) {
        ui.heading("Play");
    }

    pub fn render_fetch_aircraft_panel(&mut self, ui: &mut Ui) {
        ui.heading(if self.aircraft.is_some() {
            "Aircraft found!"
        } else {
            "No aircraft found!"
        });

        ui.label("Fetch Amount");
        let fetch_amount_input = ui.add(DragValue::new(&mut self.config.fetch_amount));
        if fetch_amount_input.lost_focus() || fetch_amount_input.drag_released() {
            self.config.save();
        }
        ui.horizontal(|ui| {
            let is_enabled = self.config.fetch_amount >= 0;
            ui.set_enabled(is_enabled);

            let fetch_photos_btn = ui.button("Fetch photos");
            if fetch_photos_btn.clicked() {
                println!("hi");
            }

            ui.label("This may take a while.");
        });
    }
}
