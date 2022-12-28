use eframe::egui;
use eframe::egui::{Context, TopBottomPanel};
use eframe::Frame;

pub struct Jetspotter;

impl Jetspotter {
    pub fn new() -> Self {
        Jetspotter {}
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
                });
            });
        });
    }
}
