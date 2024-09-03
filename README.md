# Motivation
This project should simulate the orbitals of the hydrogen atom using the Metropolis-Hastings algorithm:
The potential location of the electron is calculated by calculating isotropic steps within a certain radius.
The probability density according to the wave function is used to approve or reject the new potential positions.
A sequence of approved positions forms a random path in 3D space, which is used to visualize the shape of the orbital.
# Learning Goals
- set up a project in Rust
- Learn how to use mathematical functions in Rust
- Implement the Metropolis-Hastings algorithm
- Render 3D graphics in Rust with the kiss3d library
# Result

<img src="https://github.com/JoJoDataPhysics/orbitals/blob/main/3dz2.gif" alt="orbital" width="150"/>

# Project Architecture
```mermaid
graph TD
    A[(Quantum Theoretical Probability Density)]
    B[(Metropolis-Hastings Algorithm)]
    C[(Random Walk)]
    D[(3D Rendering)]

    A --> B --> C --> D


