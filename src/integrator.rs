use glam::DVec3;

use crate::{ray::Ray, scene::Scene, sampler::Sampler, accel::Accel};

pub trait Integrator : Send + Sync{
    fn li(&self, ray: &Ray, accel: &dyn Accel, sampler: &mut dyn Sampler, depth: i32) -> DVec3;
}

pub struct TestIntegrator {
    
}

impl TestIntegrator {
    pub fn new() -> TestIntegrator {
        TestIntegrator {}
    }
}

impl Integrator for TestIntegrator {
    fn li(&self, ray: &Ray, accel: &dyn Accel,sampler: &mut dyn Sampler, depth: i32) -> DVec3 {
        if depth <= 0 {
            return DVec3::ZERO;
        }
        if let Some(record) = accel.hit(ray) {
            if let Some(object) = record.object {
                let material = &object.material;
                if let Some((scattered, attenuation)) = material.scatter(ray, &record, sampler) {
                    return attenuation * self.li(&scattered, accel, sampler, depth - 1);
                }
            }
            return DVec3::ZERO;
        }

        let unit_direction = ray.direction.normalize();
        let t = 0.5 * (unit_direction.y + 1.0);
        (1.0 - t) * DVec3::new(1.0, 1.0, 1.0) + t * DVec3::new(0.5, 0.7, 1.0)
    }   
}



