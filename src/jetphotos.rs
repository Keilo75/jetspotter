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

pub fn fetch_photos(sender: Sender<ehttp::Result<AircraftPhoto>>) {
    thread::spawn(|| {
        thread::sleep(time::Duration::from_secs_f32(0.1));

        sender.send(Ok(AircraftPhoto {
            src: "()".to_owned(),
            photographer: "()".to_owned(),
            kind: "()".to_owned(),
            reg: "()".to_owned(),
        }));
    });
}
