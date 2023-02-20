use std::f64::consts::PI;

use glam::{DVec3};

pub fn sample_sphere(x: f64, y: f64) -> DVec3 {
    let phi = 2.0 * PI * x;
    let cos_theta = 1.0 - 2.0 * y;
    let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();
    DVec3::new(phi.cos() * sin_theta, phi.sin() * sin_theta, cos_theta).normalize()
}

pub fn sample_sphere_pdf() -> f64 {
    1.0 / (4.0 * PI)
}

pub fn sample_hemisphere(x: f64, y: f64) -> DVec3 {
    let phi = 2.0 * PI * x;
    let cos_theta = y;
    let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();
    DVec3::new(phi.cos() * sin_theta, phi.sin() * sin_theta, cos_theta).normalize()
}

pub fn sample_hemisphere_pdf() -> f64 {
    1.0 / (2.0 * PI)
}

pub fn sample_hemisphere_cosine(x: f64, y: f64) -> DVec3 {
    let phi = 2.0 * PI * x;
    let cos_theta = (1.0 - y).sqrt();
    let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();
    DVec3::new(phi.cos() * sin_theta, phi.sin() * sin_theta, cos_theta).normalize()
}

pub fn sample_hemisphere_cosine_pdf(cos_theta: f64) -> f64 {
    cos_theta / PI
}
