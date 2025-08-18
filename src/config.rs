use std::fmt;

/// Available orbital types for simulation
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum OrbitalType {
    Hydrogen1s,
    Hydrogen2px,
    Hydrogen2py,
    Hydrogen2pz,
    Hydrogen3dZ2,
}

impl fmt::Display for OrbitalType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            OrbitalType::Hydrogen1s => write!(f, "1s"),
            OrbitalType::Hydrogen2px => write!(f, "2px"),
            OrbitalType::Hydrogen2py => write!(f, "2py"),
            OrbitalType::Hydrogen2pz => write!(f, "2pz"),
            OrbitalType::Hydrogen3dZ2 => write!(f, "3dz²"),
        }
    }
}

/// Configuration parameters for orbital simulation
#[derive(Debug, Clone)]
pub struct SimulationConfig {
    /// Type of orbital to simulate
    pub orbital_type: OrbitalType,
    /// Number of burn-in steps (for MCMC convergence)
    pub burn_in_steps: usize,
    /// Target number of points to collect
    pub target_points: usize,
    /// Skip factor (collect every nth accepted point)
    pub skip_factor: usize,
    /// Step radius for random walk
    pub step_radius: f64,
}

impl Default for SimulationConfig {
    fn default() -> Self {
        Self {
            orbital_type: OrbitalType::Hydrogen3dZ2,
            burn_in_steps: 20000,
            target_points: 90000,
            skip_factor: 100,
            step_radius: 1.5,
        }
    }
}

impl SimulationConfig {
    /// Create a new configuration with default values
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the orbital type
    pub fn orbital_type(mut self, orbital_type: OrbitalType) -> Self {
        self.orbital_type = orbital_type;
        self
    }

    /// Set the number of burn-in steps
    pub fn burn_in_steps(mut self, steps: usize) -> Self {
        self.burn_in_steps = steps;
        self
    }

    /// Set the target number of points
    pub fn target_points(mut self, points: usize) -> Self {
        self.target_points = points;
        self
    }

    /// Set the skip factor
    pub fn skip_factor(mut self, factor: usize) -> Self {
        self.skip_factor = factor;
        self
    }

    /// Set the step radius
    pub fn step_radius(mut self, radius: f64) -> Self {
        self.step_radius = radius;
        self
    }

    /// Validate the configuration
    pub fn validate(&self) -> Result<(), ConfigError> {
        if self.burn_in_steps == 0 {
            return Err(ConfigError::InvalidParameter("Burn-in steps must be > 0".to_string()));
        }
        
        if self.target_points == 0 {
            return Err(ConfigError::InvalidParameter("Target points must be > 0".to_string()));
        }
        
        if self.skip_factor == 0 {
            return Err(ConfigError::InvalidParameter("Skip factor must be > 0".to_string()));
        }
        
        if self.step_radius <= 0.0 || !self.step_radius.is_finite() {
            return Err(ConfigError::InvalidParameter("Step radius must be > 0 and finite".to_string()));
        }
        
        Ok(())
    }
}

/// Configuration-related errors
#[derive(Debug, PartialEq)]
pub enum ConfigError {
    InvalidParameter(String),
    IoError(String),
}

impl fmt::Display for ConfigError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ConfigError::InvalidParameter(msg) => write!(f, "Invalid parameter: {}", msg),
            ConfigError::IoError(msg) => write!(f, "IO error: {}", msg),
        }
    }
}

impl std::error::Error for ConfigError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config_is_valid() {
        let config = SimulationConfig::default();
        assert!(config.validate().is_ok());
    }

    #[test]
    fn test_config_builder_pattern() {
        let config = SimulationConfig::new()
            .orbital_type(OrbitalType::Hydrogen1s)
            .burn_in_steps(10000)
            .target_points(50000)
            .skip_factor(50)
            .step_radius(2.0);

        assert_eq!(config.orbital_type, OrbitalType::Hydrogen1s);
        assert_eq!(config.burn_in_steps, 10000);
        assert_eq!(config.target_points, 50000);
        assert_eq!(config.skip_factor, 50);
        assert_eq!(config.step_radius, 2.0);
        assert!(config.validate().is_ok());
    }

    #[test]
    fn test_config_validation() {
        // Test invalid burn-in steps
        let config = SimulationConfig::new().burn_in_steps(0);
        assert!(matches!(config.validate(), Err(ConfigError::InvalidParameter(_))));

        // Test invalid target points
        let config = SimulationConfig::new().target_points(0);
        assert!(matches!(config.validate(), Err(ConfigError::InvalidParameter(_))));

        // Test invalid skip factor
        let config = SimulationConfig::new().skip_factor(0);
        assert!(matches!(config.validate(), Err(ConfigError::InvalidParameter(_))));

        // Test invalid step radius
        let config = SimulationConfig::new().step_radius(0.0);
        assert!(matches!(config.validate(), Err(ConfigError::InvalidParameter(_))));

        let config = SimulationConfig::new().step_radius(-1.0);
        assert!(matches!(config.validate(), Err(ConfigError::InvalidParameter(_))));

        let config = SimulationConfig::new().step_radius(f64::NAN);
        assert!(matches!(config.validate(), Err(ConfigError::InvalidParameter(_))));
    }

    #[test]
    fn test_orbital_type_display() {
        assert_eq!(format!("{}", OrbitalType::Hydrogen1s), "1s");
        assert_eq!(format!("{}", OrbitalType::Hydrogen3dZ2), "3dz²");
    }
}