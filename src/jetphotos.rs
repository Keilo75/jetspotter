use std::thread;
use std::time::{Duration, Instant};

use poll_promise::Sender;

const WAIT_DURATION: Duration = Duration::from_millis(5_000);

#[derive(Clone)]
#[allow(dead_code)]
pub struct AircraftPhoto {
    src: String,
    photographer: String,
    kind: String,
    reg: String,
}

pub fn fetch_photos(
    sender: Sender<ehttp::Result<Vec<AircraftPhoto>>>,
    current: i32,
    total: i32,
    _page: i32,
) {
    thread::spawn(move || {
        let start = Instant::now();

        let _aircraft_left = total - current;
        let mut vec = Vec::new();

        for _ in 0..2 {
            vec.push(AircraftPhoto {
                src: "()".to_owned(),
                photographer: "()".to_owned(),
                kind: "()".to_owned(),
                reg: "()".to_owned(),
            });
        }

        let elapsed = start.elapsed();
        if elapsed < WAIT_DURATION {
            let time_to_wait = WAIT_DURATION - elapsed;

            thread::sleep(time_to_wait);
        }

        sender.send(Ok(vec));
    });
}
