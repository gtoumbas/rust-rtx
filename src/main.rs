use std::io::{self, Write};
use std::fs::File;

mod vec3;
mod ray;
mod hittable;

use vec3::Vec3;
use ray::Ray;

fn hit_sphere(center: Vec3, radius: f32, r: &Ray) -> f32 {
    let oc = *r.orig() - center;
    let a = r.dir().len_squared();
    let half_b = oc.dot(r.dir());
    let c = oc.len_squared() - radius * radius;
    let discriminant = half_b * half_b - a * c;
    if discriminant < 0.0 {
        return -1.0;
    }
    (-half_b - discriminant.sqrt()) / a
}

pub fn ray_color(r: &Ray) -> Vec3 {
    let mut t = hit_sphere(Vec3::new(0.0, 0.0, -1.0), 0.5, r);
    if t > 0.0 {
        let N = (r.at(t) - Vec3::new(0.0, 0.0, -1.0)).unit_vector();
        return Vec3::new(N.x()*0.5 + 1.0, N.y() + 1.0, N.z() + 1.0);
    }
    let unit_dir = r.dir().unit_vector();
    t = 0.5 * (unit_dir.y() + 1.0);
    let white = Vec3::new(1.0, 1.0, 1.0);
    let blue = Vec3::new(0.5, 0.7, 1.0);
    let white_shade = white * (1.0 - t) + blue * t;
    white_shade
}

fn main() {
    
    // Image 
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;
    let image_height = (image_width as f32 / aspect_ratio) as i32;

    // Camera
    let viewport_height = 2.0;
    let viewport_width = aspect_ratio * viewport_height;
    let focal_length = 1.0;

    // Camera Origin
    let origin = Vec3::new(0.0, 0.0, 0.0);
    let horizontal = Vec3::new(viewport_width, 0.0, 0.0);
    let vertical = Vec3::new(0.0, viewport_height, 0.0);
    let lower_left_corner = origin - horizontal/2.0 - vertical/2.0 - Vec3::new(0.0, 0.0, focal_length);


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
            let u = x as f32 / image_width as f32;
            let v = y as f32 / image_height as f32;
            let r = Ray::new(origin, lower_left_corner + horizontal * u + vertical * v - origin);
            let pixel_color = ray_color(&r);
            pixel_color.write_color(&mut out);
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
