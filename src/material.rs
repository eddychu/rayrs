use glam::DVec3;

use crate::{ray::Ray, hittable::HitRecord, sampler::Sampler, sampling::{sample_hemisphere, sample_sphere}};

pub trait Material {
    fn scatter(&self, ray: &Ray, hit: &HitRecord, sampler: &mut dyn Sampler) -> Option<(Ray, DVec3)>;
}

pub struct Lambertian {
    pub albedo: DVec3,
}

impl Lambertian {
    pub fn new(albedo: DVec3) -> Lambertian {
        Lambertian { albedo }
    }
}

impl Material for Lambertian {
    fn scatter(&self, ray: &Ray, hit: &HitRecord, sampler: &mut dyn Sampler) -> Option<(Ray, DVec3)> {
        let (r1, r2) = sampler.get_2d();
        let target = hit.p + hit.normal + sample_hemisphere(r1, r2);
        let scattered = Ray::new(hit.p, (target - hit.p).normalize());
        Some((scattered, self.albedo))
    }
}

pub struct Metal {
    pub albedo: DVec3,
    pub fuzz: f64,
}

impl Metal {
    pub fn new(albedo: DVec3, fuzz: f64) -> Metal {
        Metal { albedo, fuzz }
    }
}

fn reflect(v: DVec3, n: DVec3) -> DVec3 {
    let v = v.normalize();
    let n = n.normalize();
    ( v - 2.0 * v.dot(n) * n).normalize()
}


fn schlick(cosine: f64, eta: f64) -> f64 {
    let r0 = (1.0 - eta) / (1.0 + eta);
    let r0 = r0 * r0;
    r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
}

fn refract(v: DVec3, n: DVec3, eta: f64) -> Option<DVec3> {
    let v = v.normalize();
    let n = n.normalize();
    let n_dot_v = n.dot(v);
    let k = 1.0 - eta * eta * (1.0 - n_dot_v * n_dot_v);
    if k < 0.0 {
        None
    } else {
        Some((eta * v - (eta * n_dot_v + k.sqrt()) * n).normalize())
    }
}



impl Material for Metal {
    fn scatter(&self, ray: &Ray, hit: &HitRecord, sampler: &mut dyn Sampler) -> Option<(Ray, DVec3)> {
        if hit.normal.dot(ray.direction) > 0.0 {
            return None;
        }
        
        let reflected = reflect(ray.direction, hit.normal);
        let (r1, r2) = sampler.get_2d();
        let scattered = Ray::new(hit.p, (reflected + self.fuzz * sample_sphere(r1, r2)).normalize());
        if scattered.direction.dot(hit.normal) > 0.0 {
            Some((scattered, self.albedo))
        } else {
            None
        }
    }
}



pub struct Dielectric {
    pub ref_idx: f64,
}

impl Dielectric {
    pub fn new(ref_idx: f64) -> Dielectric {
        Dielectric { ref_idx }
    }
}

impl Material for Dielectric {
    fn scatter(&self, ray: &Ray, hit: &HitRecord, sampler: &mut dyn Sampler) -> Option<(Ray, DVec3)> {
        let attenuation = DVec3::new(1.0, 1.0, 1.0);
        let front_face = ray.direction.dot(hit.normal) < 0.0;
        let normal = if front_face { hit.normal } else { -hit.normal };
        let eta = if front_face { 1.0 / self.ref_idx } else { self.ref_idx };
        if let Some(refract_dir) = refract(ray.direction, normal, eta) {
            let r = sampler.get_1d();
            let reflect_prob = schlick(-ray.direction.dot(normal).min(1.0), eta);
            if r < reflect_prob {
                let reflected = reflect(ray.direction, normal);
                let scattered = Ray::new(hit.p, reflected);
                return Some((scattered, attenuation));
            } else {
                let scattered = Ray::new(hit.p, refract_dir);
                return Some((scattered, attenuation));
            }
        } else {
            let reflected = reflect(ray.direction, normal);
            let scattered = Ray::new(hit.p, reflected);
            return Some((scattered, attenuation));
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_schlick() {
        let cosine = 0.5;
        let eta = 1.5;
        let r = schlick(cosine, eta);
        assert_eq!(r, 0.776393202250021);
    }

}