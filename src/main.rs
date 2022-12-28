use eframe::{egui, egui::CentralPanel, egui::Visuals};

mod aircraft;
mod jetspotter;
use jetspotter::Jetspotter;

fn main() {
    let native_options = eframe::NativeOptions::default();
    eframe::run_native(
        "Airplane ID Game",
        native_options,
        Box::new(|_cc| Box::new(Jetspotter::new())),
    );
}

impl eframe::App for Jetspotter {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        ctx.request_repaint();

        if self.config.dark_mode {
            ctx.set_visuals(Visuals::dark());
        } else {
            ctx.set_visuals(Visuals::light());
        }

        self.render_top_panel(ctx);

        CentralPanel::default().show(ctx, |ui| {
            ui.heading("whatsup");
        });
    }
}
