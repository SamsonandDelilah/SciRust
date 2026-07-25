//! Zentrale SIMD-Hardware-Erkennung und Caching für SciRust.
//! Ermittelt einmalig zur Laufzeit die verfügbaren CPU-Instruction-Sets.

use std::sync::OnceLock;

/// Unterstützte SIMD-Architektur-Ebenen
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SimdLevel {
    Scalar,
    Avx2,
    Avx512,
}

/// Ermittelt das optimale SIMD-Level der CPU und cached das Ergebnis thread-sicher.
pub fn get_optimal_simd_level() -> SimdLevel {
    static SIMD_LEVEL: OnceLock<SimdLevel> = OnceLock::new();
    
    *SIMD_LEVEL.get_or_init(|| {
        if cfg!(target_arch = "x86_64") {
            #[cfg(target_arch = "x86_64")]
            {
                if std::is_x86_feature_detected!("avx512f") {
                    return SimdLevel::Avx512;
                }
                if std::is_x86_feature_detected!("avx2") {
                    return SimdLevel::Avx2;
                }
            }
        }
        SimdLevel::Scalar
    })
}