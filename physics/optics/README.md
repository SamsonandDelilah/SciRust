# Planck Radiation Module

This module implements Planck's law of thermal radiation for an ideal black body in SI units. It provides both the frequency form $B_\nu(\nu,T)$ and the wavelength form $B_\lambda(\lambda,T)$, including analytical total power, numerical band integrals, and both peak definitions (Wien's displacement law).

---

## Physical Foundations

### Planck's Law (Frequency Form)

The spectral radiance per frequency interval is:

$$B_\nu(\nu,T) = \frac{2h\nu^3}{c^2} \cdot \frac{1}{e^{h\nu/(k_B T)} - 1}$$

* $B_\nu$: spectral radiance, unit $\mathrm{W\,m^{-2}\,sr^{-1}\,Hz^{-1}}$
* $\nu$: frequency in $\mathrm{Hz}$
* $T$: temperature in $\mathrm{K}$
* $h$: Planck constant ($6.62607015\times10^{-34}\ \mathrm{J\,s}$)
* $c$: speed of light in vacuum ($299792458\ \mathrm{m/s}$)
* $k_B$: Boltzmann constant ($1.380649\times10^{-23}\ \mathrm{J/K}$)

### Planck's Law (Wavelength Form)

The spectral radiance per wavelength interval is:

$$B_\lambda(\lambda,T) = \frac{2hc^2}{\lambda^5} \cdot \frac{1}{e^{hc/(\lambda k_B T)} - 1}$$

* $B_\lambda$: spectral radiance, unit $\mathrm{W\,m^{-2}\,sr^{-1}\,m^{-1}}$
* $\lambda$: wavelength in $\mathrm{m}$

### Relationship Between $B_\nu$ and $B_\lambda$ (Jacobian)

Both forms describe the same physical radiation, expressed in different spectral variables. For $\nu = c/\lambda$:

$$B_\lambda(\lambda,T)\,d\lambda = B_\nu(\nu,T)\,d\nu$$

This yields the conversion:

$$B_\lambda(\lambda,T) = B_\nu\!\left(\frac{c}{\lambda},T\right) \cdot \frac{c}{\lambda^2}$$

This means:

$B_\nu$ and $B_\lambda$ are **not** simply the same number with a different unit—they differ by the Jacobian factor $c/\lambda^2$.

---

## Spectral Peaks (Wien's Displacement Law)

Important: The maximum of $B_\nu$ and the maximum of $B_\lambda$ do **not** lie at the same physical position because the spectral densities are weighted differently.

### Peak of $B_\nu$ (Frequency Form)

The maximum of $B_\nu(\nu,T)$ occurs at the frequency $\nu_{\max,\nu}$ satisfying the equation:

$$3\left(1 - e^{-x}\right) = x,\quad x = \frac{h\nu_{\max,\nu}}{k_B T}$$

The solution is a universal constant $x \approx 2.821439\ldots$, giving:

$$\nu_{\max,\nu} = \frac{k_B T}{h} \cdot x \approx 5.8789232\times10^{10}\ \mathrm{Hz/K} \cdot T$$

### Peak of $B_\lambda$ (Wavelength Form)

The maximum of $B_\lambda(\lambda,T)$ occurs at the wavelength $\lambda_{\max,\lambda}$ satisfying the equation:

$$5\left(1 - e^{-y}\right) = y,\quad y = \frac{hc}{\lambda_{\max,\lambda} k_B T}$$

The solution is a universal constant $y \approx 4.965114\ldots$, giving:

$$\lambda_{\max,\lambda} = \frac{hc}{k_B T} \cdot \frac{1}{y} = \frac{b}{T}$$

using Wien's displacement constant:

$$b \approx 2.897771955\times10^{-3}\ \mathrm{m\,K}$$

### Important Note

It is **not** true that:

$$\lambda_{\max,\lambda} = \frac{c}{\nu_{\max,\nu}}$$

The two peaks represent different points in the spectrum due to the Jacobian factor when converting between frequency and wavelength.

---

## Total Power (Stefan-Boltzmann Law)

### Integrated Radiance Over All Frequencies

Integrating over all frequencies yields the radiance per solid angle:

$$\int_0^\infty B_\nu(\nu,T)\,d\nu = \frac{\sigma}{\pi} T^4$$

* Unit: $\mathrm{W\,m^{-2}\,sr^{-1}}$

### Hemispherical Exitance

The total power radiated per unit area across the entire hemisphere is:

$$M(T) = \sigma T^4$$

* Unit: $\mathrm{W\,m^{-2}}$
* $\sigma$: Stefan-Boltzmann constant ($5.670374419\times10^{-8}\ \mathrm{W\,m^{-2}\,K^{-4}}$)

---

## SI Units and Conventions

Internally, this module uses strictly SI units:

* Frequency $\nu$: $\mathrm{Hz}$
* Wavelength $\lambda$: $\mathrm{m}$
* Temperature $T$: $\mathrm{K}$
* Spectral radiance:
* $B_\nu$: $\mathrm{W\,m^{-2}\,sr^{-1}\,Hz^{-1}}$
* $B_\lambda$: $\mathrm{W\,m^{-2}\,sr^{-1}\,m^{-1}}$



Nanometers ($\mathrm{nm}$) are accepted as input/output helpers only and are immediately converted to meters internally.

---

## Implementation in the Rust Module

### Public Functions

* `planck_spectral_radiance_nu(frequency_hz, temperature_k)`
Computes $B_\nu(\nu,T)$.
* `planck_spectral_radiance_lambda(wavelength_m, temperature_k)`
Computes $B_\lambda(\lambda,T)$.
* `planck_radiance_integral_gk15(frequency_start_hz, frequency_end_hz, temperature_k)`
Numerical integral $\int_{\nu_1}^{\nu_2} B_\nu(\nu,T)\,d\nu$ using Gauss-Kronrod-15 quadrature.
* `total_radiance_per_steradian(temperature_k)`
$\int_0^\infty B_\nu(\nu,T)\,d\nu = (\sigma/\pi) T^4$.
* `stefan_boltzmann_exitance(temperature_k)`
$M(T) = \sigma T^4$.
* `wien_emission_maximum_frequency(temperature_k)`
$\nu_{\max,\nu}$ for $B_\nu$.
* `wien_emission_maximum_wavelength(temperature_k)`
$\lambda_{\max,\lambda}$ for $B_\lambda$.
* `create_planck_engineering_report(path, frequency_range_hz, temperature_k)`
Generates an engineering report containing all relevant quantities.

---

## Typical Usage

```rust
use optics::planck;

let t = 3500.0; // K
let nu_start = 5.0e13; // Hz
let nu_end = 2.0e14;   // Hz

let (integral, err) = planck::planck_radiance_integral_gk15(nu_start, nu_end, t);
let peak_nu = planck::wien_emission_maximum_frequency(t);
let peak_lambda = planck::wien_emission_maximum_wavelength(t);

```

---

## Comparison with Python Reference

For validation, it is recommended to compare Rust outputs against a high-precision Python reference implementation (e.g., using `mpmath`). Typical test cases include:

* Fixed pairs $(\nu, T)$ comparing $B_\nu$, $B_\lambda$, integrals, and peaks.
* Checking the Jacobian relation:

$$\frac{B_\lambda(\lambda,T)}{B_\nu(\nu,T) \cdot c/\lambda^2} \approx 1,\quad \lambda = c/\nu$$

The comparison results are all perfect within type definitions (mpmath, f64).

---
### Tags / Keywords: 
#### rust, optics, physics, blackbody radiation, Planck's law
---

## References and Sources

* M. Planck: "Zur Theorie des Gesetzes der Energieverteilung im Normalspektrum", 1900.
* W. Wien: "Über die Energieverteilung im Emissionsspektrum eines schwarzen Körpers", 1896.
* J. D. Jackson: *Classical Electrodynamics*, chapter on blackbody radiation.
* Wikipedia: [Planck's law](https://en.wikipedia.org/wiki/Planck's_law), [Wien's displacement law](https://en.wikipedia.org/wiki/Wien's_displacement_law).