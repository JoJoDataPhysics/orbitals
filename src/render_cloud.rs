extern crate kiss3d;
use crate::coordinate_conversion::Cartesian;
use crate::h_orbitals::BOHR_RADIUS;
use kiss3d::light::Light;
use kiss3d::nalgebra::{Point3, Translation3};
use kiss3d::window::Window;

pub fn render_cloud(positions: &Vec<Cartesian>) {
    // Define a set of 3D points.
    let mut points: Vec<Point3<f32>> = Vec::new();
    for point in positions {
        points.push(Point3::new(point.x as f32, point.y as f32, point.z as f32));
    }
    let mut window = Window::new("3D Point Cloud");

    // Set up lighting.
    window.set_light(Light::StickToCamera);

    // Iterate over each point and add it to the scene as a sphere.
    for point in points {
        let mut sphere = window.add_sphere(0.005); // Small sphere for each point
        let translation = Translation3::new(point.x, point.y, point.z);
        sphere.set_local_translation(translation); // Set position
    }

    // Keep the window open and interactive.
    while window.render() {}
}
