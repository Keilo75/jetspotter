use eframe::{
    egui,
    egui::Visuals,
    egui::{CentralPanel, ProgressBar},
};
use poll_promise::Promise;

mod jetphotos;
mod jetspotter;
mod views;
use jetspotter::{AppState, Jetspotter};
use views::View;

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

        if self.persistent.dark_mode {
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
                        if i == 0 && self.persistent.aircraft.len() == 0 {
                            ui.set_enabled(false);
                        }

                        match i {
                            0 => self.views.play.ui(&mut self.persistent, ui),
                            1 => self.render_fetch_aircraft_panel(ui),
                            _ => unreachable!(),
                        };
                    });
                }
            });

            ui.add_space(10.0);
            ui.group(|ui| self.views.statistics.ui(&mut self.persistent, ui));
        });

        if self.state == AppState::Fetching {
            if self.promise.is_none() {
                let (sender, promise) = Promise::new();
                jetphotos::fetch_photos(
                    sender,
                    self.persistent.aircraft.len(),
                    self.persistent.fetch_amount,
                    self.page,
                );

                self.state = AppState::Fetching;
                self.promise = Some(promise);
            }

            CentralPanel::default().show(ctx, |ui| {
                if let Some(promise) = &self.promise {
                    if let Some(result) = promise.ready() {
                        let mut photos = result.clone();
                        self.persistent.aircraft.append(&mut photos);
                        self.promise = None;
                        self.page += 1;

                        if self.persistent.aircraft.len() as i32 == self.persistent.fetch_amount {
                            self.persistent.save();
                            self.state = AppState::Menu;
                        }
                    }

                    ui.horizontal(|ui| {
                        ui.spinner();
                        ui.label("Fetching photos...")
                    });
                    let progress =
                        self.persistent.aircraft.len() as f32 / self.persistent.fetch_amount as f32;

                    ui.add(ProgressBar::new(progress).text(format!(
                        "{}/{}",
                        self.persistent.aircraft.len(),
                        self.persistent.fetch_amount
                    )));
                }
            });
        }
    }
}
