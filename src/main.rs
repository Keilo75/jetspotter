use eframe::egui;
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

        self.render_top_panel(ctx, frame);
    }
}
