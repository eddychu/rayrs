use std::rc::Rc;

use rayrs::{renderer::Renderer, camera::{Camera, PerspectiveCamera}, integrator::{self, TestIntegrator}, sampler::{self, RandomSampler}, scene::Scene, material::{Lambertian, Material, Metal, Dielectric}, hittable::{Sphere, Hittable}, object::Object};

fn main() {
    let width = 400;
    let height = 300;
    let samples = 100;
    let depth = 10;
    let mut renderer = Renderer::new(width, height, samples, depth);
    let camera = PerspectiveCamera::new(
        glam::DVec3::new(0.0, 0.0, 2.0),
        glam::DVec3::new(0.0, 0.0, -1.0),
        glam::DVec3::new(0.0, 1.0, 0.0),
        90.0,
        width as f64 / height as f64,
    );

    let integrator = TestIntegrator::new();
    let mut scene = Scene::new();

    let material_ground = Rc::new(Box::new(Lambertian::new(glam::DVec3::new(0.8, 0.8, 0.0))) as Box<dyn Material>) ;
    let material_center = Rc::new(Box::new(Lambertian::new(glam::DVec3::new(0.1, 0.2, 0.5))) as Box<dyn Material>);
    let material_left = Rc::new(Box::new(Dielectric::new(1.5)) as Box<dyn Material>);
    let material_right = Rc::new(Box::new(Metal::new(glam::DVec3::new(0.8, 0.6, 0.2), 0.0)) as Box<dyn Material>);

    let sphere1 = Rc::new(Box::new(Sphere::new(glam::DVec3::new(0.0, -100.5, -1.0), 100.0)) as Box<dyn Hittable>);
    let sphere2 = Rc::new(Box::new(Sphere::new(glam::DVec3::new(0.0,    0.0, -1.0), 0.5)) as Box<dyn Hittable>);
    let sphere3 = Rc::new(Box::new(Sphere::new(glam::DVec3::new(-1.0,   0.0, -1.0), 0.5)) as Box<dyn Hittable>);
    let sphere4 = Rc::new(Box::new(Sphere::new(glam::DVec3::new(1.0,    0.0, -1.0), 0.5)) as Box<dyn Hittable>);
    let object1 = Object::new(sphere1, material_ground);
    let object2 = Object::new(sphere2, material_center);
    let object3 = Object::new(sphere3, material_left);
    let object4 = Object::new(sphere4, material_right);
    scene.add(object1);
    scene.add(object2);
    scene.add(object3);
    scene.add(object4);

    renderer.render(&camera, &scene,  &integrator);

    renderer.save("first.png");
}