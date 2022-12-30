#![windows_subsystem = "windows"]

use eframe::{
    egui,
    egui::{CentralPanel, RichText},
    egui::{TopBottomPanel, Visuals},
};

mod jetphotos;
mod jetspotter;
mod views;
use jetspotter::{AppPanel, Jetspotter};
use views::{fetch_panel::FetchPanelResult, game::GameResult, play_panel::PlayPanelResult, View};

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

        if self.state.app_panel == AppPanel::Menu {
            CentralPanel::default().show(ctx, |ui| {
                ui.label(RichText::new("Play").heading().strong());
                ui.columns(2, |cols| {
                    cols[0].group(|ui| {
                        if self.state.persistent.aircraft.len() == 0 {
                            ui.set_enabled(false);
                        }

                        let play_panel_result = self.views.play_panel.ui(ui, &mut self.state);
                        if play_panel_result == PlayPanelResult::StartGame {
                            self.state.app_panel = AppPanel::Game;
                            self.views.game.start_game();
                        }
                    });

                    cols[1].group(|ui| {
                        let fetch_panel_result = self.views.fetch_panel.ui(ui, &mut self.state);
                        if fetch_panel_result == FetchPanelResult::StartFetch {
                            self.state.app_panel = AppPanel::Fetching;
                            self.state.persistent.aircraft.clear();
                            self.state.persistent.save();
                            self.views.fetch_overlay.page = 1;
                            self.views.fetch_overlay.promise = None;
                        }
                    });
                });

                ui.add_space(10.0);

                if self.state.persistent.results.games_played != 0 {
                    self.views.statistics_panel.ui(ui, &mut self.state);
                }
            });
        }

        if self.state.app_panel == AppPanel::Fetching {
            CentralPanel::default().show(ctx, |ui| {
                self.views.fetch_overlay.ui(ui, &mut self.state);
            });
        }

        if self.state.app_panel == AppPanel::Game {
            CentralPanel::default().show(ctx, |ui| {
                let game_result = self.views.game.ui(ui, &mut self.state);
                if game_result == GameResult::Exit {
                    self.state.app_panel = AppPanel::Menu;
                }

                if game_result == GameResult::NextPhoto {
                    self.views.game.start_game();
                }
            });
        }
    }
}
