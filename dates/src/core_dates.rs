// -- /dates/core_dates.rs
use chrono::{Datelike, Timelike, Local, Days, NaiveDate};

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

pub fn easter_sunday(year: i32) -> NaiveDate {
    // Gaußsche Osterformel (modifiziert nach Lichtenberg für Rust)
    let a = year % 19;
    let b = year / 100;
    let c = year % 100;
    let d = b / 4;
    let e = b % 4;
    let f = (b + 8) / 25;
    let g = (b - f + 1) / 3;
    let h = (19 * a + b - d - g + 15) % 30;
    let i = c / 4;
    let k = c % 4;
    
    // Rust-Absicherung: +7 verhindert, dass der Term vor dem % 7 negativ wird
    let l = (32 + 2 * e + 2 * i - h - k + 7) % 7; 
    
    let m = (a + 11 * h + 22 * l) / 451;
    
    // Direktes Casting zu u32 für Chrono
    let month = ((h + l - 7 * m + 114) / 31) as u32;          
    let day   = (((h + l - 7 * m + 114) % 31) + 1) as u32;    
    
    NaiveDate::from_ymd_opt(year, month, day).unwrap()
}


/// Die 9 Bundesländer Österreichs für rein informative Landesfeiertage
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum State {
    Burgenland,
    Kaernten,
    Niederoesterreich,
    Oberoesterreich,
    Salzburg,
    Steiermark,
    Tirol,
    Vorarlberg,
    Wien,
}

// =========================================================================
// 1. EBENE: STRIKTE BUNDESWEITE FEIERTAGE
// =========================================================================

/// Prüft ausschließlich bundesweite gesetzliche Feiertage (arbeitsfrei).
pub fn is_federal_holiday_at(date: NaiveDate) -> bool {
    let m = date.month();
    let d = date.day();

    // Fixe bundesweite Feiertage
    let is_fixed = matches!(
        (m, d),
        (1, 1)    // Neujahr
        | (1, 6)   // Hl. Drei Könige
        | (5, 1)   // Staatsfeiertag
        | (8, 15)  // Mariä Himmelfahrt
        | (10, 26) // Nationalfeiertag
        | (11, 1)  // Allerheiligen
        | (12, 8)  // Mariä Empfängnis
        | (12, 25) // Weihnachten
        | (12, 26) // Stefanitag
    );
    if is_fixed { return true; }

    // Bewegliche bundesweite Feiertage
    let easter = easter_sunday(date.year());
    if date == easter.checked_add_days(Days::new(1)).unwrap() { return true; }  // Ostermontag
    if date == easter.checked_add_days(Days::new(39)).unwrap() { return true; } // Christi Himmelfahrt
    if date == easter.checked_add_days(Days::new(50)).unwrap() { return true; } // Pfingstmontag
    if date == easter.checked_add_days(Days::new(60)).unwrap() { return true; } // Fronleichnam

    false
}

// =========================================================================
// 2. EBENE: REGIONALE INFO-ABFRAGEN (HINWEISE)
// =========================================================================

/// Gibt den Namen eines regionalen Feiertags zurück, falls vorhanden.
/// Dient rein als Info/Hinweis für Abfragen, zerschießt keine Fristen-Logik.
pub fn get_regional_holiday_info_at(date: NaiveDate, state: State) -> Option<&'static str> {
    let m = date.month();
    let d = date.day();

    match state {
        State::Burgenland if (m, d) == (11, 11) => Some("Hl. Martin"),
        State::Kaernten if (m, d) == (3, 19) => Some("Hl. Josef"),
        State::Niederoesterreich if (m, d) == (11, 15) => Some("Hl. Leopold"),
        State::Oberoesterreich if (m, d) == (5, 4) => Some("Hl. Florian"),
        State::Salzburg if (m, d) == (9, 24) => Some("Hl. Rupert"),
        State::Steiermark if (m, d) == (3, 19) => Some("Hl. Josef"),
        State::Tirol if (m, d) == (3, 19) => Some("Hl. Josef"),
        State::Vorarlberg if (m, d) == (3, 19) => Some("Hl. Josef"),
        State::Wien if (m, d) == (11, 15) => Some("Hl. Leopold"),
        _ => None,
    }
}

// =========================================================================
// 3. EBENE: BÖRSE & BROKER (WIENER BÖRSE / TARGET2)
// =========================================================================

/// Prüft, ob die Wiener Börse oder das TARGET2-Clearing für AT geschlossen hat.
pub fn is_trading_holiday_at(date: NaiveDate) -> bool {
    let m = date.month();
    let d = date.day();

    // Fixe Schließtage der Wiener Börse
    let is_fixed_trading_close = matches!(
        (m, d),
        (1, 1)     // Neujahr
        | (5, 1)   // Staatsfeiertag
        | (10, 26) // Nationalfeiertag
        | (12, 24) // Heiliger Abend
        | (12, 25) // Weihnachten
        | (12, 26) // Stefanitag
        | (12, 31) // Silvester
    );
    if is_fixed_trading_close { return true; }

    // Bewegliche Schließtage über Ostern
    let easter = easter_sunday(date.year());
    let good_friday = easter.checked_sub_days(Days::new(2)).unwrap();   // Karfreitag
    let easter_monday = easter.checked_add_days(Days::new(1)).unwrap(); // Ostermontag

    if date == good_friday || date == easter_monday {
        return true;
    }

    false
}

/// Prüft, ob ein regulärer AT-Handelstag vorliegt (Mo-Fr und die Wiener Börse hat geöffnet).
pub fn is_trading_day_at(date: NaiveDate) -> bool {
    let wd = date.weekday().number_from_monday();
    if wd >= 6 { 
        return false; 
    }
    
    !is_trading_holiday_at(date)
}

/// Ermittelt den nächsten gültigen AT-Börsenhandelstag.
pub fn next_trading_day_at(date: NaiveDate) -> NaiveDate {
    let mut d = date;
    while !is_trading_day_at(d) {
        d = d.checked_add_days(Days::new(1)).unwrap();
    }
    d
}



// =========================================================================
// STANDARD-ARBEITSTAGE (BUSINESS DAYS)
// =========================================================================

/// Ein normaler Bankarbeitstag (Mo-Fr, kein bundesweiter Feiertag).
pub fn is_business_day_at(date: NaiveDate) -> bool {
    let wd = date.weekday().number_from_monday();
    if wd >= 6 { return false; } // Wochenende
    
    !is_federal_holiday_at(date)
}

/// Nächster regulärer Arbeitstag.
pub fn next_business_day_at(date: NaiveDate) -> NaiveDate {
    let mut d = date;
    while !is_business_day_at(d) {
        d = d.checked_add_days(Days::new(1)).unwrap();
    }
    d
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

    #[test]
    fn test_now_time_format() {
        let time = now_time();
        assert_eq!(time.len(), 8);        // HH:MM:SS
        assert!(time.contains(':'));
    }

    #[test]
    fn test_now_date_format() {
        let date = now_date();
        assert_eq!(date.len(), 10);       // DD.MM.YYYY
        assert!(date.contains('.'));
    }

    #[test]
    fn test_weekday_values() {
        let weekday = now_weekday();
        assert!(weekday == "Montag" || weekday == "Dienstag" || 
                weekday == "Mittwoch" || weekday == "Donnerstag" ||
                weekday == "Freitag" || weekday == "Samstag" || 
                weekday == "Sonntag");
    }

    // ----------------------------------------
    // 1) is_leap_year Tests
    // ----------------------------------------
    #[test]
    fn test_leap_year_2024() {
        assert!(is_leap_year(2024));   // 2024 div 4, nicht 100
    }

    #[test]
    fn test_non_leap_2025() {
        assert!(!is_leap_year(2025));
    }

    #[test]
    fn test_leap_2000() {
        assert!(is_leap_year(2000));   // div 400
    }

    #[test]
    fn test_non_leap_1900() {
        assert!(!is_leap_year(1900));  // div 100, nicht 400
    }


    // ----------------------------------------
    // 2) current_year_info Tests (mock chrono)
    // ----------------------------------------
    #[test]
    fn test_current_year_info() {
        let (doy, total_days, leap) = current_year_info();
        assert!(!leap);              // 2026 kein Schaltjahr
        assert!((1..=total_days).contains(&doy));
        assert_eq!(total_days, 365);
    }

    #[test]
    fn test_easter_sunday() {
        // Ostersonntag 2026 war am 5. April
        assert_eq!(easter_sunday(2026), NaiveDate::from_ymd_opt(2026, 4, 5).unwrap());
    }

    #[test]
    fn test_strict_separation() {
        // 1. Testfall: 8. Dezember (Mariä Empfängnis)
        let mariae_empfaengnis = NaiveDate::from_ymd_opt(2025, 12, 8).unwrap();
        assert!(is_federal_holiday_at(mariae_empfaengnis));   // Gesetzlicher Feiertag
        assert!(!is_trading_holiday_at(mariae_empfaengnis));  // Wiener Börse hat GEÖFFNET!

        // 2. Testfall: Karfreitag (Zwei Tage vor Ostersonntag 2025)
        let karfreitag_2025 = NaiveDate::from_ymd_opt(2025, 4, 18).unwrap();
        assert!(!is_federal_holiday_at(karfreitag_2025));    // Kein gesetzlicher Feiertag in AT
        assert!(is_trading_holiday_at(karfreitag_2025));     // Aber Börse/TARGET2 ist GESCHLOSSEN!

        // 3. Testfall: Regionaler Hinweischeck (Leopolditag in Wien)
        let leopolditag = NaiveDate::from_ymd_opt(2025, 11, 15).unwrap();
        assert!(!is_federal_holiday_at(leopolditag));         // Kein freier Tag bundesweit
        assert_eq!(get_regional_holiday_info_at(leopolditag, State::Wien), Some("Hl. Leopold")); // Korrekte Info
        assert_eq!(get_regional_holiday_info_at(leopolditag, State::Tirol), None);               // In Tirol irrelevant
    }    

    #[test]
fn test_trading_day_sequence() {
    // Gründonnerstag vor Ostern 2025
    let gruendonnerstag = NaiveDate::from_ymd_opt(2025, 4, 17).unwrap();
    
    // Gründonnerstag ist ein regulärer Handelstag
    assert!(is_trading_day_at(gruendonnerstag));
    
    // Der nächste reguläre Handelstag MUSS Dienstag nach Ostern sein,
    // weil Karfreitag, Samstag, Sonntag und Ostermontag die Börse zu ist!
    let expected_tuesday = NaiveDate::from_ymd_opt(2025, 4, 22).unwrap();
    assert_eq!(next_trading_day_at(gruendonnerstag + Days::new(1)), expected_tuesday);
}

}