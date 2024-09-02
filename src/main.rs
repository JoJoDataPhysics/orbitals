extern crate orbitals;
use orbitals::coordinate_conversion::cartesian_to_spherical;
use orbitals::coordinate_conversion::Cartesian;
use orbitals::random_walk::eval_step;
use orbitals::random_walk::new_step;

fn main() {
    let origin = orbitals::coordinate_conversion::Cartesian {
        x: 0.0,
        y: 0.0,
        z: 0.0,
    };
    println!("simulating random walk");
    let mut points: Vec<Cartesian> = Vec::new();
    let mut rejected = 0;
    let mut accepted = 0;
    let radius = 0.10;
    let mut old_position = origin;
    for _ in 0..10000 {
        let new_position = new_step(old_position, radius);
        let valid_step = eval_step(old_position, new_position);
        if valid_step {
            //println!("New position: {:?}", new_position);
            old_position = new_position;
            points.push(new_position);
            accepted += 1;
        } else {
            rejected += 1;
        }
    }
    println!("Accepted steps: {}", accepted);
    println!("Rejected steps: {}", rejected);
    for point in points {
        println!("{:?}", cartesian_to_spherical(point));
    }
}
