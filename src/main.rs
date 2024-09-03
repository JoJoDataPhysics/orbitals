extern crate orbitals;

use orbitals::coordinate_conversion::Cartesian;
use orbitals::h_orbitals::BOHR_RADIUS;
use orbitals::random_walk::eval_step_1s;
use orbitals::random_walk::eval_step_3d_z2;
use orbitals::random_walk::new_step;
use orbitals::render_cloud::render_cloud;

fn main() {
    let origin = orbitals::coordinate_conversion::Cartesian {
        x: 0.0,
        y: 0.0,
        z: 0.0,
    };
    let mut points: Vec<Cartesian> = Vec::new();
    let mut rejected = 0;
    let mut accepted = 0;
    let mut stored = 0;
    let radius = 1.5 * BOHR_RADIUS;
    let mut old_position = origin;
    // Burn-in period
    println!("starting burn-in");
    for _ in 0..20000 {
        let new_position = new_step(old_position, radius);
        let valid_step = eval_step_1s(old_position, new_position);
        if valid_step {
            old_position = new_position;
        }
    }
    // Production period
    println!("starting random walk");
    let target = 90000;
    let skip = 100;
    let mut cur_skip = 0;
    while stored < target {
        let new_position = new_step(old_position, radius);
        let valid_step = eval_step_3d_z2(old_position, new_position);
        if valid_step {
            old_position = new_position;
            if cur_skip < skip {
                cur_skip += 1;
                accepted += 1;
                continue;
            } else {
                cur_skip = 0;
                points.push(new_position);
                accepted += 1;
                stored += 1;
            }
        } else {
            rejected += 1;
        }
    }
    println!("stored steps: {}", stored);
    println!("Accepted steps: {}", accepted);
    println!("Rejected steps: {}", rejected);
    render_cloud(&points);
}
