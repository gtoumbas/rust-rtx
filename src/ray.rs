// use vec3::Vec3;

use crate::vec3::Vec3;

pub struct Ray{
    orig: Vec3,
    dir: Vec3,
}

impl Ray{

    pub fn new(orig: Vec3, dir: Vec3) -> Ray{
        Ray{orig: orig, dir: dir}
    }

    pub fn orig (&self) -> Vec3{
        self.orig
    }

    pub fn dir (&self) -> Vec3{
        self.dir
    }

    pub fn at(self, t: f32) -> Vec3 {
        self.orig + self.dir * t
    }
}

pub fn ray_color(r: &Ray) -> Vec3 {
    let unit_dir = r.dir.unit_vector();
    let t = 0.5 * (unit_dir.y() + 1.0);
    let white = Vec3::new(1.0, 1.0, 1.0);
    let blue = Vec3::new(0.5, 0.7, 1.0);
    let white_shade = white * (1.0 - t) + blue * t;
    white_shade
}