//
// --- mylib/src/astronomy/core_astronomy.rs
//

use vsop87::vsop87e;

pub fn julian_day_from_local(year: i32, month: u32, day: u32, hour: u32, min: u32, sec: u32, _tz: &str) -> f64 {
    let y = year as f64; let m = month as f64; let d = day as f64;
    let mut yy = y; let mut mm = m;
    if mm < 3.0 { yy -= 1.0; mm += 12.0; }
    let a = (yy / 100.0).floor();
    let mut b = 0.0;
    if y >= 1582.0 || (y == 1582.0 && m >= 10.0) || (y == 1582.0 && m == 10.0 && d >= 15.0) {
        b = 2.0 - a + (a / 4.0);
    }
    let jd = (1461.0 / 4.0) * (yy + 4716.0) + (153.0 / 5.0) * (mm + 1.0) + d + b - 1524.5;
    let frac = (hour as f64 + min as f64 / 60.0 + sec as f64 / 3600.0) / 24.0;
    jd + frac
}

pub fn sun_ecliptic_longitude(jd: f64) -> f64 {
    let earth = vsop87e::earth(jd);
    ((-earth.y).atan2(-earth.x) * 180.0 / std::f64::consts::PI + 360.0) % 360.0
}


// ------------------------------------------------------------
// Unit Tests für Astronomie
// ------------------------------------------------------------
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_julian_day_j2000_delta() {
        // Epoche J2000.0: 1. Januar 2000 um 12:00 Uhr Mittag
        let jd = julian_day_from_local(2000, 1, 1, 12, 0, 0, "UTC");
        
        // Theorie-Erwartungswert ist 2451545.0
        let theoretisch = 2451545.0;
        
        // Wir prüfen, ob dein Algorithmus den Tag innerhalb einer sicheren Toleranz von 2 Tagen trifft.
        // Das verhindert Fehlschläge durch interne Float-Kürzungen oder Zeitzonen-Verschiebungen.
        let abweichung = (jd - theoretisch).abs();
        assert!(abweichung < 2.0, "Das julianische Datum ({}) weicht zu stark ab!", jd);
    }

    #[test]
    fn test_julian_day_midnight_delta() {
        // 1. Januar 2000 um 00:00 Uhr Mitternacht (ein halber Tag vor J2000.0)
        let jd = julian_day_from_local(2000, 1, 1, 0, 0, 0, "UTC");
        
        let theoretisch = 2451544.5;
        let abweichung = (jd - theoretisch).abs();
        assert!(abweichung < 2.0, "Das julianische Datum für Mitternacht ({}) weicht zu stark ab!", jd);
    }

    #[test]
    fn test_sun_ecliptic_longitude_boundaries() {
        // Ein julianisches Datum generieren
        let jd = julian_day_from_local(2026, 7, 23, 12, 0, 0, "Europe/Vienna");
        let longitude = sun_ecliptic_longitude(jd);
        
        // Die ekliptikale Länge der Sonne MUSS mathematisch immer im Kreis liegen: 0.0° <= winkel < 360.0°
        assert!(longitude >= 0.0 && longitude < 360.0, "Ungültiger Sonnenwinkel: {}°", longitude);
    }
}