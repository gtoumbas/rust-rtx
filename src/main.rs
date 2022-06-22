use std::io::{self, Write};
use std::fs::File;

mod vec3;
mod ray;

use vec3::Vec3;
use ray::Ray;



fn main() {
    
    // Image dimensions 
    let width = 256;
    let height = 256;

    
    // Create a new image buffer
    let stdout = io::stdout();
    let mut stdout_handle = stdout.lock();

    let mut out = File::create("out.ppm").unwrap();

    // Write the header
    out.write_all(b"P3\n").unwrap();
    out.write_all(format!("{} {}\n", width, height).as_bytes()).unwrap();
    out.write_all(b"255\n").unwrap();

    // Write the image data
    for y in (0..height).rev() {
        // Progress bar sent to stdout
        stdout_handle.write_all(format!("\rScanlines Remaining {}", y).as_bytes()).unwrap();
        for x in 0..width {
            let r = x as f32 / width as f32;
            let g = y as f32 / height as f32;
            let b = 0.25;
            let pixel = Vec3::new(r, g, b);
            pixel.write_color(&mut out);
            
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
