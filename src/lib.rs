//! # Hydrogen Atom Orbital Visualization
//!
//! This crate provides tools for visualizing hydrogen atom orbitals using Monte Carlo methods.
//! It implements the Metropolis-Hastings algorithm to sample electron positions according to
//! quantum mechanical probability distributions.
//!
//! ## Core Modules
//!
//! - [`config`]: Configuration system for simulation parameters
//! - [`h_orbitals`]: Quantum mechanical probability density functions
//! - [`random_walk`]: Metropolis-Hastings MCMC implementation
//! - [`coordinate_conversion`]: Cartesian ↔ Spherical coordinate transformations
//! - [`render_cloud`]: 3D visualization using kiss3d
//!
//! ## Example Usage
//!
//! ```rust
//! use orbitals::config::{SimulationConfig, OrbitalType};
//!
//! // Create configuration for 2pz orbital
//! let config = SimulationConfig::new()
//!     .orbital_type(OrbitalType::Hydrogen2pz)
//!     .target_points(10000);
//!
//! // Validate parameters
//! assert!(config.validate().is_ok());
//! ```

pub mod coordinate_conversion;
pub mod h_orbitals;
pub mod random_walk;
pub mod render_cloud;
pub mod config;
pub mod cli;
