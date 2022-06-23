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

    pub fn orig (&self) -> &Vec3{
        &self.orig
    }

    pub fn dir (&self) -> &Vec3{
        &self.dir
    }

    pub fn at(self, t: f32) -> Vec3 {
        self.orig + self.dir * t
    }
}

impl Clone for Ray {
    fn clone(&self) -> Ray {
        Ray{orig: self.orig, dir: self.dir}
    }
}

impl Copy for Ray {}


