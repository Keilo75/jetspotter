use eframe::egui;
use eframe::egui::{Context, TopBottomPanel};
use eframe::Frame;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Serialize, Deserialize)]
pub struct JetspotterConfig {
    pub dark_mode: bool,
    pub photo_dir: PathBuf,
}

impl JetspotterConfig {
    fn set_dark_mode(&mut self, dark_mode: bool) {
        self.dark_mode = dark_mode;
        self.save();
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
        }
    }
}

pub struct Jetspotter {
    pub config: JetspotterConfig,
}

impl Jetspotter {
    pub fn new() -> Self {
        let config: JetspotterConfig = confy::load("jetspotter", None).unwrap_or_default();
        Jetspotter { config }
    }

    pub fn name(&self) -> String {
        "Jetspotter".into()
    }
    pub fn render_top_panel(&mut self, ctx: &Context, frame: &mut Frame) {
        TopBottomPanel::top("top_panel").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                ui.heading(self.name());

                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    if ui.button("❌").clicked() {
                        frame.close();
                    }

                    let theme_btn = ui.button(if self.config.dark_mode {
                        "🌞"
                    } else {
                        "🌙"
                    });

                    if theme_btn.clicked() {
                        self.config.set_dark_mode(!self.config.dark_mode);
                    }
                });
            });
        });
    }
}
