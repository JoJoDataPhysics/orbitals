use std::f64::consts::PI;

// Constants
pub const BOHR_RADIUS: f64 = 1.0; //5.29177210903e-11; // Bohr radius in meters

/// Radial part of the 3d orbital (n=3, l=2)
fn radial_part_3d(r: f64) -> f64 {
    let a0 = BOHR_RADIUS;
    let rho = 2.0 * r / (3.0 * a0);

    // Radial function R_{3,2}(r)
    let normalization_factor = (2.0 / (3.0 * a0)).powf(3.0) * (2.0_f64 / 27.0_f64).sqrt() / 81.0;
    let radial_polynomial = rho * (15.0 - rho.powi(2)) * (-rho / 2.0).exp();

    normalization_factor * radial_polynomial
}

/// Angular part of the d_z^2 orbital (Y_{2,0})
fn angular_part_2_0(theta: f64, _phi: f64) -> f64 {
    let cos_theta = theta.cos();
    let normalization_factor = (5.0 / (16.0 * PI)).sqrt();

    normalization_factor * (3.0 * cos_theta.powi(2) - 1.0)
}

/// Probability density for the 3d_z^2 orbital
pub fn probability_density_3d_z2(r: f64, theta: f64, phi: f64) -> f64 {
    let radial = radial_part_3d(r);
    let angular = angular_part_2_0(theta, phi);

    (radial * angular).powi(2)
}
// Function to calculate the probability density of the 1s orbital
pub fn probability_density_1s(r: f64, _tau: f64, _phi: f64) -> f64 {
    let a0 = BOHR_RADIUS;
    let normalization_factor = 1.0 / (PI * a0.powi(3));
    let radial_part = (-2.0 * r / a0).exp();
    normalization_factor * radial_part
}
