// ---core_physics.rs
//pub mod core_physics;

pub mod gas_equations;

use gas_equations::{R_IDEAL, GasModel};
use roots::{find_root, RootsResult};
use num_traits::{Float, FromPrimitive};

/// Fiktive/Reale Stoffdatenbank für stoffspezifische Berechnungen (SSOT)
pub struct SubstanceData {
    pub name: &'static str,
    pub tc: f64,    // Kritische Temperatur (K)
    pub pc: f64,    // Kritischer Druck (Pa)
    pub omega: f64, // Azentrischer Faktor
    pub vdw_a: f64, // vdW Parameter a
    pub vdw_b: f64, // vdW Parameter b
}

/// Statischer Zugriff auf fiktive bzw. reale Tabellendaten
pub const CO2_DATA: SubstanceData = SubstanceData {
    name: "Carbon Dioxide (CO2)",
    tc: 304.2,
    pc: 7.38e6,
    omega: 0.225,
    vdw_a: 0.364,
    vdw_b: 4.27e-5,
};

/// Berechnet das molare Volumen v (m³/mol) über Ihre roots-Engine (Zweistufen-Rakete).
/// Löst die thermodynamische Gleichung: f(v) = p_target - p_model(T, v) = 0
pub fn calculate_molar_volume<T, M>(
    model: &M,
    temperature: T,
    target_pressure: T,
    v_start: T,
    v_end: T,
) -> Result<T, &'static str>
where
    T: Float + FromPrimitive,
    M: GasModel<T>,
{
    // Die Zielfunktion für den Nullstellen-Löser definieren
    let closure_integrand = |v: T| {
        target_pressure - model.calculate_pressure(temperature, v)
    };

    // KORREKTUR: Kein .unwrap() verwenden! Wir reichen das Result über match sauber weiter.
    match find_root(closure_integrand, v_start, v_end) {
        RootsResult::Ok { value, .. } => Ok(value),
        RootsResult::Err { reason } => Err(reason),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use gas_equations::{IdealGas, VanDerWaals, PengRobinson};

    #[test]
    fn test_gas_equations_pressure() {
        let t = 300.0;
        let v = 0.025;

        let ig = IdealGas::default();
        let p_ig = ig.calculate_pressure(t, v);
        // Ideales Gas Kontrollwert: (8.31446 * 300) / 0.025 = 99773.55 Pa
        assert!((p_ig - 99773.55).abs() < 1e-2);

        let vdw = VanDerWaals::new(CO2_DATA.vdw_a, CO2_DATA.vdw_b);
        let p_vdw = vdw.calculate_pressure(t, v);
        assert!(p_vdw.is_finite());
    }

    #[test]
    fn test_volume_solving_with_roots() {
        let t = 300.0;
        let target_p = 4.0e6; // 4 MPa Zieldruck
        
        let pr = PengRobinson::new(CO2_DATA.tc, CO2_DATA.pc, CO2_DATA.omega);

        // Grenzen für das molare Volumen festlegen [b, ideales_volumen]
        let v_min = pr.b + 1e-6;
        let v_max = (R_IDEAL * t) / target_p;

        // Volumen über das roots-Crate berechnen lassen
        let solved_v = calculate_molar_volume(&pr, t, target_p, v_min, v_max).unwrap();
        
        // Den berechneten Wert rückwärts prüfen: p(v) muss wieder target_p ergeben!
        let check_p = pr.calculate_pressure(t, solved_v);
        assert!((check_p - target_p).abs() <= 1e-2);
    }

        #[test]
    fn test_isotherm_compression_consistency() {
        // Physikalisch-Thermodynamischer Konsistenzcheck: 
        // Bei isothermer Kompression (Temperatur konstant, Volumen sinkt) MUSS der Druck steigen.
        let t = 350.0; // 350 K
        let v_large = 0.030; // 30 Liter/mol
        let v_small = 0.015; // 15 Liter/mol (Kompression auf die Hälfte)

        let pr = PengRobinson::new(CO2_DATA.tc, CO2_DATA.pc, CO2_DATA.omega);

        let p_at_v_large = pr.calculate_pressure(t, v_large);
        let p_at_v_small = pr.calculate_pressure(t, v_small);

        println!("\n[Thermo-Consistency-Log] Isotherme Kompression bei {} K:", t);
        println!("  -> Druck bei {} m³/mol: {:.2} Pa", v_large, p_at_v_large);
        println!("  -> Druck bei {} m³/mol: {:.2} Pa", v_small, p_at_v_small);

        // Der komprimierte Zustand MUSS einen höheren Druck aufweisen
        assert!(p_at_v_small > p_at_v_large);
    }

    #[test]
    fn test_vdw_vs_ideal_gas_deviation() {
        // Validierung des Realgas-Faktors:
        // Bei hohem Druck (nahe dem kritischen Punkt) müssen sich die Realgas-Modelle (vdW)
        // deutlich vom idealen Gas unterscheiden, da Kohäsionskräfte und Eigenvolumen wirken.
        let t = 310.0; // Knapp über der kritischen Temperatur von CO2
        let v = 0.0002; // Sehr dichtes Gas (0.2 Liter/mol)

        let ig = IdealGas::default();
        let vdw = VanDerWaals::new(CO2_DATA.vdw_a, CO2_DATA.vdw_b);

        let p_ideal = ig.calculate_pressure(t, v);
        let p_vdw = vdw.calculate_pressure(t, v);

        println!("\n[Thermo-Deviation-Log] Realgas-Effekt nahe dem kritischen Punkt:");
        println!("  -> Ideales Gas Druck:  {:.2} Pa", p_ideal);
        println!("  -> Van-der-Waals Druck: {:.2} Pa", p_vdw);

        // Aufgrund des starken Kovolumens (Teilchennähe) müsste der vdW-Druck hier 
        // massiv vom idealen Gas abweichen (typischerweise stark ansteigen)
        let prozentuale_abweichung = ((p_vdw - p_ideal) / p_ideal).abs();
        assert!(prozentuale_abweichung > 0.10); // Mehr als 10% Abweichung zum Idealgas
    }

     #[test]
    fn test_volume_solving_boundaries_and_errors() {
        // HÄRTETEST FÜR DIE ROOTS-ENGINE:
        // Wir wählen ein Intervall, in dem sich das Vorzeichen NICHT ändert (keine Nullstelle vorhanden).
        // Das System MUSS dies erkennen und ein sauberes `Err` zurückgeben, statt abzustürzen.
        let t = 300.0;
        let pr = PengRobinson::new(CO2_DATA.tc, CO2_DATA.pc, CO2_DATA.omega);

        // Wir fordern einen extrem hohen Zieldruck von 100 MPa
        let target_p = 100.0e6; 

        // Wir wählen ein Intervall im extrem dünnen Gasbereich (große Volumina)
        // Hier ist der wahre Druck immer winzig (nahe 0), die Differenz (target_p - p) bleibt immer positiv!
        let v_start = 1.0; // 1 m³/mol
        let v_end = 2.0;   // 2 m³/mol

        let result = calculate_molar_volume(&pr, t, target_p, v_start, v_end);

        println!("\n[Roots-Safety-Log] Roots-Engine fing ungültige physikalische Grenzen sauber ab:");
        if let Err(reason) = result {
            println!("  -> Erkannter Fehler-Grund: '{}'", reason);
        }

        // Die Assertion erwartet nun garantiert ein echtes mathematisches 'Err'
        assert!(result.is_err());
    }
}