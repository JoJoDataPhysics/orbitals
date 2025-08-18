use std::f64::consts::PI;

/// Errors that can occur during orbital calculations
#[derive(Debug, PartialEq)]
pub enum OrbitalError {
    InvalidRadius(f64),
    InvalidAngle(f64),
    NumericalError(String),
}

impl std::fmt::Display for OrbitalError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            OrbitalError::InvalidRadius(r) => write!(f, "Invalid radius: {}", r),
            OrbitalError::InvalidAngle(angle) => write!(f, "Invalid angle: {}", angle),
            OrbitalError::NumericalError(msg) => write!(f, "Numerical error: {}", msg),
        }
    }
}

impl std::error::Error for OrbitalError {}

// Constants
// Bohr radius set to 1 for normalization
pub const BOHR_RADIUS: f64 = 1.0; //5.29177210903e-11 m actual Bohr radius in meters

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

/// Radial part of the 2p orbital (n=2, l=1)
fn radial_part_2p(r: f64) -> f64 {
    let a0 = BOHR_RADIUS;
    let rho = r / a0;
    
    // Radial function R_{2,1}(r)
    let normalization_factor = (1.0 / (2.0 * a0)).powf(3.0) * (1.0 / 6.0_f64).sqrt();
    let radial_polynomial = rho * (-rho / 2.0).exp();
    
    normalization_factor * radial_polynomial
}

/// Angular part of the p_x orbital (Y_{1,1} + Y_{1,-1})
fn angular_part_px(theta: f64, phi: f64) -> f64 {
    let normalization_factor = (3.0 / (4.0 * PI)).sqrt();
    
    normalization_factor * theta.sin() * phi.cos()
}

/// Angular part of the p_y orbital (Y_{1,1} - Y_{1,-1})
fn angular_part_py(theta: f64, phi: f64) -> f64 {
    let normalization_factor = (3.0 / (4.0 * PI)).sqrt();
    
    normalization_factor * theta.sin() * phi.sin()
}

/// Angular part of the p_z orbital (Y_{1,0})
fn angular_part_pz(theta: f64, _phi: f64) -> f64 {
    let normalization_factor = (3.0 / (4.0 * PI)).sqrt();
    
    normalization_factor * theta.cos()
}

/// Probability density for the 2p_x orbital
pub fn probability_density_2p_x(r: f64, theta: f64, phi: f64) -> f64 {
    let radial = radial_part_2p(r);
    let angular = angular_part_px(theta, phi);
    
    (radial * angular).powi(2)
}

/// Probability density for the 2p_y orbital
pub fn probability_density_2p_y(r: f64, theta: f64, phi: f64) -> f64 {
    let radial = radial_part_2p(r);
    let angular = angular_part_py(theta, phi);
    
    (radial * angular).powi(2)
}

/// Probability density for the 2p_z orbital
pub fn probability_density_2p_z(r: f64, theta: f64, phi: f64) -> f64 {
    let radial = radial_part_2p(r);
    let angular = angular_part_pz(theta, phi);
    
    (radial * angular).powi(2)
}
// Function to calculate the probability density of the 1s orbital
pub fn probability_density_1s(r: f64, _tau: f64, _phi: f64) -> f64 {
    let a0 = BOHR_RADIUS;
    let normalization_factor = 1.0 / (PI * a0.powi(3));
    let radial_part = (-2.0 * r / a0).exp();
    normalization_factor * radial_part
}

/// Safe version of probability_density_1s with error checking
pub fn probability_density_1s_safe(r: f64, theta: f64, phi: f64) -> Result<f64, OrbitalError> {
    if r < 0.0 {
        return Err(OrbitalError::InvalidRadius(r));
    }
    
    if !r.is_finite() || !theta.is_finite() || !phi.is_finite() {
        return Err(OrbitalError::NumericalError("Non-finite input values".to_string()));
    }
    
    if theta < 0.0 || theta > PI {
        return Err(OrbitalError::InvalidAngle(theta));
    }

    let a0 = BOHR_RADIUS;
    let normalization_factor = 1.0 / (PI * a0.powi(3));
    let exponent = -2.0 * r / a0;
    
    // Check for numerical overflow in exponential
    if exponent < -700.0 {
        return Ok(0.0); // Underflow to zero is safe
    }
    
    let radial_part = exponent.exp();
    let result = normalization_factor * radial_part;
    
    if !result.is_finite() {
        return Err(OrbitalError::NumericalError("Probability calculation overflow".to_string()));
    }
    
    Ok(result)
}

/// Safe version of probability_density_3d_z2 with error checking
pub fn probability_density_3d_z2_safe(r: f64, theta: f64, phi: f64) -> Result<f64, OrbitalError> {
    if r < 0.0 {
        return Err(OrbitalError::InvalidRadius(r));
    }
    
    if !r.is_finite() || !theta.is_finite() || !phi.is_finite() {
        return Err(OrbitalError::NumericalError("Non-finite input values".to_string()));
    }
    
    if theta < 0.0 || theta > PI {
        return Err(OrbitalError::InvalidAngle(theta));
    }

    let radial = radial_part_3d(r);
    let angular = angular_part_2_0(theta, phi);
    
    if !radial.is_finite() || !angular.is_finite() {
        return Err(OrbitalError::NumericalError("Intermediate calculation overflow".to_string()));
    }

    let result = (radial * angular).powi(2);
    
    if !result.is_finite() {
        return Err(OrbitalError::NumericalError("Probability calculation overflow".to_string()));
    }
    
    Ok(result)
}

/// Safe version of probability_density_2p_x with error checking
pub fn probability_density_2p_x_safe(r: f64, theta: f64, phi: f64) -> Result<f64, OrbitalError> {
    validate_orbital_inputs(r, theta, phi)?;
    
    let radial = radial_part_2p(r);
    let angular = angular_part_px(theta, phi);
    
    if !radial.is_finite() || !angular.is_finite() {
        return Err(OrbitalError::NumericalError("Intermediate calculation overflow".to_string()));
    }

    let result = (radial * angular).powi(2);
    
    if !result.is_finite() {
        return Err(OrbitalError::NumericalError("Probability calculation overflow".to_string()));
    }
    
    Ok(result)
}

/// Safe version of probability_density_2p_y with error checking
pub fn probability_density_2p_y_safe(r: f64, theta: f64, phi: f64) -> Result<f64, OrbitalError> {
    validate_orbital_inputs(r, theta, phi)?;
    
    let radial = radial_part_2p(r);
    let angular = angular_part_py(theta, phi);
    
    if !radial.is_finite() || !angular.is_finite() {
        return Err(OrbitalError::NumericalError("Intermediate calculation overflow".to_string()));
    }

    let result = (radial * angular).powi(2);
    
    if !result.is_finite() {
        return Err(OrbitalError::NumericalError("Probability calculation overflow".to_string()));
    }
    
    Ok(result)
}

/// Safe version of probability_density_2p_z with error checking
pub fn probability_density_2p_z_safe(r: f64, theta: f64, phi: f64) -> Result<f64, OrbitalError> {
    validate_orbital_inputs(r, theta, phi)?;
    
    let radial = radial_part_2p(r);
    let angular = angular_part_pz(theta, phi);
    
    if !radial.is_finite() || !angular.is_finite() {
        return Err(OrbitalError::NumericalError("Intermediate calculation overflow".to_string()));
    }

    let result = (radial * angular).powi(2);
    
    if !result.is_finite() {
        return Err(OrbitalError::NumericalError("Probability calculation overflow".to_string()));
    }
    
    Ok(result)
}

/// Helper function to validate orbital inputs
fn validate_orbital_inputs(r: f64, theta: f64, phi: f64) -> Result<(), OrbitalError> {
    if r < 0.0 {
        return Err(OrbitalError::InvalidRadius(r));
    }
    
    if !r.is_finite() || !theta.is_finite() || !phi.is_finite() {
        return Err(OrbitalError::NumericalError("Non-finite input values".to_string()));
    }
    
    if theta < 0.0 || theta > PI {
        return Err(OrbitalError::InvalidAngle(theta));
    }
    
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const EPSILON: f64 = 1e-10;

    fn approx_eq(a: f64, b: f64) -> bool {
        (a - b).abs() < EPSILON
    }

    #[test]
    fn test_probability_density_1s_at_origin() {
        // At r=0, the 1s probability density should be at maximum
        let density_at_origin = probability_density_1s(0.0, 0.0, 0.0);
        let expected = 1.0 / (PI * BOHR_RADIUS.powi(3));
        assert!(approx_eq(density_at_origin, expected));
    }

    #[test]
    fn test_probability_density_1s_decreases_with_distance() {
        // 1s probability should decay exponentially with distance
        let r1 = 0.0;
        let r2 = BOHR_RADIUS;
        let r3 = 2.0 * BOHR_RADIUS;
        
        let density1 = probability_density_1s(r1, 0.0, 0.0);
        let density2 = probability_density_1s(r2, 0.0, 0.0);
        let density3 = probability_density_1s(r3, 0.0, 0.0);
        
        assert!(density1 > density2);
        assert!(density2 > density3);
        assert!(density3 > 0.0);
    }

    #[test]
    fn test_probability_density_1s_spherical_symmetry() {
        // 1s orbital should be spherically symmetric (independent of theta, phi)
        let r = BOHR_RADIUS;
        let density1 = probability_density_1s(r, 0.0, 0.0);
        let density2 = probability_density_1s(r, PI/2.0, 0.0);
        let density3 = probability_density_1s(r, PI, PI/2.0);
        let density4 = probability_density_1s(r, PI/4.0, 3.0*PI/2.0);
        
        assert!(approx_eq(density1, density2));
        assert!(approx_eq(density2, density3));
        assert!(approx_eq(density3, density4));
    }

    #[test]
    fn test_probability_density_3d_z2_is_positive() {
        // Probability density should always be non-negative
        let test_cases = vec![
            (0.0, 0.0, 0.0),
            (BOHR_RADIUS, 0.0, 0.0),
            (BOHR_RADIUS, PI/2.0, 0.0),
            (BOHR_RADIUS, PI, 0.0),
            (2.0 * BOHR_RADIUS, PI/4.0, PI/2.0),
        ];

        for (r, theta, phi) in test_cases {
            let density = probability_density_3d_z2(r, theta, phi);
            assert!(density >= 0.0, "Density should be non-negative at ({}, {}, {}): {}", r, theta, phi, density);
        }
    }

    #[test]
    fn test_probability_density_3d_z2_symmetry() {
        // d_z^2 orbital should have cylindrical symmetry around z-axis
        let r = 2.0 * BOHR_RADIUS;
        let theta = PI/3.0;
        
        let density1 = probability_density_3d_z2(r, theta, 0.0);
        let density2 = probability_density_3d_z2(r, theta, PI/2.0);
        let density3 = probability_density_3d_z2(r, theta, PI);
        let density4 = probability_density_3d_z2(r, theta, 3.0*PI/2.0);
        
        assert!(approx_eq(density1, density2));
        assert!(approx_eq(density2, density3));
        assert!(approx_eq(density3, density4));
    }

    #[test]
    fn test_probability_density_3d_z2_nodal_plane() {
        // d_z^2 orbital has a nodal surface where density should be low
        let r = 2.0 * BOHR_RADIUS;
        
        // Along z-axis (theta = 0, PI) should have higher density
        let density_z_axis = probability_density_3d_z2(r, 0.0, 0.0);
        
        // At the "magic angle" where 3cos²θ - 1 = 0, density should be zero
        let magic_angle = (1.0/3.0_f64).sqrt().acos(); // cos²θ = 1/3
        let density_nodal = probability_density_3d_z2(r, magic_angle, 0.0);
        
        assert!(density_z_axis > density_nodal);
        assert!(density_nodal < EPSILON); // Should be very close to zero
    }

    #[test]
    fn test_radial_part_3d_at_origin() {
        // Radial part at r=0 should be 0 for 3d orbital (due to r factor)
        let radial_at_origin = radial_part_3d(0.0);
        assert_eq!(radial_at_origin, 0.0);
    }

    #[test]
    fn test_angular_part_2_0_extremes() {
        // Y_{2,0} should have maximum along z-axis and minimum in xy-plane
        let along_z = angular_part_2_0(0.0, 0.0); // theta = 0
        let in_xy_plane = angular_part_2_0(PI/2.0, 0.0); // theta = π/2
        
        // Along z-axis: 3cos²(0) - 1 = 3(1) - 1 = 2
        // In xy-plane: 3cos²(π/2) - 1 = 3(0) - 1 = -1
        assert!(along_z > in_xy_plane);
        
        let expected_z = (5.0 / (16.0 * PI)).sqrt() * 2.0;
        let expected_xy = (5.0 / (16.0 * PI)).sqrt() * (-1.0);
        
        assert!(approx_eq(along_z, expected_z));
        assert!(approx_eq(in_xy_plane, expected_xy));
    }

    #[test]
    fn test_probability_density_1s_safe_error_handling() {
        // Test negative radius
        assert!(matches!(
            probability_density_1s_safe(-1.0, 0.0, 0.0),
            Err(OrbitalError::InvalidRadius(_))
        ));
        
        // Test invalid theta
        assert!(matches!(
            probability_density_1s_safe(1.0, -0.1, 0.0),
            Err(OrbitalError::InvalidAngle(_))
        ));
        
        assert!(matches!(
            probability_density_1s_safe(1.0, PI + 0.1, 0.0),
            Err(OrbitalError::InvalidAngle(_))
        ));
        
        // Test non-finite values
        assert!(matches!(
            probability_density_1s_safe(f64::NAN, 0.0, 0.0),
            Err(OrbitalError::NumericalError(_))
        ));
        
        assert!(matches!(
            probability_density_1s_safe(f64::INFINITY, 0.0, 0.0),
            Err(OrbitalError::NumericalError(_))
        ));
    }

    #[test]
    fn test_probability_density_3d_z2_safe_error_handling() {
        // Test negative radius
        assert!(matches!(
            probability_density_3d_z2_safe(-1.0, 0.0, 0.0),
            Err(OrbitalError::InvalidRadius(_))
        ));
        
        // Test invalid theta
        assert!(matches!(
            probability_density_3d_z2_safe(1.0, -0.1, 0.0),
            Err(OrbitalError::InvalidAngle(_))
        ));
        
        // Test non-finite values
        assert!(matches!(
            probability_density_3d_z2_safe(f64::NAN, 0.0, 0.0),
            Err(OrbitalError::NumericalError(_))
        ));
    }

    #[test]
    fn test_safe_functions_return_ok_for_valid_inputs() {
        // Test that safe functions work correctly for valid inputs
        let result_1s = probability_density_1s_safe(BOHR_RADIUS, PI/2.0, 0.0).unwrap();
        let expected_1s = probability_density_1s(BOHR_RADIUS, PI/2.0, 0.0);
        assert!(approx_eq(result_1s, expected_1s));
        
        let result_3d = probability_density_3d_z2_safe(2.0 * BOHR_RADIUS, PI/3.0, PI/4.0).unwrap();
        let expected_3d = probability_density_3d_z2(2.0 * BOHR_RADIUS, PI/3.0, PI/4.0);
        assert!(approx_eq(result_3d, expected_3d));
    }

    #[test]
    fn test_2p_orbitals_are_positive() {
        // All 2p probability densities should be non-negative
        let test_cases = vec![
            (BOHR_RADIUS, 0.0, 0.0),
            (BOHR_RADIUS, PI/2.0, 0.0),
            (BOHR_RADIUS, PI/2.0, PI/2.0),
            (2.0 * BOHR_RADIUS, PI/4.0, PI/4.0),
        ];

        for (r, theta, phi) in test_cases {
            let density_px = probability_density_2p_x(r, theta, phi);
            let density_py = probability_density_2p_y(r, theta, phi);
            let density_pz = probability_density_2p_z(r, theta, phi);
            
            assert!(density_px >= 0.0, "p_x density should be non-negative at ({}, {}, {}): {}", r, theta, phi, density_px);
            assert!(density_py >= 0.0, "p_y density should be non-negative at ({}, {}, {}): {}", r, theta, phi, density_py);
            assert!(density_pz >= 0.0, "p_z density should be non-negative at ({}, {}, {}): {}", r, theta, phi, density_pz);
        }
    }

    #[test]
    fn test_2p_orbital_directional_properties() {
        let r = 2.0 * BOHR_RADIUS;
        
        // p_x should be maximum along x-axis (theta=π/2, phi=0)
        let px_along_x = probability_density_2p_x(r, PI/2.0, 0.0);
        let px_along_y = probability_density_2p_x(r, PI/2.0, PI/2.0);
        let px_along_z = probability_density_2p_x(r, 0.0, 0.0);
        
        assert!(px_along_x > px_along_z);
        assert!(px_along_x > px_along_y); // Should be close to zero
        
        // p_y should be maximum along y-axis (theta=π/2, phi=π/2)
        let py_along_y = probability_density_2p_y(r, PI/2.0, PI/2.0);
        let py_along_x = probability_density_2p_y(r, PI/2.0, 0.0);
        let py_along_z = probability_density_2p_y(r, 0.0, 0.0);
        
        assert!(py_along_y > py_along_z);
        assert!(py_along_y > py_along_x); // Should be close to zero
        
        // p_z should be maximum along z-axis (theta=0)
        let pz_along_z = probability_density_2p_z(r, 0.0, 0.0);
        let pz_in_xy = probability_density_2p_z(r, PI/2.0, 0.0);
        
        assert!(pz_along_z > pz_in_xy); // Should be close to zero
    }

    #[test]
    fn test_2p_orbitals_zero_at_origin() {
        // All 2p orbitals should have zero probability at r=0 due to the r factor in radial part
        assert_eq!(probability_density_2p_x(0.0, 0.0, 0.0), 0.0);
        assert_eq!(probability_density_2p_y(0.0, 0.0, 0.0), 0.0);
        assert_eq!(probability_density_2p_z(0.0, 0.0, 0.0), 0.0);
    }

    #[test]
    fn test_2p_safe_functions_match_unsafe() {
        let test_cases = vec![
            (BOHR_RADIUS, PI/4.0, PI/6.0),
            (2.0 * BOHR_RADIUS, PI/3.0, PI/2.0),
            (0.5 * BOHR_RADIUS, PI/6.0, 3.0*PI/4.0),
        ];

        for (r, theta, phi) in test_cases {
            // Test p_x
            let unsafe_px = probability_density_2p_x(r, theta, phi);
            let safe_px = probability_density_2p_x_safe(r, theta, phi).unwrap();
            assert!(approx_eq(unsafe_px, safe_px), "p_x mismatch at ({}, {}, {})", r, theta, phi);
            
            // Test p_y
            let unsafe_py = probability_density_2p_y(r, theta, phi);
            let safe_py = probability_density_2p_y_safe(r, theta, phi).unwrap();
            assert!(approx_eq(unsafe_py, safe_py), "p_y mismatch at ({}, {}, {})", r, theta, phi);
            
            // Test p_z
            let unsafe_pz = probability_density_2p_z(r, theta, phi);
            let safe_pz = probability_density_2p_z_safe(r, theta, phi).unwrap();
            assert!(approx_eq(unsafe_pz, safe_pz), "p_z mismatch at ({}, {}, {})", r, theta, phi);
        }
    }
}
