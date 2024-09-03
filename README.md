# Project Architecture

This diagram represents the flow of data through the system, from the quantum theoretical probability density to the Metropolis-Hastings module and finally to the 3D rendering stage.

```mermaid
graph TD
    A[(Quantum Theoretical Probability Density)]
    B[(Metropolis-Hastings Module)]
    C[(3D Rendering)]

    A --> B --> C
