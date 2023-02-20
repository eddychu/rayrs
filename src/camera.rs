use glam::DVec3;

use crate::{ray::Ray, transform::Transform};

pub trait Camera : Send + Sync{
    fn get_ray(&self, u: f64, v: f64) -> Ray;
}

pub struct PerspectiveCamera {
    pub transform: Transform,
    pub vfov: f64,
    pub aspect_ratio: f64,
}

impl PerspectiveCamera {
    pub fn new(lookfrom: DVec3, lookat: DVec3, vup: DVec3, vfov: f64, aspect_ratio: f64) -> PerspectiveCamera {
        let transform = Transform::lookat(lookfrom, lookat, vup);
        PerspectiveCamera { transform, vfov, aspect_ratio }
    }
}

impl Camera for PerspectiveCamera {
    fn get_ray(&self, u: f64, v: f64) -> Ray {
        let theta = self.vfov.to_radians();
        let half_height = (theta / 2.0).tan();
        let half_width = self.aspect_ratio * half_height;
        let x = u * half_width;
        let y = v * half_height;
        let origin = self.transform.point_to_world(DVec3::ZERO);
        let dir_local = DVec3::new(x, y, -1.0);
        let dir_world = self.transform.vector_to_world(dir_local);
        Ray::new(origin, dir_world.normalize())
    }
}