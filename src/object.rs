use std::rc::Rc;

use crate::{material::Material, hittable::{Hittable, HitRecord}, ray::Ray, bbox::BBox};

pub struct Object {
    pub material: Rc<Box<dyn Material>>,
    pub hittable: Rc<Box<dyn Hittable>>,
}

impl Object {
    pub fn new(hittable: Rc<Box<dyn Hittable>>, material: Rc<Box<dyn Material>>) -> Object {
        Object { material, hittable }
    }
}

impl Hittable for Object {
    fn hit(&self, ray: &Ray) -> Option<HitRecord> {
        if let Some(record) = self.hittable.hit(ray) {
            Some(HitRecord { p: record.p, normal: record.normal, t: record.t, object: Some(self), shape: Some(self.hittable.clone()) })
        } else {
            None
        }
    }

    fn bbox(&self) -> BBox {
        self.hittable.bbox()
    }
}