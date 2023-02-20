use glam::DVec3;

use crate::accel::BVH;
use crate::camera::Camera;
use crate::integrator::Integrator;
use crate::sampler::Sampler;
use crate::scene::Scene;
pub struct Renderer {
    pub width: usize,
    pub height: usize,
    pub buffer: Vec<DVec3>,
    pub samples: usize,
    pub depth: i32,
}

impl Renderer {
    pub fn new(width: usize, height: usize, samples: usize, depth:i32) -> Renderer {
        Renderer { width, height, buffer: vec![DVec3::ZERO; width * height], samples, depth }
    }

    pub fn render(&mut self, camera: &dyn Camera, scene: &Scene, sampler: &mut dyn Sampler, integrator: &dyn Integrator) {
        let bvh = BVH::new(&scene.objects);

        for y in 0..self.height {
            for x in 0..self.width {
                let mut color = DVec3::ZERO;
                for _ in 0..self.samples {
                    let u = (x as f64 + rand::random::<f64>()) / self.width as f64 * 2.0 - 1.0;
                    let v = 1.0 - (y as f64 + rand::random::<f64>()) / self.height as f64 * 2.0;
                    let ray = camera.get_ray(u, v);
                    // println!("ray: {:?}", ray);
                    color  += integrator.li(&ray, &bvh, sampler, self.depth);
                    // color += (ray.direction + DVec3::ONE) * 0.5;
                }

                color /= self.samples as f64;
                color.x = color.x.clamp(0.0, 1.0);
                color.y = color.y.clamp(0.0, 1.0);
                color.z = color.z.clamp(0.0, 1.0);
                
                self.buffer[y * self.width + x] = color;
            }
        }
    }


    pub fn save(&self, filename: &str) {
        let mut image = image::RgbImage::new(self.width as u32, self.height as u32);
        for y in 0..self.height {
            for x in 0..self.width {
                let color = self.buffer[y * self.width + x];
                // println!("color: {:?}", color);
                let r = (color.x * 255.0) as u8;
                let g = (color.y * 255.0) as u8;
                let b = (color.z * 255.0) as u8;
                image.put_pixel(x as u32, y as u32, image::Rgb([r, g, b]));
            }
        }
        image.save(filename).unwrap();
    }

    // pub fn save(&self, filename: &str) {
    //     let mut image = image::RgbImage::new(self.width as u32, self.height as u32);
    //     for y in 0..self.height {
    //         for x in 0..self.width {
    //             let color = self.buffer[y * self.width + x];
    //             let r = (color.x * 255.0) as u8;
    //             let g = (color.y * 255.0) as u8;
    //             let b = (color.z * 255.0) as u8;
    //             image.put_pixel(x as u32, y as u32, image::Rgb([r, g, b]));
    //         }
    //     }
    //     image.save(filename).unwrap();
    // }
}