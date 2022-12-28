use eframe::{egui, egui::Visuals};
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
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        ctx.request_repaint();

        if self.config.dark_mode {
            ctx.set_visuals(Visuals::dark());
        } else {
            ctx.set_visuals(Visuals::light());
        }

        self.render_top_panel(ctx, frame);
    }
}
