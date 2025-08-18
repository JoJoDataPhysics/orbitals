extern crate orbitals;

use clap::Parser;
use orbitals::coordinate_conversion::Cartesian;
use orbitals::h_orbitals::BOHR_RADIUS;
use orbitals::random_walk::{eval_step_1s, eval_step_3d_z2, eval_step_2p_x, eval_step_2p_y, eval_step_2p_z, new_step};
use orbitals::render_cloud::{render_cloud_with_config, create_performance_config};
use orbitals::config::OrbitalType;
use orbitals::cli::Args;

fn main() {
    // Parse command-line arguments
    let args = Args::parse();
    
    // Check if user wants to list orbital types
    if args.list_orbitals {
        Args::print_orbital_types();
        return;
    }
    
    // Create configuration from CLI arguments
    let config = match args.to_config() {
        Ok(config) => config,
        Err(e) => {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }
    };
    
    // Validate configuration
    if let Err(e) = config.validate() {
        eprintln!("Configuration error: {}", e);
        return;
    }
    
    println!("Running orbital simulation: {}", config.orbital_type);
    println!("Burn-in steps: {}", config.burn_in_steps);
    println!("Target points: {}", config.target_points);
    println!("Skip factor: {}", config.skip_factor);
    println!("Step radius: {:.2}", config.step_radius);
    
    let origin = Cartesian { x: 0.0, y: 0.0, z: 0.0 };
    let mut points: Vec<Cartesian> = Vec::new();
    let mut rejected = 0;
    let mut accepted = 0;
    let mut stored = 0;
    let radius = config.step_radius * BOHR_RADIUS;
    let mut old_position = origin;
    
    // Burn-in period
    println!("Starting burn-in period...");
    for _ in 0..config.burn_in_steps {
        let new_position = new_step(old_position, radius);
        let valid_step = match config.orbital_type {
            OrbitalType::Hydrogen1s => eval_step_1s(old_position, new_position),
            OrbitalType::Hydrogen2px => eval_step_2p_x(old_position, new_position),
            OrbitalType::Hydrogen2py => eval_step_2p_y(old_position, new_position),
            OrbitalType::Hydrogen2pz => eval_step_2p_z(old_position, new_position),
            OrbitalType::Hydrogen3dZ2 => eval_step_3d_z2(old_position, new_position),
        };
        if valid_step {
            old_position = new_position;
        }
    }
    
    // Production period
    println!("Starting random walk simulation...");
    let mut cur_skip = 0;
    while stored < config.target_points {
        let new_position = new_step(old_position, radius);
        let valid_step = match config.orbital_type {
            OrbitalType::Hydrogen1s => eval_step_1s(old_position, new_position),
            OrbitalType::Hydrogen2px => eval_step_2p_x(old_position, new_position),
            OrbitalType::Hydrogen2py => eval_step_2p_y(old_position, new_position),
            OrbitalType::Hydrogen2pz => eval_step_2p_z(old_position, new_position),
            OrbitalType::Hydrogen3dZ2 => eval_step_3d_z2(old_position, new_position),
        };
        
        if valid_step {
            old_position = new_position;
            if cur_skip < config.skip_factor {
                cur_skip += 1;
                accepted += 1;
                continue;
            } else {
                cur_skip = 0;
                points.push(new_position);
                accepted += 1;
                stored += 1;
                
                // Progress indicator
                if stored % (config.target_points / 10) == 0 {
                    println!("Progress: {:.0}% ({}/{})", 
                             (stored as f64 / config.target_points as f64) * 100.0, 
                             stored, 
                             config.target_points);
                }
            }
        } else {
            rejected += 1;
        }
    }
    
    println!("\nSimulation complete!");
    println!("Stored points: {}", stored);
    println!("Accepted steps: {}", accepted);
    println!("Rejected steps: {}", rejected);
    println!("Acceptance rate: {:.2}%", (accepted as f64 / (accepted + rejected) as f64) * 100.0);
    
    println!("Preparing orbital visualization...");
    let render_config = create_performance_config(points.len());
    render_cloud_with_config(&points, &render_config);
}
