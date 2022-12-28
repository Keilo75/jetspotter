use core::time;
use std::thread;

use poll_promise::Sender;

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
    page: i32,
) {
    let aircraft_left = total - current;
    dbg!(page, aircraft_left);

    thread::spawn(|| {
        thread::sleep(time::Duration::from_secs_f32(0.1));

        let mut vec = Vec::new();

        for _ in 0..2 {
            vec.push(AircraftPhoto {
                src: "()".to_owned(),
                photographer: "()".to_owned(),
                kind: "()".to_owned(),
                reg: "()".to_owned(),
            });
        }

        sender.send(Ok(vec));
    });
}
