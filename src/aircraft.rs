pub struct AircraftPhoto {
    url: String,
    source: String,
    photographer: String,
}

pub enum AircraftType {
    A320,
    A318,
}

pub struct Aircraft {
    photo: AircraftPhoto,
    ty: AircraftType,
}
