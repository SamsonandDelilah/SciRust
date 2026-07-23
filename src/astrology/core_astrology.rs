//
// --- mylib/src/astrology/score_astrology.rs
//


use chrono::{Datelike, Local};

pub fn sun_sign(month: u32, day: u32) -> &'static str {
    match (month, day) {
        (1, 20..=31) | (2, 1..=18) => "Wassermann", (2, 19..=29) | (3, 1..=20) => "Fische", 
        (3, 21..=31) | (4, 1..=19) => "Widder", (4, 20..=30) | (5, 1..=20) => "Stier",
        (5, 21..=31) | (6, 1..=20) => "Zwillinge", (6, 21..=30) | (7, 1..=22) => "Krebs",
        (7, 23..=31) | (8, 1..=22) => "Löwe", (8, 23..=31) | (9, 1..=22) => "Jungfrau",
        (9, 23..=30) | (10, 1..=22) => "Waage", (10, 23..=31) | (11, 1..=21) => "Skorpion",
        (11, 22..=30) | (12, 1..=21) => "Schütze", (12, 22..=31) | (1, 1..=19) => "Steinbock",
        _ => "Ungültig"
    }
}

pub fn sun_sign_today() -> &'static str {
    let today = Local::now().date_naive();
    sun_sign(today.month(), today.day())
}

pub fn sun_sign_vsop87(year: i32, month: u32, day: u32, hour: u32, min: u32, sec: u32, timezone: &str) -> &'static str {
    // Greift pfadübergreifend direkt auf das neue, eigenständige Astronomie-Ordner-Modul zu
    let jd = crate::astronomy::core_astronomy::julian_day_from_local(year, month, day, hour, min, sec, timezone);
    let sun_lon = crate::astronomy::core_astronomy::sun_ecliptic_longitude(jd);
    
    let sign_idx = (sun_lon / 30.0) as usize % 12;
    ["Widder","Stier","Zwillinge","Krebs","Löwe","Jungfrau",
     "Waage","Skorpion","Schütze","Steinbock","Wassermann","Fische"][sign_idx]
}
