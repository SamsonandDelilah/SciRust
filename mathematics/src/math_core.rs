use crate::math_constants::MathConstants;

pub fn are_nearly_equal(
    a: f64, 
    b: f64, 
    absolute_tol: Option<f64>, 
    relative_tol: Option<f64>
) -> bool {
    // Standardwerte definieren, falls None übergeben wird
    let abs_tol = absolute_tol.unwrap_or(MathConstants::MIN_ABSTOL_F64);
    let rel_tol = relative_tol.unwrap_or(MathConstants::MIN_RELTOL_F64);

    if a.is_nan() || b.is_nan() {
        return false;
    }
    if a == b {
        return true;
    }

    let diff = (a - b).abs();
    if diff <= abs_tol {
        return true;
    }

    let scale = a.abs().max(b.abs());
    if scale == 0.0 || scale.is_infinite() {
        return false;
    }

    (diff / scale) <= rel_tol
}

