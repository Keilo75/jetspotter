use scraper::{ElementRef, Html, Selector};
use serde::{Deserialize, Serialize};
use std::thread;
use std::time::{Duration, Instant};

use poll_promise::Sender;

const WAIT_DURATION: Duration = Duration::from_millis(5_000);

#[derive(Clone, Serialize, Deserialize)]
pub enum AircraftKind {
    A220,
    A300,
    A310,
    A318,
    A319,
    A320,
    A321,
    A330,
    A340,
    A350,
    A380,
    B707,
    B717,
    B727,
    B737,
    B747,
    B757,
    B767,
    B777,
    B787,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct AircraftPhoto {
    id: String,
    url: String,
    photographer: String,
    kind: AircraftKind,
    full_kind: String,
    registration: String,
    airline: String,
}

pub fn fetch_photos(sender: Sender<Vec<AircraftPhoto>>, current: usize, total: i32, page: i32) {
    thread::spawn(move || {
        let start = Instant::now();
        let aircraft_left = total as usize - current;

        let url = get_url(page);
        let response = reqwest::blocking::get(url).unwrap().text().unwrap();
        let vec = parse_response(response, aircraft_left);

        let elapsed = start.elapsed();
        if elapsed < WAIT_DURATION {
            let time_to_wait = WAIT_DURATION - elapsed;
            thread::sleep(time_to_wait);
        }

        sender.send(vec);
    });
}

pub fn get_url(page: i32) -> String {
    let url = format!("https://www.jetphotos.com/showphotos.php?aircraft%5B0%5D=Airbus+A300%3B&aircraft%5B1%5D=Airbus+A310%3B&aircraft%5B2%5D=Airbus+A318%3B&aircraft%5B3%5D=Airbus+A319%3B&aircraft%5B4%5D=Airbus+A320%3B&aircraft%5B5%5D=Airbus+A321%3B&aircraft%5B6%5D=Airbus+A330%3B&aircraft%5B7%5D=Airbus+A340%3B&aircraft%5B8%5D=Airbus+A350%3B&aircraft%5B9%5D=Airbus+A380%3B&aircraft%5B10%5D=Boeing+707%3B&aircraft%5B11%5D=Boeing+717%3B&aircraft%5B12%5D=Boeing+727%3B&aircraft%5B13%5D=Boeing+737%3B&aircraft%5B14%5D=Boeing+747%3B&aircraft%5B15%5D=Boeing+757%3B&aircraft%5B16%5D=Boeing+767%3B&aircraft%5B17%5D=Boeing+777%3B&aircraft%5B18%5D=Boeing+787%3B&aircraft%5B19%5D=Bombardier+CSeries+CS100%3B&aircraft%5B20%5D=Bombardier+CSeries+CS300%3B&fromDate=&genre=all&search-type=AdvancedMulti&sort-order=0&toDate=&page={}", page);
    url
}

pub fn parse_response(response: String, aircraft_left: usize) -> Vec<AircraftPhoto> {
    let mut vec = Vec::new();

    let document = Html::parse_document(response.as_str());
    let photo_selector = Selector::parse("div[data-photo]").unwrap();

    let photo_divs = document.select(&photo_selector).take(aircraft_left);
    for photo_div in photo_divs {
        vec.push(parse_photo_div(photo_div));
    }

    vec
}

pub fn parse_photo_div(div: ElementRef) -> AircraftPhoto {
    let id = div.value().attr("data-photo").unwrap().to_owned();

    let small_photo_selector = Selector::parse("img.result__photo").unwrap();
    let small_photo_url = div
        .select(&small_photo_selector)
        .next()
        .unwrap()
        .value()
        .attr("src")
        .unwrap();

    let url = small_photo_url.replace("//", "").replace("/400/", "/full/");

    AircraftPhoto {
        id,
        url,
        photographer: "".to_string(),
        kind: AircraftKind::A220,
        full_kind: "".to_string(),
        airline: "".to_string(),
        registration: "".to_string(),
    }
}
