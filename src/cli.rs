use clap::{Parser, ValueEnum};
use crate::config::{OrbitalType, SimulationConfig};

/// Command-line interface for orbital visualization
#[derive(Parser)]
#[command(name = "orbitals")]
#[command(about = "Visualize hydrogen atom orbitals using Monte Carlo simulation")]
#[command(version)]
pub struct Args {
    /// Type of orbital to visualize
    #[arg(short, long, value_enum, default_value_t = CliOrbitalType::Dz2)]
    pub orbital: CliOrbitalType,

    /// Number of points to collect
    #[arg(short = 'n', long, default_value_t = 90000)]
    pub points: usize,

    /// Number of burn-in steps
    #[arg(short, long, default_value_t = 20000)]
    pub burn_in: usize,

    /// Skip factor (collect every nth accepted point)
    #[arg(short, long, default_value_t = 100)]
    pub skip: usize,

    /// Step radius for random walk
    #[arg(short, long, default_value_t = 1.5)]
    pub radius: f64,

    /// List available orbital types and exit
    #[arg(long)]
    pub list_orbitals: bool,
}

/// CLI-friendly orbital type enumeration
#[derive(Clone, Debug, ValueEnum)]
pub enum CliOrbitalType {
    /// 1s orbital (ground state)
    #[value(name = "1s")]
    S1,
    /// 2px orbital
    #[value(name = "2px")]
    Px2,
    /// 2py orbital  
    #[value(name = "2py")]
    Py2,
    /// 2pz orbital
    #[value(name = "2pz")]
    Pz2,
    /// 3dz² orbital
    #[value(name = "3dz2")]
    Dz2,
}

impl From<CliOrbitalType> for OrbitalType {
    fn from(cli_orbital: CliOrbitalType) -> Self {
        match cli_orbital {
            CliOrbitalType::S1 => OrbitalType::Hydrogen1s,
            CliOrbitalType::Px2 => OrbitalType::Hydrogen2px,
            CliOrbitalType::Py2 => OrbitalType::Hydrogen2py,
            CliOrbitalType::Pz2 => OrbitalType::Hydrogen2pz,
            CliOrbitalType::Dz2 => OrbitalType::Hydrogen3dZ2,
        }
    }
}

impl Args {
    /// Convert CLI arguments to simulation configuration
    pub fn to_config(&self) -> Result<SimulationConfig, String> {
        let config = SimulationConfig::new()
            .orbital_type(self.orbital.clone().into())
            .target_points(self.points)
            .burn_in_steps(self.burn_in)
            .skip_factor(self.skip)
            .step_radius(self.radius);

        config.validate().map_err(|e| format!("Configuration error: {}", e))?;
        Ok(config)
    }

    /// Print available orbital types
    pub fn print_orbital_types() {
        println!("Available orbital types:");
        println!("  1s    - 1s orbital (ground state, spherically symmetric)");
        println!("  2px   - 2px orbital (dumbbell along x-axis)");
        println!("  2py   - 2py orbital (dumbbell along y-axis)");
        println!("  2pz   - 2pz orbital (dumbbell along z-axis)");
        println!("  3dz2  - 3dz² orbital (complex d orbital shape)");
        println!();
        println!("Example usage:");
        println!("  cargo run -- --orbital 2pz --points 50000");
        println!("  cargo run -- -o 1s -n 100000 -b 30000");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cli_orbital_conversion() {
        assert_eq!(OrbitalType::from(CliOrbitalType::S1), OrbitalType::Hydrogen1s);
        assert_eq!(OrbitalType::from(CliOrbitalType::Px2), OrbitalType::Hydrogen2px);
        assert_eq!(OrbitalType::from(CliOrbitalType::Py2), OrbitalType::Hydrogen2py);
        assert_eq!(OrbitalType::from(CliOrbitalType::Pz2), OrbitalType::Hydrogen2pz);
        assert_eq!(OrbitalType::from(CliOrbitalType::Dz2), OrbitalType::Hydrogen3dZ2);
    }

    #[test]
    fn test_config_creation() {
        let args = Args {
            orbital: CliOrbitalType::S1,
            points: 1000,
            burn_in: 500,
            skip: 10,
            radius: 2.0,
            list_orbitals: false,
        };

        let config = args.to_config().unwrap();
        assert_eq!(config.orbital_type, OrbitalType::Hydrogen1s);
        assert_eq!(config.target_points, 1000);
        assert_eq!(config.burn_in_steps, 500);
        assert_eq!(config.skip_factor, 10);
        assert_eq!(config.step_radius, 2.0);
    }
}