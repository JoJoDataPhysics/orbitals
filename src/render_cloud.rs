extern crate kiss3d;
use core::f32;

use crate::coordinate_conversion::Cartesian;
use kiss3d::camera::ArcBall;
use kiss3d::light::Light;
use kiss3d::nalgebra::{Point3, Translation3};
use kiss3d::window::Window;

/// Configuration for rendering performance
pub struct RenderConfig {
    /// Maximum number of points to render (for performance)
    pub max_points: usize,
    /// Scale factor for visualization
    pub scale: f64,
    /// Sphere radius for each point
    pub sphere_radius: f32,
}

impl Default for RenderConfig {
    fn default() -> Self {
        Self {
            max_points: 50000,
            scale: 0.05,
            sphere_radius: 0.0015,
        }
    }
}

/// Render the orbital point cloud with default settings
pub fn render_cloud(positions: &Vec<Cartesian>) {
    render_cloud_with_config(positions, &RenderConfig::default());
}

/// Render the orbital point cloud with custom configuration
pub fn render_cloud_with_config(positions: &Vec<Cartesian>, config: &RenderConfig) {
    let point_count = positions.len().min(config.max_points);
    
    println!("Rendering {} points ({}% of total)", 
             point_count, 
             (point_count as f64 / positions.len() as f64) * 100.0);

    // Pre-allocate vector with known capacity for better performance
    let mut points = Vec::with_capacity(point_count);
    
    // Use iterator sampling for better performance and memory usage
    let step = if positions.len() > config.max_points {
        positions.len() / config.max_points
    } else {
        1
    };

    for (i, point) in positions.iter().enumerate() {
        if i % step == 0 && points.len() < config.max_points {
            // Apply scaling transformation once
            let scaled_point = Point3::new(
                (point.x * config.scale) as f32,
                (point.y * config.scale) as f32,
                (point.z * config.scale) as f32,
            );
            points.push(scaled_point);
        }
    }

    println!("Initializing 3D window...");
    
    // Wayland compatibility fix - detect Wayland and fallback to X11
    if std::env::var("XDG_SESSION_TYPE").unwrap_or_default() == "wayland" {
        println!("Detected Wayland session, attempting X11 fallback for graphics compatibility...");
        std::env::set_var("WAYLAND_DISPLAY", "");
        // Also try setting GDK backend as additional fallback
        std::env::set_var("GDK_BACKEND", "x11");
    }
    
    let mut window = Window::new("Hydrogen Orbital Visualization");

    // Set up optimized lighting and camera
    window.set_light(Light::StickToCamera);
    let eye = Point3::new(0.0, 4.0, 0.0);
    let at = Point3::new(0.0, 0.0, 0.0);
    let mut camera = ArcBall::new(eye, at);

    println!("Adding {} spheres to scene...", points.len());
    
    // Batch sphere creation for better performance
    for (i, point) in points.iter().enumerate() {
        let mut sphere = window.add_sphere(config.sphere_radius);
        let translation = Translation3::new(point.x, point.y, point.z);
        sphere.set_local_translation(translation);
        
        // Progress indicator for large point clouds
        if i % 10000 == 0 && i > 0 {
            println!("Added {} spheres...", i);
        }
    }

    println!("Rendering complete! Use mouse to rotate, scroll to zoom.");
    println!("Close the window to exit.");

    // Render loop with camera controls
    while window.render_with_camera(&mut camera) {
        // The rendering loop handles user interaction automatically
    }
}

/// Create an optimized render configuration for large point clouds
pub fn create_performance_config(point_count: usize) -> RenderConfig {
    let max_points = if point_count > 100000 {
        25000  // Reduce points for very large clouds
    } else if point_count > 50000 {
        40000  // Moderate reduction for large clouds
    } else {
        point_count  // Use all points for smaller clouds
    };

    RenderConfig {
        max_points,
        scale: 0.05,
        sphere_radius: if max_points > 30000 { 0.001 } else { 0.0015 },
    }
}
