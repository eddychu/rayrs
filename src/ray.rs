use std::f64::{EPSILON, MAX};

use glam::DVec3;

#[derive(Debug, Clone)]
pub struct Ray {
    pub origin: DVec3,
    pub direction: DVec3,
    pub min_t: f64,
    pub max_t: f64,
}

impl Ray {
    pub fn new(origin: DVec3, direction: DVec3) -> Ray {
        Ray { origin, direction, min_t : EPSILON, max_t: MAX }
    }

    pub fn at(&self, t: f64) -> DVec3 {
        self.origin + self.direction * t
    }
}

