extern crate kiss3d;
use core::f32;

use crate::coordinate_conversion::Cartesian;
use kiss3d::camera::ArcBall;
use kiss3d::light::Light;
use kiss3d::nalgebra::{Point3, Translation3};
use kiss3d::window::Window;

pub fn render_cloud(positions: &Vec<Cartesian>) {
    // Define a set of 3D points.
    let mut points: Vec<Point3<f32>> = Vec::new();
    for point in positions {
        let scale: f64 = 0.05;
        let point = Cartesian {
            x: point.x * scale,
            y: point.y * scale,
            z: point.z * scale,
        };
        points.push(Point3::new(point.x as f32, point.y as f32, point.z as f32));
    }
    let mut window = Window::new("3D Point Cloud");

    // Set up lighting.
    window.set_light(Light::StickToCamera);
    let eye = Point3::new(0.0, 4.0, 0.0); // Camera position along the X-axis
    let at = Point3::new(0.0, 0.0, 0.0); // Looking at the origin
    let mut camera = ArcBall::new(eye, at);
    // Iterate over each point and add it to the scene as a sphere.
    for point in points {
        let mut sphere = window.add_sphere(0.0015); // Small sphere for each point
        let translation = Translation3::new(point.x, point.y, point.z);
        sphere.set_local_translation(translation); // Set position
    }

    // Keep the window open and interactive.
    //while window.render() {}
    while window.render_with_camera(&mut camera) {}
}
