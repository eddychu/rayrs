use std::rc::Rc;

use glam::{ DVec3};
use rayrs::{renderer::Renderer, camera::{PerspectiveCamera}, integrator::TestIntegrator, sampler::{self, RandomSampler, Sampler}, scene::Scene, material::{Lambertian, Material, Metal, Dielectric}, hittable::{Sphere, Hittable}, object::Object};

fn main() {
    let width = 400;
    let height = 300;
    let samples = 50;
    let depth = 50;
    let mut renderer = Renderer::new(width, height, samples, depth);
    let camera = PerspectiveCamera::new(
        glam::DVec3::new(13.0, 3.0, 3.0),
        glam::DVec3::new(0.0, 0.0, 0.0),
        glam::DVec3::new(0.0, 1.0, 0.0),
        30.0,
        width as f64 / height as f64,
    );

    let integrator = TestIntegrator::new();
    let mut sampler = RandomSampler::new();
    let mut scene = Scene::new();

    let ground_material = Rc::new(Box::new(Lambertian::new(glam::DVec3::new(0.5, 0.5, 0.5))) as Box<dyn Material>);
    scene.add(Object::new(Rc::new(Box::new(Sphere::new(glam::DVec3::new(0.0, -1000.0, 0.0), 1000.0)) as Box<dyn Hittable>), ground_material));

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = sampler.get_1d();
            let center = glam::DVec3::new(a as f64 + 0.9 * sampler.get_1d(), 0.2, b as f64 + 0.9 * sampler.get_1d());

            if (center - DVec3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                let sphere_material: Rc<Box<dyn Material>>;
                if choose_mat < 0.8 {
                    let albedo = DVec3::new(sampler.get_1d(), sampler.get_1d(), sampler.get_1d()) * DVec3::new(sampler.get_1d(), sampler.get_1d(), sampler.get_1d());
                    sphere_material = Rc::new(Box::new(Lambertian::new(albedo)) as Box<dyn Material>);
                    scene.add(Object::new(Rc::new(Box::new(Sphere::new(center, 0.2)) as Box<dyn Hittable>), sphere_material));
                } else if choose_mat < 0.95 {
                    let albedo = DVec3::new(1.0, 1.0, 1.0) * sampler.get_1d();
                    let fuzz = sampler.get_1d() * 0.5;
                    sphere_material = Rc::new(Box::new(Metal::new(albedo, fuzz)) as Box<dyn Material>);
                    scene.add(Object::new(Rc::new(Box::new(Sphere::new(center, 0.2)) as Box<dyn Hittable>), sphere_material));
                } else {
                    sphere_material = Rc::new(Box::new(Dielectric::new(1.5)) as Box<dyn Material>);
                    scene.add(Object::new(Rc::new(Box::new(Sphere::new(center, 0.2)) as Box<dyn Hittable>), sphere_material));
                }
            }
        }
    }

    let material1 = Rc::new(Box::new(Dielectric::new(1.5)) as Box<dyn Material>);
    scene.add(Object::new(Rc::new(Box::new(Sphere::new(glam::DVec3::new(0.0, 1.0, 0.0), 1.0)) as Box<dyn Hittable>), material1));

    let material2 = Rc::new(Box::new(Lambertian::new(glam::DVec3::new(0.4, 0.2, 0.1))) as Box<dyn Material>);
    scene.add(Object::new(Rc::new(Box::new(Sphere::new(glam::DVec3::new(-4.0, 1.0, 0.0), 1.0)) as Box<dyn Hittable>), material2));

    let material3 = Rc::new(Box::new(Metal::new(glam::DVec3::new(0.7, 0.6, 0.5), 0.0)) as Box<dyn Material>);
    scene.add(Object::new(Rc::new(Box::new(Sphere::new(glam::DVec3::new(4.0, 1.0, 0.0), 1.0)) as Box<dyn Hittable>), material3));

    renderer.render(&camera, &scene, &integrator);

    renderer.save("second.png");
}