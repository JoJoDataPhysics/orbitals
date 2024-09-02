use std::f64::consts::PI;

// Constants
pub const BOHR_RADIUS: f64 = 1.0; //5.29177210903e-11; // Bohr radius in meters

// Function to calculate the probability density of the 1s orbital
pub fn probability_density_1s(r: f64) -> f64 {
    let a0 = BOHR_RADIUS;
    let normalization_factor = 1.0 / (PI * a0.powi(3));
    let radial_part = (-2.0 * r / a0).exp();
    normalization_factor * radial_part
}
