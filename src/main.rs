use std::io::{self, Write};
use std::fs::File;
use std::rand;

mod vec3;
mod ray;
mod hittable;
mod camera;

use vec3::Vec3;
use ray::Ray;
use hittable:: {HittableList, HitRecord, Sphere, Hittable};
use camera::Camera;

// fn hit_sphere(center: Vec3, radius: f32, r: &Ray) -> f32 {
//     let oc = *r.orig() - center;
//     let a = r.dir().len_squared();
//     let half_b = oc.dot(r.dir());
//     let c = oc.len_squared() - radius * radius;
//     let discriminant = half_b * half_b - a * c;
//     if discriminant < 0.0 {
//         return -1.0;
//     }
//     (-half_b - discriminant.sqrt()) / a
// }

// FIXME not sure if world should be a HittableList
pub fn ray_color(r: &Ray, world: &HittableList) -> Vec3 {
    let mut rec = HitRecord::new(0.0, Vec3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 0.0, 0.0), false);

    if world.hit(r, 0.001, std::f32::INFINITY, &mut rec) {
        return (rec.normal() + Vec3::new(1.0, 1.0, 1.0)) * 0.5;
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

    // World 
    let mut world = HittableList::new();
    world.add(Box::new(Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5)));
    world.add(Box::new(Sphere::new(Vec3::new(0.0, -100.5, -1.0), 100.0)));



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
                pixel_color = pixel_color + ray_color(&r, &world);
            }
            pixel_color.write_color(&mut out, samples_per_pixel);
        }
    }


    // let vec1 = Vec3::new(1.0, 6.0, -3.0);
    // let vec2 = Vec3::new(-1.0, -1.0, -1.0);
    // let vec3 = vec1 + vec2;
    // let vec4 = vec1 - vec2;
    // let vec5 = vec1 * vec2;
    // let vec6 = vec1 / vec2;
    // let vec7 = vec1.dot(&vec2);
    // let vec8 = vec1.cross(&vec2);
    // let vec9 = vec1.unit_vector();

    // println!("{:?}", vec3);
    // println!("{:?}", vec4);
    // println!("{:?}", vec5);
    // println!("{:?}", vec6);
    // println!("{:?}", vec7);
    // println!("{:?}", vec8);
    // println!("{:?}", vec9);
    

    // // print len
    // println!("{}", vec1.len());
    // println!("{}", vec1.len_squared());



}
