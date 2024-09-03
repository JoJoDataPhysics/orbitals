# Motivation
This project should visualize the orbitals of the hydrogen atom using the Metropolis-Hastings algorithm:
Potential locations of the electron are set by moving in randomly oriented steps within a certain step radius.
Ratios of the probability densities are used to approve or reject the new potential positions.
A sequence of approved positions forms a random walk path in 3D space. The Nodes of this path ar used to visualize the shape of the orbital.
# Learning Goals
- Set up my first project in Rust
- Learn how to use mathematical functions in Rust
- Implement the Metropolis-Hastings algorithm in Rust (based on my experience from prior MC simulations I did in Python / R)
- Render 3D graphics in Rust with the kiss3d library
# Result
<img src="https://github.com/JoJoDataPhysics/orbitals/blob/main/3dz2.gif" alt="orbital" width="250"/>

# Project Architecture
```mermaid
graph TD
    A[(Quantum Theoretical 
        Probability Density)]
    B[(Metropolis-Hastings)]
    C[(Random Walk)]
    D[(3D Rendering)]

    A --> B --> C --> D


