# Planck Radiation Module (Rust)

Dieses Modul implementiert das Plancksche Strahlungsgesetz für einen idealen schwarzen Körper in SI-Einheiten. Es bietet sowohl die Frequenzform \(B_\nu(\nu,T)\) als auch die Wellenlängenform \(B_\lambda(\lambda,T)\), inklusive analytischer Gesamtleistung, numerischer Bandintegrale und der beiden Peak-Definitionen (Wien'sches Verschiebungsgesetz).

---

## Physikalische Grundlagen

### Plancksches Strahlungsgesetz (Frequenzform)

Die spektrale Strahldichte pro Frequenzintervall ist:

\[
B_\nu(\nu,T) = \frac{2h\nu^3}{c^2} \cdot \frac{1}{e^{h\nu/(k_B T)} - 1}
\]

- \(B_\nu\): spektrale Strahldichte, Einheit \(\mathrm{W\,m^{-2}\,sr^{-1}\,Hz^{-1}}\)
- \(\nu\): Frequenz in \(\mathrm{Hz}\)
- \(T\): Temperatur in \(\mathrm{K}\)
- \(h\): Plancksches Wirkungsquantum (\(6.62607015\times10^{-34}\ \mathrm{J\,s}\))
- \(c\): Lichtgeschwindigkeit im Vakuum (\(299792458\ \mathrm{m/s}\))
- \(k_B\): Boltzmann-Konstante (\(1.380649\times10^{-23}\ \mathrm{J/K}\))

### Plancksches Strahlungsgesetz (Wellenlängenform)

Die spektrale Strahldichte pro Wellenlängenintervall ist:

\[
B_\lambda(\lambda,T) = \frac{2hc^2}{\lambda^5} \cdot \frac{1}{e^{hc/(\lambda k_B T)} - 1}
\]

- \(B_\lambda\): spektrale Strahldichte, Einheit \(\mathrm{W\,m^{-2}\,sr^{-1}\,m^{-1}}\)
- \(\lambda\): Wellenlänge in \(\mathrm{m}\)

### Zusammenhang zwischen \(B_\nu\) und \(B_\lambda\) (Jacobian)

Beide Formen beschreiben dieselbe physikalische Strahlung, nur bezogen auf unterschiedliche Spektralvariablen. Für \(\nu = c/\lambda\) gilt:

\[
B_\lambda(\lambda,T)\,d\lambda = B_\nu(\nu,T)\,d\nu
\]

Daraus folgt die Umrechnung:

\[
B_\lambda(\lambda,T) = B_\nu\!\left(\frac{c}{\lambda},T\right) \cdot \frac{c}{\lambda^2}
\]

Das bedeutet:  
\(B_\nu\) und \(B_\lambda\) sind **nicht** einfach dieselbe Zahl mit anderer Einheit – sie unterscheiden sich durch den Jacobian-Faktor \(c/\lambda^2\).

---

## Peaks der Spektralverteilung (Wien'sches Verschiebungsgesetz)

Wichtig: Das Maximum von \(B_\nu\) und das Maximum von \(B_\lambda\) liegen **nicht** bei derselben physikalischen Stelle, weil die Spektraldichten unterschiedlich gewichtet sind.

### Peak von \(B_\nu\) (Frequenzform)

Das Maximum von \(B_\nu(\nu,T)\) liegt bei der Frequenz \(\nu_{\max,\nu}\), die die Gleichung erfüllt:

\[
3\left(1 - e^{-x}\right) = x,\quad x = \frac{h\nu_{\max,\nu}}{k_B T}
\]

Die Lösung ist eine universelle Konstante \(x \approx 2.821439\ldots\), sodass:

\[
\nu_{\max,\nu} = \frac{k_B T}{h} \cdot x \approx 5.8789232\times10^{10}\ \mathrm{Hz/K} \cdot T
\]

### Peak von \(B_\lambda\) (Wellenlängenform)

Das Maximum von \(B_\lambda(\lambda,T)\) liegt bei der Wellenlänge \(\lambda_{\max,\lambda}\), die die Gleichung erfüllt:

\[
5\left(1 - e^{-y}\right) = y,\quad y = \frac{hc}{\lambda_{\max,\lambda} k_B T}
\]

Die Lösung ist eine universelle Konstante \(y \approx 4.965114\ldots\), sodass:

\[
\lambda_{\max,\lambda} = \frac{hc}{k_B T} \cdot \frac{1}{y} = \frac{b}{T}
\]

mit dem Wien'schen Verschiebungsgesetz:

\[
b \approx 2.897771955\times10^{-3}\ \mathrm{m\,K}
\]

### Wichtiger Hinweis

Es gilt **nicht**:

\[
\lambda_{\max,\lambda} = \frac{c}{\nu_{\max,\nu}}
\]

Die beiden Peaks sind unterschiedliche Punkte im Spektrum. Das liegt am Jacobian bei der Umrechnung zwischen Frequenz und Wellenlänge.

---

## Gesamtleistung (Stefan-Boltzmann-Gesetz)

### Integrierte Strahldichte über alle Frequenzen

Das Integral über alle Frequenzen ergibt die Strahldichte pro Raumwinkel:

\[
\int_0^\infty B_\nu(\nu,T)\,d\nu = \frac{\sigma}{\pi} T^4
\]

- Einheit: \(\mathrm{W\,m^{-2}\,sr^{-1}}\)

### Hemisphärische Ausstrahlung (Exitance)

Die über den gesamten Halbraum abgestrahlte Leistung pro Fläche ist:

\[
M(T) = \sigma T^4
\]

- Einheit: \(\mathrm{W\,m^{-2}}\)
- \(\sigma\): Stefan-Boltzmann-Konstante (\(5.670374419\times10^{-8}\ \mathrm{W\,m^{-2}\,K^{-4}}\))

---

## SI-Einheiten und Konventionen

Intern verwendet dieses Modul ausschließlich SI-Einheiten:

- Frequenz \(\nu\): \(\mathrm{Hz}\)
- Wellenlänge \(\lambda\): \(\mathrm{m}\)
- Temperatur \(T\): \(\mathrm{K}\)
- Spektrale Strahldichte:
  - \(B_\nu\): \(\mathrm{W\,m^{-2}\,sr^{-1}\,Hz^{-1}}\)
  - \(B_\lambda\): \(\mathrm{W\,m^{-2}\,sr^{-1}\,m^{-1}}\)

Nanometer (\(\mathrm{nm}\)) werden nur als Ein-/Ausgabe-Hilfe verwendet und intern sofort in Meter umgerechnet.

---

## Implementierung im Rust-Modul

### Öffentliche Funktionen

- `planck_spectral_radiance_nu(frequency_hz, temperature_k)`  
  Berechnet \(B_\nu(\nu,T)\).

- `planck_spectral_radiance_lambda(wavelength_m, temperature_k)`  
  Berechnet \(B_\lambda(\lambda,T)\).

- `planck_radiance_integral_gk15(frequency_start_hz, frequency_end_hz, temperature_k)`  
  Numerisches Integral \(\int_{\nu_1}^{\nu_2} B_\nu(\nu,T)\,d\nu\) mit Gauss-Kronrod-15-Quadratur.

- `total_radiance_per_steradian(temperature_k)`  
  \(\int_0^\infty B_\nu(\nu,T)\,d\nu = (\sigma/\pi) T^4\).

- `stefan_boltzmann_exitance(temperature_k)`  
  \(M(T) = \sigma T^4\).

- `wien_emission_maximum_frequency(temperature_k)`  
  \(\nu_{\max,\nu}\) für \(B_\nu\).

- `wien_emission_maximum_wavelength(temperature_k)`  
  \(\lambda_{\max,\lambda}\) für \(B_\lambda\).

- `create_planck_engineering_report(path, frequency_range_hz, temperature_k)`  
  Erzeugt einen Ingenieursreport mit allen relevanten Größen.

---

## Typische Verwendung

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

## Vergleich mit Python-Referenz

Zur Validierung wird empfohlen, die Rust-Ergebnisse mit einer Python-Referenzimplementierung (z. B. mit `mpmath` in hoher Präzision) zu vergleichen. Typische Testfälle:

- Feste Paare \((\nu, T)\) und Vergleich von \(B_\nu\), \(B_\lambda\), Integralen und Peaks.
- Prüfung des Jacobian-Verhältnisses:
  \[
  \frac{B_\lambda(\lambda,T)}{B_\nu(\nu,T) \cdot c/\lambda^2} \approx 1,\quad \lambda = c/\nu
  \]

---

## Literatur und Quellen

- M. Planck: „Zur Theorie des Gesetzes der Energieverteilung im Normalspektrum“, 1900.
- W. Wien: „Über die Energieverteilung im Emissionsspektrum eines schwarzen Körpers“, 1896.
- J. D. Jackson: *Classical Electrodynamics*, Kapitel zu Schwarzkörperstrahlung.
- Wikipedia: [Planck's law](https://en.wikipedia.org/wiki/Planck's_law), [Wien's displacement law](https://en.wikipedia.org/wiki/Wien's_displacement_law).