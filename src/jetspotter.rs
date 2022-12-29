use std::collections::HashMap;
use strum::IntoEnumIterator;

use poll_promise::Promise;
use serde::{Deserialize, Serialize};

use crate::jetphotos::{AircraftKind, AircraftPhoto};
use crate::views::Views;

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
        let mut aircraft_results = HashMap::new();
        for aircraft_kind in AircraftKind::iter() {
            let aircraft_result = AircraftResult {
                games_played: 0,
                games_won: 0,
                misses: HashMap::new(),
            };

            aircraft_results.insert(aircraft_kind.to_string(), aircraft_result);
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
    pub aircraft_results: HashMap<String, AircraftResult>,
}

#[derive(Serialize, Deserialize)]
pub struct AircraftResult {
    pub games_played: i32,
    pub games_won: i32,
    pub misses: HashMap<String, i32>,
}

#[derive(PartialEq, Debug)]
pub enum AppState {
    Menu,
    Fetching,
}

pub struct Jetspotter {
    pub persistent: PersistentData,
    pub state: AppState,
    pub promise: Option<Promise<Vec<AircraftPhoto>>>,
    pub page: i32,
    pub views: Views,
}
impl Jetspotter {
    pub fn new() -> Self {
        let persistent = PersistentData::load();

        Jetspotter {
            persistent,
            state: AppState::Menu,
            promise: None,
            page: 1,
            views: Default::default(),
        }
    }
}
