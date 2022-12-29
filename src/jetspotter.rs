use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::{
    jetphotos::aircraft_photo::{AircraftKind, AircraftPhoto},
    views::Views,
};
use strum::IntoEnumIterator;

#[derive(Serialize, Deserialize)]
pub struct PersistentData {
    pub dark_mode: bool,
    pub fetch_amount: i32,
    pub aircraft: Vec<AircraftPhoto>,
    pub results: Results,
}

impl PersistentData {
    fn load() -> Self {
        let config: Self = confy::load("jetspotter", None).unwrap_or_default();
        config
    }

    pub fn save(&mut self) {
        confy::store("jetspotter", None, self).unwrap();
    }
}

impl Default for PersistentData {
    fn default() -> Self {
        let mut aircraft_results = Vec::new();
        for aircraft_kind in AircraftKind::iter() {
            let aircraft_result = AircraftResult {
                aircraft: aircraft_kind,
                games_played: 0,
                games_won: 0,
                misses: HashMap::new(),
            };

            aircraft_results.push(Box::new(aircraft_result));
        }

        let results = Results {
            games_played: 0,
            games_won: 0,
            aircraft_results,
        };

        Self {
            dark_mode: true,
            aircraft: Vec::new(),
            fetch_amount: 100,
            results,
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct Results {
    pub games_played: i32,
    pub games_won: i32,
    pub aircraft_results: Vec<Box<AircraftResult>>,
}

#[derive(Serialize, Deserialize)]
pub struct AircraftResult {
    pub aircraft: AircraftKind,
    pub games_played: i32,
    pub games_won: i32,
    pub misses: HashMap<String, i32>,
}

#[derive(PartialEq, Debug)]
pub enum AppPanel {
    Menu,
    Fetching,
}

pub struct AppState {
    pub app_panel: AppPanel,
    pub persistent: PersistentData,
}

impl Default for AppState {
    fn default() -> Self {
        AppState {
            app_panel: AppPanel::Menu,
            persistent: PersistentData::load(),
        }
    }
}

pub struct Jetspotter {
    pub state: AppState,
    pub views: Views,
}

impl Jetspotter {
    pub fn new() -> Self {
        Jetspotter {
            state: Default::default(),
            views: Default::default(),
        }
    }
}
