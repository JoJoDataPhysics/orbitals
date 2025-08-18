# AGENTS.md - Development Guidelines

## Build/Test Commands
- `cargo build` - Build the project
- `cargo run` - Run the main orbital visualization
- `cargo test` - Run all tests
- `cargo test test_name` - Run specific test by name
- `cargo check` - Quick syntax/type checking without building
- `cargo fmt` - Format code according to Rust standards
- `cargo clippy` - Run lints and suggestions

## Code Style Guidelines
- **Imports**: Use explicit `use` statements, group std library imports first
- **Formatting**: Follow `cargo fmt` standards (already configured)
- **Types**: Use explicit type annotations for struct fields, rely on inference for locals
- **Naming**: snake_case for functions/variables, PascalCase for structs/enums
- **Documentation**: Use `///` for public items with clear descriptions and examples
- **Constants**: Use `pub const` with ALL_CAPS naming (e.g., `BOHR_RADIUS`)
- **Error Handling**: Use custom error enums with `std::error::Error` trait implementation
- **Structs**: Use `#[derive(Debug, Clone, Copy)]` for simple data structures
- **Module Organization**: Each module in separate file, expose via `lib.rs`
- **Tests**: Place tests in `#[cfg(test)]` modules at bottom of files

## Project Context
This is a scientific visualization project for hydrogen atom orbitals using Metropolis-Hastings MCMC simulation. The main dependencies are `rand` for random number generation and `kiss3d` for 3D rendering. Focus on mathematical accuracy and clear physics-based naming conventions.