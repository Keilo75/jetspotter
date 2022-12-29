use eframe::{
    egui,
    egui::{CentralPanel, ProgressBar},
    egui::{TopBottomPanel, Visuals},
};
use poll_promise::Promise;

mod jetphotos;
mod jetspotter;
mod views;
use jetspotter::{AppPanel, Jetspotter};
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

        if self.state.persistent.dark_mode {
            ctx.set_visuals(Visuals::dark());
        } else {
            ctx.set_visuals(Visuals::light());
        }

        TopBottomPanel::top("top").show(ctx, |ui| {
            self.views.top_panel.ui(ui, &mut self.state);
        });

        CentralPanel::default().show(ctx, |ui| {
            ui.set_enabled(self.state.app_panel != AppPanel::Fetching);

            ui.columns(2, |cols| {
                for (i, col) in cols.iter_mut().enumerate() {
                    col.group(|ui| {
                        if i == 0 && self.state.persistent.aircraft.len() == 0 {
                            ui.set_enabled(false);
                        }

                        match i {
                            0 => self.views.play_panel.ui(ui, &mut self.state),
                            1 => self.views.fetch_panel.ui(ui, &mut self.state),
                            _ => unreachable!(),
                        };
                    });
                }
            });

            ui.add_space(10.0);
            ui.group(|ui| self.views.statistics_panel.ui(ui, &mut self.state));
        });

        if self.state.app_panel == AppPanel::Fetching {
            if self.promise.is_none() {
                let (sender, promise) = Promise::new();
                jetphotos::fetch_photos(
                    sender,
                    self.state.persistent.aircraft.len(),
                    self.state.persistent.fetch_amount,
                    self.page,
                );

                self.state.app_panel = AppPanel::Fetching;
                self.promise = Some(promise);
            }

            CentralPanel::default().show(ctx, |ui| {
                if let Some(promise) = &self.promise {
                    if let Some(result) = promise.ready() {
                        let mut photos = result.clone();
                        self.state.persistent.aircraft.append(&mut photos);
                        self.promise = None;
                        self.page += 1;

                        if self.state.persistent.aircraft.len() as i32
                            == self.state.persistent.fetch_amount
                        {
                            self.state.persistent.save();
                            self.state.app_panel = AppPanel::Menu;
                        }
                    }

                    ui.horizontal(|ui| {
                        ui.spinner();
                        ui.label("Fetching photos...")
                    });
                    let progress = self.state.persistent.aircraft.len() as f32
                        / self.state.persistent.fetch_amount as f32;

                    ui.add(ProgressBar::new(progress).text(format!(
                        "{}/{}",
                        self.state.persistent.aircraft.len(),
                        self.state.persistent.fetch_amount
                    )));
                }
            });
        }
    }
}
