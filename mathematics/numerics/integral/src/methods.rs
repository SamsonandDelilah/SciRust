//! Pure mathematische Algorithmen für die numerische Integration auf SciRust-Plattform-Niveau.
//! Vollständig generisch implementiert über Trait Bounds (Static Dispatch).

use num_traits::{Float, FromPrimitive};

/// 1. Gauß-Kronrod 15-Punkt-Quadratur mit mathematischer Fehlerschätzung.
///    Berechnet simultan ein 7-Punkt-Gauß- und ein 15-Punkt-Kronrod-Integral.
pub fn gauss_kronrod_15<T, F>(f: &F, a: T, b: T) -> (T, T)
where
    T: Float + FromPrimitive,
    F: Fn(T) -> T,
{
    let x_gk = [
        T::from_f64(0.0).unwrap(),
        T::from_f64(0.2077849550078985).unwrap(),
        T::from_f64(0.4058451513773972).unwrap(),
        T::from_f64(0.5860872354676911).unwrap(),
        T::from_f64(0.7415311855993944).unwrap(),
        T::from_f64(0.8648644233597691).unwrap(),
        T::from_f64(0.9491079123427585).unwrap(),
        T::from_f64(0.9914553711208126).unwrap(),
    ];

    let w_k = [
        T::from_f64(0.2094820417845704).unwrap(),
        T::from_f64(0.2044329400752989).unwrap(),
        T::from_f64(0.1903505780647854).unwrap(),
        T::from_f64(0.1690047266392679).unwrap(),
        T::from_f64(0.1406532595855059).unwrap(),
        T::from_f64(0.1047900103222502).unwrap(),
        T::from_f64(0.0630920926299786).unwrap(),
        T::from_f64(0.0229353220105292).unwrap(),
    ];

    let w_g = [
        T::from_f64(0.4179591836734694).unwrap(),
        T::from_f64(0.3818300505051189).unwrap(),
        T::from_f64(0.2797053914892767).unwrap(),
        T::from_f64(0.1294849661688697).unwrap(),
    ];

    let zero_five = T::from_f64(0.5).unwrap();
    let cent = zero_five * (b + a);
    let h = zero_five * (b - a);

    // KORREKTUR: Initialisierung als Skalar-Werte (Typ T), multipliziert mit dem jeweils ersten Gewicht [0]
    let f_cent = f(cent);
    let mut gk_sum = f_cent * w_k[0];
    let mut g_sum = f_cent * w_g[0];

    for i in 1..8 {
        let p = h * x_gk[i];
        let f_sum = f(cent + p) + f(cent - p);
        
        gk_sum = gk_sum + f_sum * w_k[i];
        if i % 2 == 0 {
            g_sum = g_sum + f_sum * w_g[i / 2];
        }
    }

    let integral = gk_sum * h;
    let error = (integral - (g_sum * h)).abs();

    (integral, error)
}

/// 2. Global adaptive Quadratur.
///    Teilt das Intervall rekursiv dort auf, wo die lokale Fehlerschätzung die Toleranz bricht.
pub fn adaptive_quadrature<T, F>(f: &F, a: T, b: T, tol: T, max_depth: u32) -> T
where
    T: Float + FromPrimitive,
    F: Fn(T) -> T,
{
    let (res, err) = gauss_kronrod_15(f, a, b);
    
    if err <= tol || max_depth == 0 {
        return res;
    }

    let zero_five = T::from_f64(0.5).unwrap();
    let mid = zero_five * (a + b);
    let half_tol = zero_five * tol;

    adaptive_quadrature(f, a, mid, half_tol, max_depth - 1) 
        + adaptive_quadrature(f, mid, b, half_tol, max_depth - 1)
}

/// 3. Double Exponential (DE) Integration (Tanh-Sinh Quadratur).
///    Transformiert das Intervall so, dass Singularitäten an den Rändern komplett verschwinden.
pub fn ee_integral<T, F>(f: &F, a: T, b: T, _tol: T) -> T
where
    T: Float + FromPrimitive,
    F: Fn(T) -> T,
{
    let zero_five = T::from_f64(0.5).unwrap();
    let cent = zero_five * (b + a);
    let h_half = zero_five * (b - a);
    
    let mut sum = T::zero();
    let h_step = T::from_f64(0.1).unwrap();
    let pi_by_two = T::from_f64(std::f64::consts::FRAC_PI_2).unwrap();

    for n in -30..=30 {
        let t = T::from_i32(n).unwrap() * h_step;
        let sinh_t = t.sinh();
        let x = cent + h_half * (pi_by_two * sinh_t).tanh();
        
        let cosh_t = t.cosh();
        let cosh_pi_2_sinh = (pi_by_two * sinh_t).cosh();
        let weight = (pi_by_two * cosh_t) / (cosh_pi_2_sinh * cosh_pi_2_sinh);
        
        let fx = f(x);
        if fx.is_finite() && !fx.is_nan() {
            sum = sum + fx * weight;
        }
    }
    
    sum * h_half * h_step
}

/// 4. Symbolischer CAS-Integrator (Computeralgebra-Platzhalter).
///
/// Versucht, die Stammfunktion analytisch über ein mathematisches Regelwerk zu bestimmen.
/// Gibt im aktuellen Entwicklungsstadium (Dummy) eine Fehlermeldung zurück.
pub fn symbolic_integral<T>(_expression: &str, _a: T, _b: T) -> Result<T, &'static str> {
    Err("Symbolische CAS-Integration in SciRust befindet sich noch in der Entwicklung (Stufe: Vorbereitet).")
}

/// 5. Erweiterte Trapez-Regel für kontinuierliche Funktionen oder diskrete Daten.
///
/// Berechnet das Integral über eine feste Anzahl an Intervallen (Schritten).
pub fn trapezoidal_integral<T, F>(f: &F, a: T, b: T, steps: usize) -> T
where
    T: Float + FromPrimitive,
    F: Fn(T) -> T,
{
    if steps == 0 { return T::zero(); }
    
    let n = T::from_usize(steps).unwrap();
    let h = (b - a) / n;
    
    let mut sum = T::from_f64(0.5).unwrap() * (f(a) + f(b));
    
    for i in 1..steps {
        let x = a + T::from_usize(i).unwrap() * h;
        sum = sum + f(x);
    }
    
    sum * h
}

/// 6. Erweiterte Simpson-Regel (Composite Simpson) für schnelle Abschätzungen.
///
/// Erfordert eine gerade Anzahl an Intervallen (steps). Wenn ungerade, wird steps um 1 erhöht.
pub fn simpson_integral<T, F>(f: &F, a: T, b: T, steps: usize) -> T
where
    T: Float + FromPrimitive,
    F: Fn(T) -> T,
{
    if steps == 0 { return T::zero(); }
    // Simpson benötigt eine gerade Anzahl an Intervallen
    let adjusted_steps = if !steps.is_multiple_of(2) { steps + 1 } else { steps };
    
    let n = T::from_usize(adjusted_steps).unwrap();
    let h = (b - a) / n;
    
    let mut sum = f(a) + f(b);
    let two = T::from_f64(2.0).unwrap();
    let four = T::from_f64(4.0).unwrap();
    
    for i in 1..adjusted_steps {
        let x = a + T::from_usize(i).unwrap() * h;
        if i % 2 == 0 {
            sum = sum + two * f(x);
        } else {
            sum = sum + four * f(x);
        }
    }
    
    sum * (h / T::from_f64(3.0).unwrap())
}

/// 7. Klassische Romberg-Integration für glatte Funktionen.
///
/// Nutzt die Richardson-Extrapolations-Matrix auf Basis der Trapezregel für beschleunigte Konvergenz.
pub fn romberg_integral<T, F>(f: &F, a: T, b: T, tol: T) -> T
where
    T: Float + FromPrimitive,
    F: Fn(T) -> T,
{
    let max_steps = 6; // Numerisch stabiles Maximum für die Matrixgröße
    let mut r = vec![vec![T::zero(); max_steps]; max_steps];
    
    // Erste Zeile initialisieren (Trapezregel mit 1 Intervall)
    r[0][0] = trapezoidal_integral(f, a, b, 1);
    
    let two = T::from_f64(2.0).unwrap();
    
    for i in 1..max_steps {
        // Schrittweise Verdopplung der Intervalle
        let steps = 1 << i; 
        r[i][0] = trapezoidal_integral(f, a, b, steps);
        
        // Richardson-Extrapolation
        for j in 1..=i {
            let factor = T::from_f64(4.0f64.powi(j as i32) - 1.0).unwrap();
            r[i][j] = r[i][j-1] + (r[i][j-1] - r[i-1][j-1]) / factor;
        }
        
        // Konvergenz-Prüfung gegen die Toleranzschranke
        if (r[i][i] - r[i-1][i-1]).abs() < tol {
            return r[i][i];
        }
    }
    
    r[max_steps - 1][max_steps - 1]
}

/// 8. Diskreter Vektor-Integrator für unregelmäßige oder äquidistante Labormesswerte.
///
/// Erkennt vollautomatisch (O(N)), ob die Datenpunkte gleichmäßig verteilt sind,
/// und wählt autonom das präzisere Simpson-Verfahren oder die robuste Trapezregel.
pub fn discrete_vector_integral<T>(x: &[T], y: &[T]) -> Result<T, &'static str>
where
    T: Float + FromPrimitive,
{
    if x.len() != y.len() || x.is_empty() {
        return Err("Dimensionen der X- und Y-Messreihen stimmen nicht überein.");
    }
    if x.len() < 2 {
        return Ok(T::zero());
    }

    // Äquidistanz-Prüfung (O(N))
    let step = x[1] - x[0];
    let is_equidistant = x.windows(2).all(|w| (w[1] - w[0] - step).abs() < T::from_f64(1e-5).unwrap());

    if is_equidistant && x.len() >= 3 {
        // Äquidistante Messreihe -> Simpson-Summenregel zünden
        let h = step;
        let n = x.len() - 1;
        let mut sum = y[0] + y[n];
        let two = T::from_f64(2.0).unwrap();
        let four = T::from_f64(4.0).unwrap();

        for i in 1..n {
            if i % 2 == 1 {
                sum = sum + four * y[i];
            } else {
                sum = sum + two * y[i];
            }
        }
        Ok((h / T::from_f64(3.0).unwrap()) * sum)
    } else {
        // Unregelmäßige Messreihe -> Robuste Trapez-Regel nutzen
        let mut sum = T::zero();
        let zero_five = T::from_f64(0.5).unwrap();
        for i in 0..x.len() - 1 {
            sum = sum + zero_five * (x[i+1] - x[i]) * (y[i] + y[i+1]);
        }
        Ok(sum)
    }
}