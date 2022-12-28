use eframe::egui;
use eframe::egui::{Context, TopBottomPanel};
use eframe::Frame;

pub struct JetspotterConfig {
    pub dark_mode: bool,
}

impl Default for JetspotterConfig {
    fn default() -> Self {
        Self { dark_mode: true }
    }
}

pub struct Jetspotter {
    pub config: JetspotterConfig,
}

impl Jetspotter {
    pub fn new() -> Self {
        Jetspotter {
            config: Default::default(),
        }
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
                        self.config.dark_mode = !self.config.dark_mode;
                    }
                });
            });
        });
    }
}
