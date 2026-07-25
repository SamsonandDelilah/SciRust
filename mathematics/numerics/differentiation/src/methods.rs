//! Numerische Ableitungsverfahren für SciRust.
//! Vollständig generisch via Float, Zero, One, FromPrimitive (num_traits).

use num_traits::{Float, FromPrimitive};
use num_complex::Complex;

/// 6. Komplexe Schrittweiten-Methode (Complex-Step Derivative)
///    Eliminiert den numerischen Auslöschungsfehler komplett. 
///    Erreicht quasi maschinelle Präzision ohne Wahl eines optimalen h.
pub fn complex_step_derivative<T, F>(f: &F, x: T) -> T
where
    T: Float + FromPrimitive,
    F: Fn(Complex<T>) -> Complex<T>,
{
    // Extrem kleines h ist hier unproblematisch (keine Auslöschung!)
    let h = T::from_f64(1e-20).unwrap();
    let x_c = Complex::new(x, h);
    
    let fx_c = f(x_c);
    
    // Ableitung = Imaginärteil von f(x + ih) geteilt durch h
    fx_c.im / h
}

/// 1. Vorwärts-Differenzenquotient (O(h))
#[inline]
pub fn forward_difference<T, F>(f: &F, x: T, h: T) -> T
where
    T: Float,
    F: Fn(T) -> T,
{
    (f(x + h) - f(x)) / h
}

/// 2. Rückwärts-Differenzenquotient (O(h))
#[inline]
pub fn backward_difference<T, F>(f: &F, x: T, h: T) -> T
where
    T: Float,
    F: Fn(T) -> T,
{
    (f(x) - f(x - h)) / h
}

/// 3. Zentrale Differenzenquotient (O(h^2)) - Standard für hohe Präzision
#[inline]
pub fn central_difference<T, F>(f: &F, x: T, h: T) -> T
where
    T: Float + FromPrimitive,
    F: Fn(T) -> T,
{
    let two = T::from_u8(2).unwrap();
    (f(x + h) - f(x - h)) / (two * h)
}

/// 4. Fünf-Punkte-Stencil / High-Order Central (O(h^4))
///    Vorbild: SciPy / Boost
pub fn high_order_central_5<T, F>(f: &F, x: T, h: T) -> T
where
    T: Float + FromPrimitive,
    F: Fn(T) -> T,
{
    //let two = T::from_u8(2).unwrap();
    let eight = T::from_u8(8).unwrap();
    let twelve = T::from_u8(12).unwrap();

    // Formel: (-f(x+2h) + 8f(x+h) - 8f(x-h) + f(x-2h)) / (12h)
    let term = -f(x + h + h) + eight * f(x + h) - eight * f(x - h) + f(x - h - h);
    term / (twelve * h)
}

/// 5. Adaptive Richardson-Extrapolation (Basiert auf zentralem Differenzenquotienten)
///    Minimiert Auslöschung und Diskretisierungsfehler automatisch.
pub fn adaptive_differentiation<T, F>(f: &F, x: T) -> (T, T)
where
    T: Float + FromPrimitive,
    F: Fn(T) -> T,
{
    let h = T::from_f64(1e-4).unwrap();
    let two = T::from_u8(2).unwrap();
    let four = T::from_u8(4).unwrap();
    
    let d1 = central_difference(f, x, h);
    let mut h_curr = h;
    
    // Einfache iterativer Schritt zur Verfeinerung
    h_curr = h_curr / two;
    let d2 = central_difference(f, x, h_curr);
    
    // Richardson O(h^4) Extrapolation aus zwei O(h^2) Schritten
    let refined = (four * d2 - d1) / (four - T::one());
    
    let error_est = (refined - d2).abs();
    (refined, error_est)
}