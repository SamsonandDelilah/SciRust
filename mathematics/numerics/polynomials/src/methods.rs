//! Mathematische Algorithmen für Polynome (Horner, Estrin, Ableitung).
#[cfg(target_arch = "x86_64")]
use core::arch::x86_64::*;
use core_simd::{get_optimal_simd_level, SimdLevel};
use num_traits::{Float, FromPrimitive};

/// Ein Trait, das die hardwarenahe Auswertung kapselt
pub trait SimdEvaluator: Float + FromPrimitive {
    fn evaluate_accelerated(coefficients: &[Self], x: Self) -> Self;
}

// Implementierung für f64 (mit echtem AVX2-Turbo)
impl SimdEvaluator for f64 {
    #[inline]
    fn evaluate_accelerated(coefficients: &[Self], x: Self) -> Self {
        match get_optimal_simd_level() {
            SimdLevel::Avx512 | SimdLevel::Avx2 => {
                unsafe { evaluate_horner_avx2_f64(coefficients, x) }
            }
            SimdLevel::Scalar => {
                evaluate_horner(coefficients, x)
            }
        }
    }
}

// Implementierung für f32 (mit echtem AVX2-Turbo)
impl SimdEvaluator for f32 {
    #[inline]
    fn evaluate_accelerated(coefficients: &[Self], x: Self) -> Self {
        match get_optimal_simd_level() {
            SimdLevel::Avx512 | SimdLevel::Avx2 => {
                unsafe { evaluate_horner_avx2_f32(coefficients, x) }
            }
            SimdLevel::Scalar => {
                evaluate_horner(coefficients, x)
            }
        }
    }
}


/// Klassisches Horner-Schema (O(n) Operationen, optimal für serielle Ausführung)
#[inline]
pub fn evaluate_horner<T>(coefficients: &[T], x: T) -> T
where
    T: Float + FromPrimitive,
{
    let mut result = T::zero();
    let mut iter = coefficients.iter().rev();
    if let Some(&c) = iter.next() {
        result = c;
        for &c in iter {
            result = result * x + c;
        }
    }
    result
}

/// AVX2-optimiertes Horner-Schema für f64
/// # Safety
#[cfg(target_arch = "x86_64")]
#[target_feature(enable = "avx2,fma")]
pub unsafe fn evaluate_horner_avx2_f64(coefficients: &[f64], x: f64) -> f64 {
    if coefficients.is_empty() {
        return 0.0;
    }

    let n = coefficients.len();
    let mut acc =  _mm256_setzero_pd();
    let vx =  _mm256_set1_pd(x);

    let chunk_size = 4;
    let mut i = 0;

    while i + chunk_size <= n {
        // Hier das unsafe für das Laden und FMA ergänzen:
        let c_chunk = unsafe { _mm256_loadu_pd(coefficients.as_ptr().add(i)) };
        acc = _mm256_fmadd_pd(acc, vx, c_chunk);
        
        i += chunk_size;
    }

    let mut temp = [0.0; 4];
    unsafe { _mm256_storeu_pd(temp.as_mut_ptr(), acc); }
    
    let mut result = temp[0] * x.powi(3) + temp[1] * x.powi(2) + temp[2] * x + temp[3];

    while i < n {
        result = result * x + coefficients[i];
        i += 1;
    }

    result
}


/// AVX2-optimiertes Horner-Schema für f32
/// # Safety
#[cfg(target_arch = "x86_64")]
#[target_feature(enable = "avx2,fma")]
pub unsafe fn evaluate_horner_avx2_f32(coefficients: &[f32], x: f32) -> f32 {
    if coefficients.is_empty() {
        return 0.0;
    }

    let n = coefficients.len();
    let mut acc = _mm256_setzero_ps();
    let vx = _mm256_set1_ps(x);

    // Wir verarbeiten 8 Koeffizienten gleichzeitig im 256-Bit Vektor (f32)
    let chunk_size = 8;
    let mut i = 0;

    while i + chunk_size <= n {
        // Lade 8 Koeffizienten unaligned in ein YMM-Register
        let c_chunk= unsafe { _mm256_loadu_ps(coefficients.as_ptr().add(i)) };
        
        // Horner-Schritt mit FMA für f32
        acc = _mm256_fmadd_ps(acc, vx, c_chunk);
        
        i += chunk_size;
    }

    // Extrahiere die 8 Werte aus dem Vektor und summiere sie skalar auf
    let mut temp = [0.0f32; 8];
    unsafe { _mm256_storeu_ps(temp.as_mut_ptr(), acc); }
    
    // Horner-Auswertung über die akkumulierten Vektorelemente
    let mut result = 0.0f32;
    for &val in &temp {
        result = result * x.powi(8) + val; // Korrekte Skalierung für den 8er-Block
    }

    // Rest-Koeffizienten (falls n nicht durch 8 teilbar ist) skalar verarbeiten
    while i < n {
        result = result * x + coefficients[i];
        i += 1;
    }

    result
}


/// Estrins Schema (Gruppierung zur besseren Ausnutzung von Instruction-Level Parallelism)
pub fn evaluate_estrin<T>(coefficients: &[T], x: T) -> T
where
    T: Float + FromPrimitive,
{
    let n = coefficients.len();
    if n == 0 {
        return T::zero();
    }
    if n == 1 {
        return coefficients[0];
    }
    if n == 2 {
        return coefficients[0] + coefficients[1] * x;
    }

    let x2 = x * x;
    
    // Einfache Baumstruktur für Estrin (rekursiv oder blockweise aufgebaut)
    // Beispiel für ein Basis-Estrin auf Paaren:
    let mut acc = T::zero();
    let mut x_power = T::one();

    let mut i = 0;
    while i < n {
        let term_low = coefficients[i];
        let term_high = if i + 1 < n { coefficients[i + 1] } else { T::zero() };
        
        let pair_val = term_low + term_high * x;
        acc = acc + pair_val * x_power;

        x_power = x_power * x2;
        i += 2;
    }
    
    acc
}

/// Horner-Schema mit simultaner Ableitung
pub fn evaluate_with_derivative<T>(coefficients: &[T], x: T) -> (T, T)
where
    T: Float + FromPrimitive,
{
    if coefficients.is_empty() {
        return (T::zero(), T::zero());
    }

    let n = coefficients.len();
    let mut p = *coefficients.last().unwrap();
    let mut dp = T::zero();

    for i in (0..n - 1).rev() {
        dp = dp * x + p;
        p = p * x + coefficients[i];
    }

    (p, dp)
}

/// Formale Ableitung der Koeffizienten
pub fn differentiate_coefficients<T>(coefficients: &[T]) -> Vec<T>
where
    T: Float + FromPrimitive,
{
    if coefficients.len() <= 1 {
        return vec![T::zero()];
    }

    let mut derived = Vec::with_capacity(coefficients.len() - 1);
    for (i, &c) in coefficients.iter().enumerate().skip(1) {
        let factor = T::from_usize(i).unwrap();
        derived.push(c * factor);
    }
    derived
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_evaluate_horner() {
        // p(x) = 3 + 2x + 1x^2
        let poly = vec![3.0, 2.0, 1.0];
        
        // p(2) = 3 + 4 + 4 = 11
        let val = evaluate_horner(&poly, 2.0);
        assert_eq!(val, 11.0);

        // p(0) = 3
        let val_zero = evaluate_horner(&poly, 0.0);
        assert_eq!(val_zero, 3.0);
    }

    #[test]
    fn test_evaluate_estrin() {
        // p(x) = 1 - 2x + 3x^2 - 4x^3 + 5x^4
        let poly = vec![1.0, -2.0, 3.0, -4.0, 5.0];
        let x = 2.0;

        let horner_res = evaluate_horner(&poly, x);
        let estrin_res = evaluate_estrin(&poly, x);

        assert!((horner_res - estrin_res).abs() < 1e-12);
    }

    #[test]
    fn test_evaluate_with_derivative() {
        // p(x) = 3 + 2x + 1x^2  =>  p'(x) = 2 + 2x
        let poly = vec![3.0, 2.0, 1.0];
        let x = 2.0;

        // p(2) = 11, p'(2) = 6
        let (p_val, dp_val) = evaluate_with_derivative(&poly, x);
        
        assert_eq!(p_val, 11.0);
        assert_eq!(dp_val, 6.0);
    }

    #[test]
    fn test_differentiate_coefficients() {
        // p(x) = 5 + 3x + 4x^2 + 2x^3  =>  p'(x) = 3 + 8x + 6x^2
        let poly = vec![5.0, 3.0, 4.0, 2.0];
        let derived = differentiate_coefficients(&poly);

        assert_eq!(derived, vec![3.0, 8.0, 6.0]);
    }

    #[test]
    fn test_constant_and_empty_polynomials() {
        // Konstantes Polynom: p(x) = 7
        let const_poly = vec![7.0];
        assert_eq!(evaluate_horner(&const_poly, 10.0), 7.0);
        assert_eq!(differentiate_coefficients(&const_poly), vec![0.0]);

        // Leeres Polynom
        let empty_poly: Vec<f64> = vec![];
        assert_eq!(evaluate_horner(&empty_poly, 5.0), 0.0);
        assert_eq!(evaluate_with_derivative(&empty_poly, 5.0), (0.0, 0.0));
    }
}