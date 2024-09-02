use std::f64::consts::PI;

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
