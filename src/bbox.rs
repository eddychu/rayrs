use glam::DVec3;

use crate::ray::Ray;

#[derive(Debug, Clone, Copy)]
pub struct BBox {
    pub min: DVec3,
    pub max: DVec3,
}

impl Default for BBox {
    fn default() -> Self {
        Self {
            min: DVec3::new(f64::INFINITY, f64::INFINITY, f64::INFINITY),
            max: DVec3::new(f64::NEG_INFINITY, f64::NEG_INFINITY, f64::NEG_INFINITY),
        }
    }
}

impl BBox {
    pub fn new(min: DVec3, max: DVec3) -> Self {
        Self { min, max }
    }

    pub fn union(&self, other: &Self) -> Self {
        Self {
            min: DVec3::new(
                self.min.x.min(other.min.x),
                self.min.y.min(other.min.y),
                self.min.z.min(other.min.z),
            ),
            max: DVec3::new(
                self.max.x.max(other.max.x),
                self.max.y.max(other.max.y),
                self.max.z.max(other.max.z),
            ),
        }
    }

    pub fn intersect(&self, other: &Self) -> bool {
        self.min.x <= other.max.x
            && self.max.x >= other.min.x
            && self.min.y <= other.max.y
            && self.max.y >= other.min.y
            && self.min.z <= other.max.z
            && self.max.z >= other.min.z
    }

    pub fn contains(&self, point: &DVec3) -> bool {
        self.min.x <= point.x
            && self.max.x >= point.x
            && self.min.y <= point.y
            && self.max.y >= point.y
            && self.min.z <= point.z
            && self.max.z >= point.z
    }

    pub fn center(&self) -> DVec3 {
        (self.min + self.max) * 0.5
    }

    pub fn diagonal(&self) -> DVec3 {
        self.max - self.min
    }

    pub fn surface_area(&self) -> f64 {
        let d = self.diagonal();
        2.0 * (d.x * d.y + d.x * d.z + d.y * d.z)
    }

    pub fn volume(&self) -> f64 {
        let d = self.diagonal();
        d.x * d.y * d.z
    }

    pub fn max_extent(&self) -> usize {
        let d = self.diagonal();
        if d.x > d.y && d.x > d.z {
            0
        } else if d.y > d.z {
            1
        } else {
            2
        }
    }

    pub fn hit(&self, ray: &Ray) -> bool {
        let mut t0 = ray.min_t;
        let mut t1 = ray.max_t;
        for a in 0..3 {
            let inv_d = 1.0 / ray.direction[a];
            let mut t_near = (self.min[a] - ray.origin[a]) * inv_d;
            let mut t_far = (self.max[a] - ray.origin[a]) * inv_d;
            if inv_d < 0.0 {
                std::mem::swap(&mut t_near, &mut t_far);
            }
            t0 = t_near.max(t0);
            t1 = t_far.min(t1);
            if t0 >= t1 {
                return false;
            }
        }
        true
    }

}
