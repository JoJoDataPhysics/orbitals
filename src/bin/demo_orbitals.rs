extern crate orbitals;

use orbitals::coordinate_conversion::{Cartesian, cartesian_to_spherical};
use orbitals::h_orbitals::*;
use orbitals::config::{SimulationConfig, OrbitalType};
use orbitals::random_walk::*;
use std::f64::consts::PI;

fn analyze_orbital_shape(orbital_type: OrbitalType, name: &str) {
    println!("\n🎯 {} ORBITAL ANALYSIS", name.to_uppercase());
    println!("═══════════════════════════════════");
    
    // Test points at various locations
    let test_points = vec![
        (0.0, 0.0, 0.0, "Origin"),
        (1.0, 0.0, 0.0, "X-axis (+1)"),
        (0.0, 1.0, 0.0, "Y-axis (+1)"),
        (0.0, 0.0, 1.0, "Z-axis (+1)"),
        (0.0, 0.0, -1.0, "Z-axis (-1)"),
        (0.707, 0.707, 0.0, "XY diagonal"),
        (0.5, 0.5, 0.707, "XYZ diagonal"),
    ];
    
    println!("Probability densities at key positions:");
    for (x, y, z, label) in test_points {
        let cart = Cartesian { x, y, z };
        let sph = cartesian_to_spherical(cart);
        
        let density = match orbital_type {
            OrbitalType::Hydrogen1s => probability_density_1s(sph.r, sph.theta, sph.phi),
            OrbitalType::Hydrogen2px => probability_density_2p_x(sph.r, sph.theta, sph.phi),
            OrbitalType::Hydrogen2py => probability_density_2p_y(sph.r, sph.theta, sph.phi),
            OrbitalType::Hydrogen2pz => probability_density_2p_z(sph.r, sph.theta, sph.phi),
            OrbitalType::Hydrogen3dZ2 => probability_density_3d_z2(sph.r, sph.theta, sph.phi),
        };
        
        println!("  {:12} ({:5.2}, {:5.2}, {:5.2}): {:.6}", label, x, y, z, density);
    }
    
    // Run a mini-simulation to show distribution
    println!("\nMini-simulation (100 points):");
    let origin = Cartesian { x: 0.0, y: 0.0, z: 0.0 };
    let mut points = Vec::new();
    let mut position = origin;
    let mut accepted = 0;
    let mut rejected = 0;
    
    // Simple MCMC sampling
    for _ in 0..5000 {  // Generate more, keep fewer
        let new_pos = new_step(position, 1.5);
        let valid = match orbital_type {
            OrbitalType::Hydrogen1s => eval_step_1s(position, new_pos),
            OrbitalType::Hydrogen2px => eval_step_2p_x(position, new_pos),
            OrbitalType::Hydrogen2py => eval_step_2p_y(position, new_pos),
            OrbitalType::Hydrogen2pz => eval_step_2p_z(position, new_pos),
            OrbitalType::Hydrogen3dZ2 => eval_step_3d_z2(position, new_pos),
        };
        
        if valid {
            position = new_pos;
            accepted += 1;
            if accepted % 50 == 0 && points.len() < 100 {
                points.push(position);
            }
        } else {
            rejected += 1;
        }
    }
    
    println!("  Acceptance rate: {:.1}%", (accepted as f64 / (accepted + rejected) as f64) * 100.0);
    println!("  Sample points collected: {}", points.len());
    
    // Analyze distribution characteristics
    if !points.is_empty() {
        let mut distances = Vec::new();
        let mut z_values = Vec::new();
        
        for point in &points {
            let sph = cartesian_to_spherical(*point);
            distances.push(sph.r);
            z_values.push(point.z);
        }
        
        distances.sort_by(|a, b| a.partial_cmp(b).unwrap());
        z_values.sort_by(|a, b| a.partial_cmp(b).unwrap());
        
        let median_distance = distances[distances.len() / 2];
        let median_z = z_values[z_values.len() / 2];
        let max_distance = distances.last().unwrap();
        
        println!("  Median distance from origin: {:.3}", median_distance);
        println!("  Median Z-coordinate: {:.3}", median_z);
        println!("  Maximum distance: {:.3}", max_distance);
        
        // Show a few sample coordinates
        println!("  Sample coordinates:");
        for (i, point) in points.iter().take(5).enumerate() {
            println!("    Point {}: ({:.3}, {:.3}, {:.3})", i+1, point.x, point.y, point.z);
        }
    }
}

fn main() {
    println!("🚀 HYDROGEN ORBITAL VISUALIZATION DEMO");
    println!("====================================");
    println!("This demo shows the characteristic shapes and distributions");
    println!("of different hydrogen atom orbitals using Monte Carlo sampling.\n");
    
    // Analyze each orbital type
    analyze_orbital_shape(OrbitalType::Hydrogen1s, "1s");
    analyze_orbital_shape(OrbitalType::Hydrogen2pz, "2pz");
    analyze_orbital_shape(OrbitalType::Hydrogen2px, "2px");
    analyze_orbital_shape(OrbitalType::Hydrogen3dZ2, "3dz²");
    
    println!("\n📊 ORBITAL SHAPE DESCRIPTIONS");
    println!("════════════════════════════");
    println!("1s:   Spherical cloud centered at nucleus (ground state)");
    println!("2px:  Dumbbell shape along X-axis with nodal plane at x=0");
    println!("2py:  Dumbbell shape along Y-axis with nodal plane at y=0"); 
    println!("2pz:  Dumbbell shape along Z-axis with nodal plane at z=0");
    println!("3dz²: Complex shape with lobes along Z-axis and ring in XY-plane");
    
    println!("\n🎨 In the 3D visualization, you would see:");
    println!("• Point clouds forming these characteristic shapes");
    println!("• Higher density = more points = higher electron probability");
    println!("• Interactive rotation and zoom controls");
    println!("• Real-time rendering with performance optimization");
}