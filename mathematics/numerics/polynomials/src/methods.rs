//! Mathematical algorithm for polynoms (Horner, Estrin, Differentiation).
//! #[cfg(target_arch = "x86_64")]
use core::arch::x86_64::*;
use core_simd::{get_optimal_simd_level, SimdLevel};
use num_traits::{Float, FromPrimitive};

/// Trait respective Hardware selection
pub trait SimdEvaluator: Float + FromPrimitive {
    // 1. Standard Horner / Potenzreihen
    fn evaluate_accelerated(coefficients: &[Self], x: Self) -> Self;
    
    // 2. Chebyshev-Polynome
    fn evaluate_chebyshev(coefficients: &[Self], x: Self) -> Self;
    
 /*    
    // Hermite polynome (not yet implemented)
    fn evaluate_hermite(coefficients: &[Self], x: Self) -> Self;
*/
}

impl SimdEvaluator for f64 {
    #[inline]
    fn evaluate_accelerated(coefficients: &[Self], x: Self) -> Self {
        match get_optimal_simd_level() {
            SimdLevel::Avx512 | SimdLevel::Avx2 => unsafe { evaluate_horner_avx2_f64(coefficients, x) },
            SimdLevel::Scalar => evaluate_horner(coefficients, x),
        }
    }

    #[inline]
    fn evaluate_chebyshev(coefficients: &[Self], x: Self) -> Self {
        match get_optimal_simd_level() {
            //SimdLevel::Avx512 | SimdLevel::Avx2 => unsafe { evaluate_chebyshev_avx2_f64(coefficients, x) },
            SimdLevel::Scalar => evaluate_chebyshev(coefficients, x),
            _ => { evaluate_chebyshev(coefficients, x) }
        }
    }
/*
    #[inline]
    fn evaluate_hermite(coefficients: &[Self], x: Self) -> Self {
        match get_optimal_simd_level() {
            SimdLevel::Avx512 | SimdLevel::Avx2 => unsafe { evaluate_hermite_avx2_f64(coefficients, x) },
            SimdLevel::Scalar => evaluate_hermite_scalar(coefficients, x),
        }
        */
}


/// f32 implementation with AVX2 (not yet AVX512)
impl SimdEvaluator for f32 {
    #[inline]
    fn evaluate_accelerated(coefficients: &[Self], x: Self) -> Self {
        match get_optimal_simd_level() {
            SimdLevel::Avx512 | SimdLevel::Avx2 => {
                unsafe { evaluate_horner_avx2_f32(coefficients, x) }
            }
            SimdLevel::Scalar => evaluate_horner(coefficients, x)
        }
    }

    #[inline]
    fn evaluate_chebyshev(coefficients: &[Self], x: Self) -> Self {
        match get_optimal_simd_level() {
            SimdLevel::Avx512 | SimdLevel::Avx2 => {
                // TODO: Später durch AVX2-Implementierung für Chebyshev ersetzen
                evaluate_chebyshev(coefficients, x)
            }
            SimdLevel::Scalar => evaluate_chebyshev(coefficients, x)
        }
    }
}


/// Classic Horner schema (O(n) operations, optimal für serial processing)
#[inline]
pub fn evaluate_horner<T>(coefficients: &[T], x: T) -> T
where
    T: Float + FromPrimitive,
{
    let mut result = T::zero();
    // Startet beim höchsten Grad c_n und läuft absteigend bis c_0
    for &c in coefficients.iter().rev() {
        result = result * x + c;
    }
    result
}

/// AVX2-optimized Horner schema for f64
/// # Safety 
#[cfg(target_arch = "x86_64")]
#[target_feature(enable = "avx2,fma")]
pub unsafe fn evaluate_horner_avx2_f64(coefficients: &[f64], x: f64) -> f64 {
    use std::arch::x86_64::*;

    let n = coefficients.len();
    if n == 0 {
        return 0.0;
    }
    if n < 4 {
        return evaluate_horner(coefficients, x);
    }

    let mut i = n;
    let mut result = 0.0;

    // 1. Skalarer Rest vom Ende her, bis die Anzahl durch 4 teilbar ist
    while !i.is_multiple_of(4) {
        i -= 1;
        result = result * x + coefficients[i];
    }

    // 2. AVX2 for 4 blocks
    let vx4 = _mm256_set1_pd(x.powi(4));
    let mut acc = _mm256_setzero_pd();

    while i >= 4 {
        i -= 4;
        // Sicherheits-Block für rohe Pointer-Operationen (Clippy-konform explizit gekapselt)
        let c_chunk = unsafe { _mm256_loadu_pd(coefficients.as_ptr().add(i)) };
        acc = _mm256_fmadd_pd(acc, vx4, c_chunk);
    }

    let mut temp = [0.0; 4];
    unsafe {
        _mm256_storeu_pd(temp.as_mut_ptr(), acc);
    }

    let mut vec_res = 0.0;
    for j in (0..4).rev() {
        vec_res = vec_res * x + temp[j];
    }

    result = result * x.powi(i as i32) + vec_res;
    result
}

/// AVX2-optimiertized Horner schema for f32
/// # Safety
#[cfg(target_arch = "x86_64")]
#[target_feature(enable = "avx2,fma")]
pub unsafe fn evaluate_horner_avx2_f32(coefficients: &[f32], x: f32) -> f32 {
    use std::arch::x86_64::*;

    let n = coefficients.len();
    if n == 0 {
        return 0.0;
    }
    if n < 8 {
        return evaluate_horner(coefficients, x);
    }

    let mut i = n;
    let mut result = 0.0f32;

    // 1. Skalarer Rest vom Ende her, bis die Anzahl durch 8 teilbar ist
    while !i.is_multiple_of(8) {
        i -= 1;
        result = result * x + coefficients[i];
    }

    // 2. AVX2 8er-Blöcke
    let vx8 = _mm256_set1_ps(x.powi(8));
    let mut acc = _mm256_setzero_ps();

    while i >= 8 {
        i -= 8;
        // Sicherheits-Block für rohe Pointer-Operationen (Clippy-konform explizit gekapselt)
        let c_chunk = unsafe { _mm256_loadu_ps(coefficients.as_ptr().add(i)) };
        acc = _mm256_fmadd_ps(acc, vx8, c_chunk);
    }

    let mut temp = [0.0f32; 8];
    unsafe {
        _mm256_storeu_ps(temp.as_mut_ptr(), acc);
    }

    let mut vec_res = 0.0f32;
    for j in (0..8).rev() {
        vec_res = vec_res * x + temp[j];
    }

    result = result * x.powi(i as i32) + vec_res;
    result
}


/// Estrins Scheme (Grouping for optimal processin gof Instruction-Level Parallelism)
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

/// Horner schema mit simultanious differentiatation
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

/// Formal Differentiation of coefficients
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

//
//  --- Chebychev with Clenshaw algorithm ---
//
/// Using Clenshaw algorithm to evaluate Chebyshev series at x.
/// (basic scalare implementation)
#[inline]
pub fn evaluate_chebyshev<T>(coefficients: &[T], x: T) -> T
where
    T: num_traits::Float + num_traits::FromPrimitive,
{
    if coefficients.is_empty() {
        return T::zero();
    }
    if coefficients.len() == 1 {
        return coefficients[0];
    }

    let two = T::from_f64(2.0).unwrap();
    let mut d_next_2 = T::zero(); // entspricht d_{k+2}
    let mut d_next_1 = T::zero(); // entspricht d_{k+1}

    // Clenshaw-Rekursion läuft rückwärts vom letzten Koeffizienten bis k = 1
    for &c in coefficients.iter().skip(1).rev() {
        let d_current = c + (x * two * d_next_1) - d_next_2;
        d_next_2 = d_next_1;
        d_next_1 = d_current;
    }

    // Rsult for  Clenshaw: c_0 + x * d_1 - d_2
    coefficients[0] + (x * d_next_1) - d_next_2
}

//
//  --- unit tests ---
//

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