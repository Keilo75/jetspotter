use std::thread;
use std::time::{Duration, Instant};

use poll_promise::Sender;
use scraper::{Html, Selector};

use self::aircraft_photo::AircraftPhoto;
pub mod aircraft_photo;

const WAIT_DURATION: Duration = Duration::from_millis(5_000);

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

fn get_url(page: i32) -> String {
    let url = format!("https://www.jetphotos.com/showphotos.php?aircraft%5B0%5D=Airbus+A300%3B&aircraft%5B1%5D=Airbus+A310%3B&aircraft%5B2%5D=Airbus+A318%3B&aircraft%5B3%5D=Airbus+A319%3B&aircraft%5B4%5D=Airbus+A320%3B&aircraft%5B5%5D=Airbus+A321%3B&aircraft%5B6%5D=Airbus+A330%3B&aircraft%5B7%5D=Airbus+A340%3B&aircraft%5B8%5D=Airbus+A350%3B&aircraft%5B9%5D=Airbus+A380%3B&aircraft%5B10%5D=Boeing+707%3B&aircraft%5B11%5D=Boeing+717%3B&aircraft%5B12%5D=Boeing+727%3B&aircraft%5B13%5D=Boeing+737%3B&aircraft%5B14%5D=Boeing+747%3B&aircraft%5B15%5D=Boeing+757%3B&aircraft%5B16%5D=Boeing+767%3B&aircraft%5B17%5D=Boeing+777%3B&aircraft%5B18%5D=Boeing+787%3B&aircraft%5B19%5D=Bombardier+CSeries+CS100%3B&aircraft%5B20%5D=Bombardier+CSeries+CS300%3B&fromDate=&genre=all&search-type=AdvancedMulti&sort-order=0&toDate=&page={}", page);
    url
}

fn parse_response(response: String, aircraft_left: usize) -> Vec<AircraftPhoto> {
    let mut vec = Vec::new();

    let document = Html::parse_document(response.as_str());
    let photo_selector = Selector::parse("div[data-photo]").unwrap();

    let photo_divs = document.select(&photo_selector).take(aircraft_left);
    for photo_div in photo_divs {
        vec.push(AircraftPhoto::from_photo_div(photo_div));
    }

    vec
}
