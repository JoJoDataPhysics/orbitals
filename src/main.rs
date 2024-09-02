mod coordinate_conversion;
mod h_orbitals;
use coordinate_conversion::{cartesian_to_spherical, spherical_to_cartesian, Cartesian, Spherical};
use h_orbitals::probability_density_1s;
use h_orbitals::BOHR_RADIUS;

fn main() {
    // e Cartesian coordinate
    let cartesian = Cartesian {
        x: 3.0,
        y: 4.0,
        z: 5.0,
    };
    println!("Original Cartesian coordinates: {:?}", cartesian);

    let spherical = Spherical {
        r: 3.0,
        theta: 71_f64.to_radians(),
        phi: 45_f64.to_radians(),
    };

    println!("Original Spherical coordinates: {:?}", spherical);
    // Convert to Spherical coordinates
    let sph = cartesian_to_spherical(cartesian);
    let cart = spherical_to_cartesian(spherical);
    println!(
        "Converted to Spherical coordinates:\n  r = {:.5}\n  theta = {:.5} rad ({:.2} degrees)\n  phi = {:.5} rad ({:.2} degrees)",
        sph.r,
        sph.theta,
        sph.theta.to_degrees(),
        sph.phi,
        sph.phi.to_degrees()
    );

    // Convert back to Cartesian coordinates
    let cart_converted = spherical_to_cartesian(sph);
    println!(
        "Converted back to Cartesian coordinates: {:?}",
        cart_converted
    );
    println!("Converted to Cartesian coordinates: {:?}", cart);

    // Example: Calculate the probability density at various distances from the nucleus
    let distances = [0.0, 0.1 * BOHR_RADIUS, BOHR_RADIUS, 2.0 * BOHR_RADIUS];

    println!("\nExample: Calculate the probability density at various distances from the proton:");
    println!("Probability density for the 1s orbital:");
    for &r in distances.iter() {
        let density = probability_density_1s(r);
        println!(
            "At r = {:.2e} m, probability density = {:.5e} m^-3",
            r, density
        );
    }
}
