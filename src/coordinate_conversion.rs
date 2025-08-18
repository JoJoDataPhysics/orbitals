use std::f64::consts::PI;

/// Errors that can occur during coordinate conversion
#[derive(Debug, PartialEq)]
pub enum CoordinateError {
    InvalidRadius(f64),
    InvalidAngle(f64),
    NumericalError(String),
}

impl std::fmt::Display for CoordinateError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CoordinateError::InvalidRadius(r) => write!(f, "Invalid radius: {}", r),
            CoordinateError::InvalidAngle(angle) => write!(f, "Invalid angle: {}", angle),
            CoordinateError::NumericalError(msg) => write!(f, "Numerical error: {}", msg),
        }
    }
}

impl std::error::Error for CoordinateError {}

/// Represents Cartesian coordinates (x, y, z)
#[derive(Debug, Clone, Copy)]
pub struct Cartesian {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

/// Represents Spherical coordinates (r, theta, phi)
#[derive(Debug, Clone, Copy)]
pub struct Spherical {
    pub r: f64,     // Radius
    pub theta: f64, // Polar angle in radians [0, PI]
    pub phi: f64,   // Azimuthal angle in radians [0, 2*PI]
}

/// Converts Cartesian coordinates to Spherical coordinates
///
/// # Arguments
///
/// * `cart` - A reference to a Cartesian coordinate
///
/// # Returns
///
/// * `Spherical` - The equivalent spherical coordinates
pub fn cartesian_to_spherical(cart: Cartesian) -> Spherical {
    let x = cart.x;
    let y = cart.y;
    let z = cart.z;

    let r = (x.powi(2) + y.powi(2) + z.powi(2)).sqrt();
    let theta = if r == 0.0 { 0.0 } else { (z / r).acos() };
    let phi = y.atan2(x);
    let phi = if phi < 0.0 { phi + 2.0 * PI } else { phi };

    Spherical { r, theta, phi }
}

/// Safe version of cartesian_to_spherical with error checking
///
/// # Arguments
///
/// * `cart` - A Cartesian coordinate
///
/// # Returns
///
/// * `Result<Spherical, CoordinateError>` - The equivalent spherical coordinates or an error
pub fn cartesian_to_spherical_safe(cart: Cartesian) -> Result<Spherical, CoordinateError> {
    let x = cart.x;
    let y = cart.y;
    let z = cart.z;

    // Check for invalid input values
    if !x.is_finite() || !y.is_finite() || !z.is_finite() {
        return Err(CoordinateError::NumericalError("Non-finite coordinate values".to_string()));
    }

    let r = (x.powi(2) + y.powi(2) + z.powi(2)).sqrt();
    
    if !r.is_finite() {
        return Err(CoordinateError::NumericalError("Radius calculation overflow".to_string()));
    }

    let theta = if r == 0.0 { 
        0.0 
    } else { 
        let cos_theta = z / r;
        if cos_theta.abs() > 1.0 {
            return Err(CoordinateError::NumericalError("cos(theta) out of range".to_string()));
        }
        cos_theta.acos()
    };
    
    let phi = y.atan2(x);
    let phi = if phi < 0.0 { phi + 2.0 * PI } else { phi };

    if !theta.is_finite() || !phi.is_finite() {
        return Err(CoordinateError::NumericalError("Invalid angle calculation".to_string()));
    }

    Ok(Spherical { r, theta, phi })
}

/// Converts Spherical coordinates to Cartesian coordinates
///
/// # Arguments
///
/// * `sph` - A reference to a Spherical coordinate
///
/// # Returns
///
/// * `Cartesian` - The equivalent Cartesian coordinates
pub fn spherical_to_cartesian(sph: Spherical) -> Cartesian {
    let r = sph.r;
    let theta = sph.theta;
    let phi = sph.phi;

    let sin_theta = theta.sin();
    let x = r * sin_theta * phi.cos();
    let y = r * sin_theta * phi.sin();
    let z = r * theta.cos();

    Cartesian { x, y, z }
}

/// Safe version of spherical_to_cartesian with error checking
///
/// # Arguments
///
/// * `sph` - A Spherical coordinate
///
/// # Returns
///
/// * `Result<Cartesian, CoordinateError>` - The equivalent Cartesian coordinates or an error
pub fn spherical_to_cartesian_safe(sph: Spherical) -> Result<Cartesian, CoordinateError> {
    let r = sph.r;
    let theta = sph.theta;
    let phi = sph.phi;

    // Validate inputs
    if r < 0.0 {
        return Err(CoordinateError::InvalidRadius(r));
    }
    
    if !r.is_finite() || !theta.is_finite() || !phi.is_finite() {
        return Err(CoordinateError::NumericalError("Non-finite coordinate values".to_string()));
    }
    
    if theta < 0.0 || theta > PI {
        return Err(CoordinateError::InvalidAngle(theta));
    }

    let sin_theta = theta.sin();
    let x = r * sin_theta * phi.cos();
    let y = r * sin_theta * phi.sin();
    let z = r * theta.cos();

    if !x.is_finite() || !y.is_finite() || !z.is_finite() {
        return Err(CoordinateError::NumericalError("Cartesian calculation overflow".to_string()));
    }

    Ok(Cartesian { x, y, z })
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::f64::consts::{PI, SQRT_2};

    const EPSILON: f64 = 1e-10;

    fn approx_eq(a: f64, b: f64) -> bool {
        (a - b).abs() < EPSILON
    }

    #[test]
    fn test_origin_conversion() {
        let origin = Cartesian { x: 0.0, y: 0.0, z: 0.0 };
        let spherical = cartesian_to_spherical(origin);
        
        assert_eq!(spherical.r, 0.0);
        assert_eq!(spherical.theta, 0.0);
        
        let back_to_cartesian = spherical_to_cartesian(spherical);
        assert!(approx_eq(back_to_cartesian.x, 0.0));
        assert!(approx_eq(back_to_cartesian.y, 0.0));
        assert!(approx_eq(back_to_cartesian.z, 0.0));
    }

    #[test]
    fn test_unit_sphere_conversions() {
        let test_cases = vec![
            // (x, y, z) -> expected (r, theta, phi)
            (1.0, 0.0, 0.0, 1.0, PI/2.0, 0.0),           // +x axis
            (0.0, 1.0, 0.0, 1.0, PI/2.0, PI/2.0),        // +y axis
            (0.0, 0.0, 1.0, 1.0, 0.0, 0.0),              // +z axis
            (-1.0, 0.0, 0.0, 1.0, PI/2.0, PI),           // -x axis
            (0.0, -1.0, 0.0, 1.0, PI/2.0, 3.0*PI/2.0),   // -y axis
            (0.0, 0.0, -1.0, 1.0, PI, 0.0),              // -z axis
        ];

        for (x, y, z, expected_r, expected_theta, expected_phi) in test_cases {
            let cart = Cartesian { x, y, z };
            let sph = cartesian_to_spherical(cart);
            
            assert!(approx_eq(sph.r, expected_r), "r mismatch for ({}, {}, {}): expected {}, got {}", x, y, z, expected_r, sph.r);
            assert!(approx_eq(sph.theta, expected_theta), "theta mismatch for ({}, {}, {}): expected {}, got {}", x, y, z, expected_theta, sph.theta);
            assert!(approx_eq(sph.phi, expected_phi), "phi mismatch for ({}, {}, {}): expected {}, got {}", x, y, z, expected_phi, sph.phi);
        }
    }

    #[test]
    fn test_roundtrip_conversion() {
        let test_points = vec![
            Cartesian { x: 1.0, y: 2.0, z: 3.0 },
            Cartesian { x: -2.5, y: 1.7, z: -0.8 },
            Cartesian { x: 0.1, y: -0.1, z: 5.0 },
            Cartesian { x: SQRT_2, y: SQRT_2, z: 0.0 },
        ];

        for original in test_points {
            let spherical = cartesian_to_spherical(original);
            let back_to_cartesian = spherical_to_cartesian(spherical);
            
            assert!(approx_eq(original.x, back_to_cartesian.x), "x roundtrip failed: {} -> {}", original.x, back_to_cartesian.x);
            assert!(approx_eq(original.y, back_to_cartesian.y), "y roundtrip failed: {} -> {}", original.y, back_to_cartesian.y);
            assert!(approx_eq(original.z, back_to_cartesian.z), "z roundtrip failed: {} -> {}", original.z, back_to_cartesian.z);
        }
    }

    #[test]
    fn test_spherical_coordinate_ranges() {
        let test_points = vec![
            Cartesian { x: 1.0, y: 1.0, z: 1.0 },
            Cartesian { x: -1.0, y: -1.0, z: -1.0 },
            Cartesian { x: 3.0, y: -2.0, z: 1.5 },
        ];

        for point in test_points {
            let sph = cartesian_to_spherical(point);
            
            // r should be non-negative
            assert!(sph.r >= 0.0, "Radius should be non-negative: {}", sph.r);
            
            // theta should be in [0, PI]
            assert!(sph.theta >= 0.0 && sph.theta <= PI, "Theta should be in [0, PI]: {}", sph.theta);
            
            // phi should be in [0, 2*PI)
            assert!(sph.phi >= 0.0 && sph.phi < 2.0 * PI, "Phi should be in [0, 2*PI): {}", sph.phi);
        }
    }

    #[test]
    fn test_cartesian_to_spherical_safe_error_handling() {
        // Test non-finite values
        assert!(matches!(
            cartesian_to_spherical_safe(Cartesian { x: f64::NAN, y: 0.0, z: 0.0 }),
            Err(CoordinateError::NumericalError(_))
        ));
        
        assert!(matches!(
            cartesian_to_spherical_safe(Cartesian { x: f64::INFINITY, y: 0.0, z: 0.0 }),
            Err(CoordinateError::NumericalError(_))
        ));
        
        // Test very large values that might cause overflow
        let large_val = f64::MAX / 2.0;
        let result = cartesian_to_spherical_safe(Cartesian { x: large_val, y: large_val, z: large_val });
        // This should either succeed or fail gracefully
        match result {
            Ok(sph) => assert!(sph.r > 0.0),
            Err(CoordinateError::NumericalError(_)) => (), // Acceptable
            _ => panic!("Unexpected error type"),
        }
    }

    #[test]
    fn test_spherical_to_cartesian_safe_error_handling() {
        // Test negative radius
        assert!(matches!(
            spherical_to_cartesian_safe(Spherical { r: -1.0, theta: 0.0, phi: 0.0 }),
            Err(CoordinateError::InvalidRadius(_))
        ));
        
        // Test invalid theta
        assert!(matches!(
            spherical_to_cartesian_safe(Spherical { r: 1.0, theta: -0.1, phi: 0.0 }),
            Err(CoordinateError::InvalidAngle(_))
        ));
        
        assert!(matches!(
            spherical_to_cartesian_safe(Spherical { r: 1.0, theta: PI + 0.1, phi: 0.0 }),
            Err(CoordinateError::InvalidAngle(_))
        ));
        
        // Test non-finite values
        assert!(matches!(
            spherical_to_cartesian_safe(Spherical { r: f64::NAN, theta: 0.0, phi: 0.0 }),
            Err(CoordinateError::NumericalError(_))
        ));
    }

    #[test]
    fn test_safe_functions_match_unsafe_for_valid_inputs() {
        let test_points = vec![
            Cartesian { x: 1.0, y: 2.0, z: 3.0 },
            Cartesian { x: -1.5, y: 0.5, z: -2.0 },
            Cartesian { x: 0.0, y: 0.0, z: 1.0 },
        ];

        for point in test_points {
            let unsafe_result = cartesian_to_spherical(point);
            let safe_result = cartesian_to_spherical_safe(point).unwrap();
            
            assert!(approx_eq(unsafe_result.r, safe_result.r));
            assert!(approx_eq(unsafe_result.theta, safe_result.theta));
            assert!(approx_eq(unsafe_result.phi, safe_result.phi));
            
            let unsafe_back = spherical_to_cartesian(unsafe_result);
            let safe_back = spherical_to_cartesian_safe(safe_result).unwrap();
            
            assert!(approx_eq(unsafe_back.x, safe_back.x));
            assert!(approx_eq(unsafe_back.y, safe_back.y));
            assert!(approx_eq(unsafe_back.z, safe_back.z));
        }
    }
}
