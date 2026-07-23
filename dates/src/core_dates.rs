// src/dates/core_dates.rs
use chrono::{Datelike, Timelike, Local};

pub fn now_time() -> String {
    let now = Local::now();
    format!("{:02}:{:02}:{:02}", now.hour(), now.minute(), now.second())
}

pub fn now_date() -> String {
    let now = Local::now().date_naive();
    format!("{:02}.{:02}.{}", now.day(), now.month(), now.year())
}

pub fn now() -> String {
    format!("{} {}", now_date(), now_time())
}

pub fn now_weekday() -> &'static str {
    let now = Local::now();
    match now.weekday().num_days_from_monday() {
        0 => "Montag", 1 => "Dienstag", 2 => "Mittwoch",
        3 => "Donnerstag", 4 => "Freitag", 5 => "Samstag",
        6 => "Sonntag", _ => "Fehler",
    }
}

pub fn is_leap_year(year: i32) -> bool {
    year % 4 == 0 && (year % 100 != 0 || year % 400 == 0)
}

pub fn current_year_info() -> (u32, u32, bool) {
    let today = Local::now().date_naive();
    let year = today.year();
    let doy = today.ordinal();
    let total_days = if is_leap_year(year) { 366 } else { 365 };
    let leap = is_leap_year(year);
    println!("\nHeute: {}./{} Tag (Schaltjahr: {})\n", doy, total_days, if leap { "ja" } else { "nein" });
    (doy, total_days, leap)
}

// ------------------------------------------------------------
// Unit Tests
// ------------------------------------------------------------
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_leap_year() {
        // Standard-Schaltjahre (durch 4 teilbar)
        assert!(is_leap_year(2024));
        assert!(is_leap_year(2028));

        // Keine Schaltjahre (nicht durch 4 teilbar)
        assert!(!is_leap_year(2023));
        assert!(!is_leap_year(2025));

        // Jahrhundert-Regel: Durch 100 teilbar -> KEIN Schaltjahr
        assert!(!is_leap_year(1900));
        assert!(!is_leap_year(2100));

        // Ausnahme zur Jahrhundert-Regel: Durch 400 teilbar -> DOCH ein Schaltjahr
        assert!(is_leap_year(2000));
        assert!(is_leap_year(2400));
    }

    #[test]
    fn test_now_formats() {
        let time_str = now_time();
        let date_str = now_date();
        let now_str = now();

        // Prüfe, ob die Längen der generierten Strings exakt stimmen
        // HH:MM:SS -> 8 Zeichen
        assert_eq!(time_str.len(), 8);
        // DD.MM.YYYY -> 10 Zeichen
        assert_eq!(date_str.len(), 10);
        // DD.MM.YYYY HH:MM:SS -> 19 Zeichen
        assert_eq!(now_str.len(), 19);

        // Prüfe auf korrekte Trennzeichen
        assert_eq!(time_str.chars().nth(2), Some(':'));
        assert_eq!(date_str.chars().nth(2), Some('.'));
        assert!(now_str.contains(' ')); // Leerzeichen zwischen Datum und Zeit
    }

    #[test]
    fn test_now_weekday_valid() {
        let weekday = now_weekday();
        
        // Der Wochentag darf niemals ein "Fehler"-String sein
        assert_ne!(weekday, "Fehler");
        
        // Prüfe, ob das Ergebnis einem der gültigen deutschen Wochentage entspricht
        let valid_days = ["Montag", "Dienstag", "Mittwoch", "Donnerstag", "Freitag", "Samstag", "Sonntag"];
        assert!(valid_days.contains(&weekday));
    }
}