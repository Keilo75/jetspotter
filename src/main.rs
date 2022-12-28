use eframe::{
    egui,
    egui::Visuals,
    egui::{CentralPanel, ProgressBar},
};

mod jetspotter;
use jetspotter::{AppState, Jetspotter};

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

        let is_fetching = self.state == AppState::Fetching;

        CentralPanel::default().show(ctx, |ui| {
            ui.columns(2, |cols| {
                for (i, col) in cols.iter_mut().enumerate() {
                    col.set_enabled(!is_fetching);

                    col.group(|ui| {
                        if i == 0 && (self.aircraft.is_none()) {
                            ui.set_enabled(false);
                        }

                        match i {
                            0 => self.render_play_panel(ui),
                            1 => self.render_fetch_aircraft_panel(ui),
                            _ => unreachable!(),
                        };
                    });
                }
            });

            if is_fetching {
                ui.horizontal(|ui| {
                    ui.spinner();
                    ui.label("Fetching photos...")
                });

                ui.add(ProgressBar::new(0.5).text("50/100"));
            }
        });
    }
}
