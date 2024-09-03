use crate::coordinate_conversion::{
    cartesian_to_spherical, spherical_to_cartesian, Cartesian, Spherical,
};
use crate::h_orbitals::probability_density_1s;
use crate::h_orbitals::probability_density_3d_z2;
use crate::h_orbitals::BOHR_RADIUS;
use rand::Rng;
use std::f64::consts::PI;

fn eval_step_probability(prob_1: f64, prob_2: f64) -> bool {
    // Accept the step if the probability of the new position is greater than the probability of
    // the old position. If not, accept the step with a probability equal to the ratio of the two
    // probabilities.
    (1.0_f64).min(prob_2 / prob_1) > rand::thread_rng().gen::<f64>()
}

pub fn new_step(start_pt: Cartesian, radius: f64) -> Cartesian {
    // Generate a new step in a random direction with a random distance
    // inside given radius from the start point.
    let mut rng = rand::thread_rng();
    let r = radius * BOHR_RADIUS * rng.gen::<f64>();
    let theta = 2.0 * PI * rng.gen::<f64>();
    let phi = PI * rng.gen::<f64>();
    let sph = Spherical { r, theta, phi };
    let inc = spherical_to_cartesian(sph);
    Cartesian {
        x: start_pt.x + inc.x,
        y: start_pt.y + inc.y,
        z: start_pt.z + inc.z,
    }
}

pub fn eval_step_1s(pos_1: Cartesian, pos_2: Cartesian) -> bool {
    //Evaluation based on 1s wavefunction
    let sph_1 = cartesian_to_spherical(pos_1);
    let sph_2 = cartesian_to_spherical(pos_2);
    let r_1 = sph_1.r;
    let r_2 = sph_2.r;
    let prob_1 = probability_density_1s(r_1, 0.0, 0.0);
    let prob_2 = probability_density_1s(r_2, 0.0, 0.0);
    eval_step_probability(prob_1, prob_2)
}

pub fn eval_step_3d_z2(pos_1: Cartesian, pos_2: Cartesian) -> bool {
    //Evaluation based on 3d z^2 wavefunction
    let sph_1 = cartesian_to_spherical(pos_1);
    let sph_2 = cartesian_to_spherical(pos_2);
    let r_1 = sph_1.r;
    let r_2 = sph_2.r;
    let theta_1 = sph_1.theta;
    let theta_2 = sph_2.theta;
    let phi_1 = sph_1.phi;
    let phi_2 = sph_2.phi;
    let prob_1 = probability_density_3d_z2(r_1, theta_1, phi_1);
    let prob_2 = probability_density_3d_z2(r_2, theta_2, phi_2);
    eval_step_probability(prob_1, prob_2)
}
