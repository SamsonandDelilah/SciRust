# SciRust :: Astronomy 🌌

[![Rust Edition 2024](https://shields.io)](https://rust-lang.org)
[![Domain: Physics/Astronomy](https://shields.io)]()

The `astronomy` package delivers targeted, high-precision astronomical time tracking and solar orbital mechanics within the **SciRust** ecosystem. By leveraging the standardized planetary ephemeris framework, it establishes the ultimate astronomical baseline required for downstream calculations like dynamic calendar engines and planetary tracking systems.

---

## 🏛️ Functional Reference & Core Engine

Following the **SciRust Single Source of Truth (SSOT)** architectural doctrine, this engine is optimized for raw mathematical execution using hard-coded `f64` boundaries. It bypasses heavy structural wrappers to ensure zero-overhead performance directly on modern CPU registers.

### ⏱️ Chronological Reference (`core_astronomy.rs`)

*   **`julian_day_from_local(year, month, day, hour, min, sec, tz) -> f64`**
    *   **Explanation**: Computes the exact continuous **Julian Day (JD)** counter from raw local calendar components.
    *   **Mechanics**: Implements a strict mathematical timeline transformation. It features hard-coded floor-division corrections for the **Gregorian Calendar leap shifts** implemented after October 15, 1582, ensuring high chronological integrity.

### 🪐 Ephemeris Analytics

*   **`sun_ecliptic_longitude(jd) -> f64`**
    *   **Explanation**: Computes the true **apparent ecliptic longitude** ($\lambda$) of the Sun relative to the Earth.
    *   **Mechanics**: Directly queries the highly precise, semi-analytical **`vsop87e`** planetary theory model. It extracts the Earth’s geocentric rectangular coordinate vectors ($x$, $y$) at the exact Julian fraction, computes the trigonometric angular sweep via raw `atan2`, and normalizes the final result to a strict circular degree sweep ($[0^\circ, 360^\circ[$).

---

## 🏎️ Engineering Integration

This specialized engine relies on external ephemeris crates for heavy series expansions, making it extremely lightweight and focused:

*   **VSOP87 Integration**: Natively processes spatial perturbations without dynamic vector allocation, making it exceptionally fast inside deep iterative loops.
*   **Drift-Free Precision**: Explicit floating-point floor truncations guarantee perfect sync between astronomical timeline anchors and civilian calendar layers.

---

## 🛠️ Usage Example

```rust
use astronomy::{julian_day_from_local, sun_ecliptic_longitude};

fn main() {
    // 1. Calculate an absolute Julian Day anchor (e.g., July 24, 2026, 12:00:00)
    let jd = julian_day_from_local(2026, 7, 24, 12, 0, 0, "UTC");
    
    // 2. Query the exact apparent position of the Sun
    let longitude = sun_ecliptic_longitude(jd);
    
    println!("Sun apparent ecliptic longitude at JD {}: {:.4}°", jd, longitude);
}
```

---

## ⚖️ License

This package is part of the **SciRust** workspace and is dual-licensed under the **MIT License**.