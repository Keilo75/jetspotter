use eframe::egui::{Align, ComboBox, Label, Layout, RichText, ScrollArea};
use strum::IntoEnumIterator;
use strum_macros::{Display, EnumIter};

use crate::jetspotter::{AircraftResult, AppState};

#[derive(PartialEq, Display, EnumIter, Clone)]
enum Sort {
    Name,
    #[strum(serialize = "Amount Games")]
    AmountGames,
    #[strum(serialize = "Win Rate")]
    WinRate,
}

#[derive(PartialEq, Display)]
enum SortDirection {
    Ascending,
    Descending,
}

pub struct StatisticsView {
    sort: Sort,
    sort_direction: SortDirection,
}

impl Default for StatisticsView {
    fn default() -> Self {
        Self {
            sort: Sort::Name,
            sort_direction: SortDirection::Ascending,
        }
    }
}

impl super::View for StatisticsView {
    fn ui(&mut self, ui: &mut eframe::egui::Ui, state: &mut AppState) {
        ui.horizontal(|ui| {
            ui.horizontal(|ui| {
                ui.label(RichText::new("Aircraft Statistics").heading().strong());

                ui.add(stats(
                    state.persistent.results.games_played,
                    state.persistent.results.games_won,
                ));
            });
            ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                ComboBox::from_label("")
                    .selected_text(self.sort_direction.to_string())
                    .show_ui(ui, |ui| {
                        ui.selectable_value(
                            &mut self.sort_direction,
                            SortDirection::Descending,
                            "Descending",
                        );
                        ui.selectable_value(
                            &mut self.sort_direction,
                            SortDirection::Ascending,
                            "Ascending",
                        );
                    });

                ComboBox::from_label("Sort by")
                    .selected_text(self.sort.to_string())
                    .show_ui(ui, |ui| {
                        for sort in Sort::iter() {
                            ui.selectable_value(&mut self.sort, sort.clone(), sort.to_string());
                        }
                    });
            });
        });

        let sorted_results = sort_results(
            &state.persistent.results.aircraft_results,
            &self.sort,
            &self.sort_direction,
        );

        ui.group(|ui| {
            ScrollArea::vertical().show(ui, |ui| {
                for (i, result) in sorted_results.enumerate() {
                    if i != 0 {
                        ui.separator();
                    }

                    ui.label(RichText::new(result.aircraft.to_string()).heading());
                    ui.add(stats(result.games_played, result.games_won));
                }
            });
        });
    }
}

fn sort_results<'a>(
    results: &'a Vec<Box<AircraftResult>>,
    sort: &Sort,
    sort_direction: &SortDirection,
) -> Box<dyn Iterator<Item = &'a Box<AircraftResult>> + 'a> {
    let sorted = match sort {
        Sort::Name => results,
        Sort::AmountGames => results,
        Sort::WinRate => results,
    }
    .iter();

    if sort_direction == &SortDirection::Descending {
        Box::new(sorted.rev())
    } else {
        Box::new(sorted)
    }
}

pub fn stats(games_played: i32, games_won: i32) -> Label {
    let win_rate = if games_played == 0 {
        0.0
    } else {
        let win_rate = games_won as f32 / games_played as f32;
        (win_rate * 100.0).round()
    };

    Label::new(format!(
        "Games played: {} | Games won: {} | Win rate: {}%",
        games_played, games_won, win_rate
    ))
}
