use std::io::{self, Write};
use std::fs::File;
use std::rc::Rc;
use rand;

mod vec3;
mod ray;
mod hittable;
mod camera;
mod material;

use vec3::{Vec3, rand_unit_vector};
use ray::Ray;
use hittable:: {HittableList, HitRecord, Sphere, Hittable};
use camera::Camera;
use material::{Lambertian, Metal};


// FIXME not sure if world should be a HittableList
pub fn ray_color(r: &Ray, world: &HittableList, depth: i32) -> Vec3 {
    if depth <= 0 {
        return Vec3::new_empty();
    }

    let mut rec = HitRecord::new_empty();

    if world.hit(r, 0.001, std::f32::INFINITY, &mut rec) {
        let mut scattered = Ray::new_empty();
        let mut attenuation = Vec3::new_empty();

        if rec.mat_ptr().scatter(r, &rec, &mut attenuation, &mut scattered) {
            return attenuation * ray_color(&scattered, world, depth - 1);
        }
        return Vec3::new_empty();
    }
    let unit_direction = r.dir().unit_vector();
    let t = 0.5 * (unit_direction.y() + 1.0);
    Vec3::new(1.0, 1.0, 1.0) *  (1.0 - t) + Vec3::new(0.5, 0.7, 1.0) * t
        
}

fn main() {
    
    // Image 
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;
    let image_height = (image_width as f32 / aspect_ratio) as i32;
    let samples_per_pixel = 100;
    let max_depth = 50;

    // World 
    let mut world = HittableList::new();

    let material_ground = Lambertian::new(Vec3::new(0.8, 0.8, 0.0));
    let material_center = Lambertian::new(Vec3::new(0.7, 0.3, 0.3));
    let material_right = Metal::new(Vec3::new(0.8, 0.6, 0.2));
    let material_left = Metal::new(Vec3::new(0.8, 0.8, 0.8));

    // make &Lambertian into Rc<dyn Material>
    let material_ground_ptr = Rc::new(material_ground);
    let material_center_ptr = Rc::new(material_center);
    let material_right_ptr = Rc::new(material_right);
    let material_left_ptr = Rc::new(material_left);
    
    // This is ugly 
    world.add(Box::new(Sphere::new(Vec3::new(0.0, -100.5, -1.0), 100.0, material_ground_ptr)));
    world.add(Box::new(Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5, material_center_ptr)));
    world.add(Box::new(Sphere::new(Vec3::new(-1.0, 0.0, -1.0), 0.5, material_left_ptr)));
    world.add(Box::new(Sphere::new(Vec3::new(1.0, 0.0, -1.0), 0.5, material_right_ptr)));



    // Camera
    let cam: Camera = Camera::new();


    // Rendering 

    // Create a new image buffer
    let stdout = io::stdout();
    let mut stdout_handle = stdout.lock();
    let mut out = File::create("out.ppm").unwrap();

    // Write the header
    out.write_all(b"P3\n").unwrap();
    out.write_all(format!("{} {}\n", image_width, image_height).as_bytes()).unwrap();
    out.write_all(b"255\n").unwrap();

    // Write the image data
    for y in (0..image_height).rev() {
        // Progress bar sent to stdout
        stdout_handle.write_all(format!("\rScanlines Remaining {}", y).as_bytes()).unwrap();
        for x in 0..image_width {
            let mut pixel_color = Vec3::new(0.0, 0.0, 0.0);
            for _ in 0..samples_per_pixel {
                let u = (x as f32 + rand::random::<f32>()) / (image_width as f32);
                let v = (y as f32 + rand::random::<f32>()) / (image_height as f32);
                let r = cam.get_ray(u, v);
                pixel_color = pixel_color + ray_color(&r, &world, max_depth);
            }
            pixel_color.write_color(&mut out, samples_per_pixel);
        }
    }
}
