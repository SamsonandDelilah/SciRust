//
//   --- Library\src\prefix.rs als Library siehe lib.rs ---
//


// 1. Enum der Prefix-Typen
#[derive(Debug, PartialEq, Copy, Clone)]
pub enum PrefixType {
    Yocto,
    Zepto,
    Atto,
    Femto,
    Pico,
    Nano,
    Micro,
    Milli,
    Centi,
    Deci,
    Deca,
    Hecto,
    Kilo,
    Mega,
    Giga,
    Tera,
    Peta,
    Exa,
    Zetta,
    Yotta,
}

// 2. Struct für SI-Präfix mit zugehörigen Feldern
#[derive(Debug, Clone)]
pub struct SIPrefix {
    pub kind: PrefixType,
    pub name: &'static str,
    pub symbol: &'static str,
    pub multiplier: f64,
}

impl SIPrefix {
    // 3. Erstelle neue Instanzen (Konstanten)
    const YOCTO: SIPrefix = SIPrefix { kind: PrefixType::Yocto, name: "yocto", symbol: "y", multiplier: 1e-24 };
    const ZEPTO: SIPrefix = SIPrefix { kind: PrefixType::Zepto, name: "zepto", symbol: "z", multiplier: 1e-21 };
    const ATTO: SIPrefix = SIPrefix { kind: PrefixType::Atto, name: "atto", symbol: "a", multiplier: 1e-18 };
    const FEMTO: SIPrefix = SIPrefix { kind: PrefixType::Femto, name: "femto", symbol: "f", multiplier: 1e-15 };
    const PICO: SIPrefix = SIPrefix { kind: PrefixType::Pico, name: "pico", symbol: "p", multiplier: 1e-12 };
    const NANO: SIPrefix = SIPrefix { kind: PrefixType::Nano, name: "nano", symbol: "n", multiplier: 1e-9 };
    const MICRO: SIPrefix = SIPrefix { kind: PrefixType::Micro, name: "micro", symbol: "μ", multiplier: 1e-6 };
    const MILLI: SIPrefix = SIPrefix { kind: PrefixType::Milli, name: "milli", symbol: "m", multiplier: 1e-3 };
    const CENTI: SIPrefix = SIPrefix { kind: PrefixType::Centi, name: "centi", symbol: "c", multiplier: 1e-2 };
    const DECI: SIPrefix = SIPrefix { kind: PrefixType::Deci, name: "deci", symbol: "d", multiplier: 1e-1 };
    const DECA: SIPrefix = SIPrefix { kind: PrefixType::Deca, name: "deca", symbol: "da", multiplier: 1e1 };
    const HECTO: SIPrefix = SIPrefix { kind: PrefixType::Hecto, name: "hecto", symbol: "h", multiplier: 1e2 };
    const KILO: SIPrefix = SIPrefix { kind: PrefixType::Kilo, name: "kilo", symbol: "k", multiplier: 1e3 };
    const MEGA: SIPrefix = SIPrefix { kind: PrefixType::Mega, name: "mega", symbol: "M", multiplier: 1e6 };
    const GIGA: SIPrefix = SIPrefix { kind: PrefixType::Giga, name: "giga", symbol: "G", multiplier: 1e9 };
    const TERA: SIPrefix = SIPrefix { kind: PrefixType::Tera, name: "tera", symbol: "T", multiplier: 1e12 };
    const PETA: SIPrefix = SIPrefix { kind: PrefixType::Peta, name: "peta", symbol: "P", multiplier: 1e15 };
    const EXA: SIPrefix = SIPrefix { kind: PrefixType::Exa, name: "exa", symbol: "E", multiplier: 1e18 };
    const ZETTA: SIPrefix = SIPrefix { kind: PrefixType::Zetta, name: "zetta", symbol: "Z", multiplier: 1e21 };
    const YOTTA: SIPrefix = SIPrefix { kind: PrefixType::Yotta, name: "yotta", symbol: "Y", multiplier: 1e24 };
    
    // 4. Gib alle SIPrefixe als Array zurück
    pub fn all() -> &'static [SIPrefix] {
        use PrefixType::*;
        &[
            SIPrefix::YOCTO,
            SIPrefix::ZEPTO,
            SIPrefix::ATTO,
            SIPrefix::FEMTO,
            SIPrefix::PICO,
            SIPrefix::NANO,
            SIPrefix::MICRO,
            SIPrefix::MILLI,
            SIPrefix::CENTI,
            SIPrefix::DECI,
            SIPrefix::DECA,
            SIPrefix::HECTO,
            SIPrefix::KILO,
            SIPrefix::MEGA,
            SIPrefix::GIGA,
            SIPrefix::TERA,
            SIPrefix::PETA,
            SIPrefix::EXA,
            SIPrefix::ZETTA,
            SIPrefix::YOTTA,
        ]
    }

    pub fn kind(&self) -> PrefixType {
        self.kind
    }
    
    pub fn name(&self) -> &str {
        self.name
    }
    
    pub fn symbol(&self) -> &str {
        self.symbol
    }
    
    pub fn multiplier(&self) -> f64 {
        self.multiplier
    }
    
    // 5. Suche nach einem SIPrefix anhand seines Namens (kleine Buchstaben)
    pub fn prefix_find_by_name(name: &str) -> Option<&'static SIPrefix> {
        SIPrefix::all().iter().find(|p| p.name == name)
    }
    
    // 6. Ausgabe als String
    pub fn prefix_to_string(&self) -> String {
        format!("{} ({}) = {}", self.name, self.symbol, self.multiplier)
    }

    pub fn get_prefix_type(&self) -> PrefixType {
        self.kind
    }

    pub fn get_prefix_name(&self) -> &str {
        self.name
    }

}


