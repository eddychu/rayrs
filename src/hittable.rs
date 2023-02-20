use std::rc::Rc;

use glam::DVec3;

use crate::{ray::Ray, object::Object, bbox::BBox};

pub struct HitRecord<'object> {
    pub p: DVec3,
    pub normal: DVec3,
    pub t: f64,
    pub shape: Option<Rc<Box<dyn Hittable>>>,
    pub object: Option<&'object Object>,
}

pub trait Hittable {
    fn hit(&self, ray: &Ray) -> Option<HitRecord>;
    fn bbox(&self) -> BBox {
        BBox::default()
    }
}

pub struct Sphere {
    pub center: DVec3,
    pub radius: f64,
}

impl Sphere {
    pub fn new(center: DVec3, radius: f64) -> Sphere {
        Sphere { center, radius }
    }

}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray) -> Option<HitRecord> {
        let oc = ray.origin - self.center;
        let a = ray.direction.length_squared();
        let half_b = oc.dot(ray.direction);
        let c  = oc.length_squared() - self.radius * self.radius;
        let discriminant = half_b * half_b - a * c;
        if discriminant < 0.0 {
            return None;
        }
        let sqrtd = discriminant.sqrt();
        let mut root = (-half_b - sqrtd) / a;
        if root <= ray.min_t || root >= ray.max_t {
            root = (-half_b + sqrtd) / a;
            if root <= ray.min_t || root >= ray.max_t {
                return None;
            }
        }
        let t = root;
        let p = ray.at(t);
        let normal = (p - self.center) / self.radius;
        Some(HitRecord { p, normal, t, object: None, shape: None})
    }

    fn bbox(&self) -> BBox {
        BBox::new(
            self.center - DVec3::splat(self.radius + 0.0001) ,
            self.center + DVec3::splat(self.radius + 0.0001),
        )
    }
}

pub struct HittableList {
    pub objects: Vec<Rc<Box<dyn Hittable>>>,
}

impl HittableList {
    pub fn new() -> HittableList {
        HittableList { objects: Vec::new() }
    }

    pub fn add(&mut self, object: Rc<Box<dyn Hittable>>) {
        self.objects.push(object);
    }
}


impl Hittable for HittableList {
    fn hit(&self, ray: &Ray) -> Option<HitRecord> {
        let mut closest_so_far = ray.max_t;
        let mut hit_record = None;
        for object in &self.objects {
            if let Some(rec) = object.hit(ray) {
                if rec.t < closest_so_far {
                    closest_so_far = rec.t;
                    hit_record = Some(rec);
                }
            }
        }
        hit_record
    }

    fn bbox(&self) -> BBox {
        if self.objects.is_empty() {
            return BBox::default();
        }
        let mut bbox = self.objects[0].bbox();
        for object in &self.objects {
            bbox = bbox.union(&object.bbox());
        }
        bbox
    }
}