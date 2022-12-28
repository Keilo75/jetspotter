use eframe::{
    egui,
    egui::Visuals,
    egui::{CentralPanel, ProgressBar},
};
use poll_promise::Promise;

mod jetphotos;
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

        CentralPanel::default().show(ctx, |ui| {
            ui.set_enabled(self.state != AppState::Fetching);

            ui.columns(2, |cols| {
                for (i, col) in cols.iter_mut().enumerate() {
                    col.group(|ui| {
                        if i == 0 && self.aircraft.len() == 0 {
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
        });

        if self.state == AppState::Fetching {
            if self.promise.is_none() {
                let (sender, promise) = Promise::new();

                let current = self.aircraft.len() as i32;
                let total = self.config.fetch_amount;
                jetphotos::fetch_photos(sender, current, total, self.page);

                self.state = AppState::Fetching;
                self.promise = Some(promise);
            }

            CentralPanel::default().show(ctx, |ui| {
                if let Some(promise) = &self.promise {
                    if let Some(result) = promise.ready() {
                        match result {
                            Ok(photos) => {
                                let mut photos = photos.clone();
                                self.aircraft.append(&mut photos);
                                self.promise = None;
                                self.page += 1;

                                if self.aircraft.len() as i32 == self.config.fetch_amount {
                                    self.state = AppState::Menu;
                                }
                            }
                            Err(error) => {
                                panic!("Could not fetch: {}", error)
                            }
                        }
                    }

                    ui.horizontal(|ui| {
                        ui.spinner();
                        ui.label("Fetching photos...")
                    });
                    let progress = self.aircraft.len() as f32 / self.config.fetch_amount as f32;

                    ui.add(ProgressBar::new(progress).text(format!(
                        "{}/{}",
                        self.aircraft.len(),
                        self.config.fetch_amount
                    )));
                }
            });
        }
    }
}
