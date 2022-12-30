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

pub struct StatisticsPanel {
    sort: Sort,
    sort_direction: SortDirection,
}

impl Default for StatisticsPanel {
    fn default() -> Self {
        Self {
            sort: Sort::Name,
            sort_direction: SortDirection::Ascending,
        }
    }
}

impl super::View for StatisticsPanel {
    fn ui(&mut self, ui: &mut eframe::egui::Ui, state: &mut AppState) {
        ui.horizontal(|ui| {
            ui.horizontal(|ui| {
                ui.label(RichText::new("Statistics").heading().strong());

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
            &mut state.persistent.results.aircraft_results,
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

                    ui.horizontal(|ui| {
                        ui.add(stats(result.games_played, result.games_won));

                        if !result.misses.is_empty() {
                            ui.with_layout(Layout::right_to_left(Align::TOP), |ui| {
                                let mut misses = result
                                    .misses
                                    .iter()
                                    .map(|(id, num)| (id.clone(), num.clone()))
                                    .collect::<Vec<(String, i32)>>();

                                misses.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
                                let joined_misses = misses
                                    .iter()
                                    .take(3)
                                    .map(|(id, num)| {
                                        let percentage = calculate_win_rate(
                                            result.games_played - result.games_won,
                                            *num,
                                        );
                                        format!("{} ({}%)", id, percentage)
                                    })
                                    .collect::<Vec<String>>()
                                    .join(", ");

                                ui.label(format!("Common Misses: {}", joined_misses));
                            });
                        }
                    });
                }
            });
        });
    }
}

fn sort_results<'a>(
    results: &'a mut Vec<Box<AircraftResult>>,
    sort: &Sort,
    sort_direction: &SortDirection,
) -> Box<dyn Iterator<Item = &'a Box<AircraftResult>> + 'a> {
    let sorted = match sort {
        Sort::Name => {
            results.sort_by_key(|k| k.aircraft.to_string());
            results
        }
        Sort::AmountGames => {
            results.sort_by_key(|k| k.games_played);
            results
        }
        Sort::WinRate => {
            results.sort_by_key(|k| calculate_win_rate(k.games_played, k.games_won));
            results
        }
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
        0
    } else {
        calculate_win_rate(games_played, games_won)
    };

    Label::new(format!(
        "Games played: {} | Games won: {} | Win rate: {}%",
        games_played, games_won, win_rate
    ))
}

pub fn calculate_win_rate(games_played: i32, games_won: i32) -> i32 {
    let win_rate = games_won as f32 / games_played as f32;
    (win_rate * 100.0).round() as i32
}
