
use crate::accel::{BVHNode, Accel, BVH};
use crate::hittable::{Hittable, HitRecord};
use crate::object::Object;
use crate::ray::Ray;

pub struct Scene {
    pub objects: Vec<Object>,
}

unsafe impl Send for Scene {}
unsafe impl Sync for Scene {}

impl Scene {
    pub fn new() -> Scene {
        Scene { objects: Vec::new()}
    }

    pub fn hit(&self, ray: &Ray) -> Option<HitRecord> {
        
        let mut closest = std::f64::MAX;
        let mut hit = None;
        for object in &self.objects {
            if let Some(record) = object.hit(ray) {
                if record.t < closest {
                    closest = record.t;
                    hit = Some(record);
                }
            }
        }
        hit
    }

    pub fn add(&mut self, object: Object) {
        self.objects.push(object);
    }
}