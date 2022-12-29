use scraper::{ElementRef, Selector};
use serde::{Deserialize, Serialize};
use strum_macros::{Display, EnumIter};

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq, Eq, Hash, EnumIter, Display)]
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

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct AircraftPhoto {
    pub id: String,
    pub url: String,
    pub photographer: String,
    pub kind: AircraftKind,
    pub full_kind: String,
    pub registration: String,
    pub airline: String,
}

impl AircraftPhoto {
    pub fn from_photo_div(div: ElementRef) -> AircraftPhoto {
        let id = div.value().attr("data-photo").unwrap().to_owned();

        let url = parse_url(&div);

        let info_selector = Selector::parse(".mobile-only .result__infoListText").unwrap();
        let mut info_items = div.select(&info_selector);

        let photographer = parse_info(info_items.next());
        let full_kind = parse_info(info_items.next());
        let kind = parse_kind(&full_kind);
        let registration = parse_info(info_items.next());
        let registration = parse_registration(registration);
        let airline = parse_info(info_items.next());

        let photo = AircraftPhoto {
            id,
            url,
            photographer,
            kind,
            full_kind,
            airline,
            registration,
        };

        photo
    }
}

fn parse_info(element: Option<ElementRef>) -> String {
    element
        .unwrap()
        .text()
        .collect::<String>()
        .trim()
        .to_owned()
}

fn parse_url(div: &ElementRef) -> String {
    let small_photo_selector = Selector::parse("img.result__photo").unwrap();
    let small_photo_url = div
        .select(&small_photo_selector)
        .next()
        .unwrap()
        .value()
        .attr("src")
        .unwrap();

    let url = small_photo_url
        .replace("//", "http://")
        .replace("/400/", "/full/");
    url
}

fn parse_registration(registration: String) -> String {
    registration.split(" ").next().unwrap().to_owned()
}

fn parse_kind(full_kind: &String) -> AircraftKind {
    if full_kind.starts_with("Airbus A300") {
        return AircraftKind::A300;
    }
    if full_kind.starts_with("Airbus A310") {
        return AircraftKind::A310;
    }
    if full_kind.starts_with("Airbus A318") {
        return AircraftKind::A318;
    }
    if full_kind.starts_with("Airbus A319") {
        return AircraftKind::A319;
    }
    if full_kind.starts_with("Airbus A320") {
        return AircraftKind::A320;
    }
    if full_kind.starts_with("Airbus A321") {
        return AircraftKind::A321;
    }
    if full_kind.starts_with("Airbus A330") {
        return AircraftKind::A330;
    }
    if full_kind.starts_with("Airbus A340") {
        return AircraftKind::A340;
    }
    if full_kind.starts_with("Airbus A350") {
        return AircraftKind::A350;
    }
    if full_kind.starts_with("Airbus A380") {
        return AircraftKind::A380;
    }

    if full_kind.starts_with("Boeing 707") {
        return AircraftKind::B707;
    }
    if full_kind.starts_with("Boeing 717") {
        return AircraftKind::B717;
    }
    if full_kind.starts_with("Boeing 727") {
        return AircraftKind::B727;
    }
    if full_kind.starts_with("Boeing 737") {
        return AircraftKind::B737;
    }
    if full_kind.starts_with("Boeing 747") {
        return AircraftKind::B747;
    }
    if full_kind.starts_with("Boeing 757") {
        return AircraftKind::B757;
    }
    if full_kind.starts_with("Boeing 767") {
        return AircraftKind::B767;
    }
    if full_kind.starts_with("Boeing 777") {
        return AircraftKind::B777;
    }
    if full_kind.starts_with("Boeing 787") {
        return AircraftKind::B787;
    }

    if full_kind.starts_with("Bombardier CSeries") {
        return AircraftKind::A220;
    }

    panic!("Could not match {}", full_kind)
}
