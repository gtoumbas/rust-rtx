use std::rc::Rc;

use crate::ray::Ray;
use crate::vec3::Vec3;
use crate::material::{Material, Lambertian};

pub struct HitRecord {
    t: f32,
    p: Vec3,
    mat_ptr: Rc<dyn Material>, 
    normal: Vec3,
    front_face: bool,
}

impl HitRecord {

    pub fn new(t: f32, p: Vec3, normal: Vec3, front_face: bool, mat_ptr: Rc<dyn Material>) -> HitRecord {
        HitRecord {
            t,
            p,
            mat_ptr,
            normal,
            front_face,
        }
    }

    pub fn new_empty() -> HitRecord {
        HitRecord {
            t: 0.0,
            p: Vec3::new(0.0, 0.0, 0.0),
            mat_ptr: Rc::new(Lambertian::new(Vec3::new_empty())),
            normal: Vec3::new(0.0, 0.0, 0.0),
            front_face: false,
        }
    }

    pub fn set_face_normal(&mut self, r: &Ray, outward_normal: Vec3) {
        self.front_face = r.dir().dot(&outward_normal) < 0.0;
        self.normal = if self.front_face {outward_normal} else {outward_normal * -1.0};
    }

    pub fn normal(&self) -> Vec3 {
        self.normal
    }

    pub fn mat_ptr(&self) -> &dyn Material {
        &*self.mat_ptr
    }

    pub fn t(&self) -> f32 {
        self.t
    }

    pub fn p(&self) -> Vec3 {
        self.p
    }

    pub fn front_face(&self) -> bool {
        self.front_face
    }
}


// Not sure if this is the best way to do this. May be better to use enum
pub trait Hittable {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32, rec: &mut HitRecord) -> bool;
}


pub struct Sphere {
    center: Vec3,
    radius: f32,
    mat_ptr: Rc<dyn Material>,
}

impl Sphere {
    pub fn new(center: Vec3, radius: f32, mat_ptr: Rc<dyn Material>) -> Sphere {
        Sphere {
            center,
            radius,
            mat_ptr,
        }
    }
}

impl Hittable for Sphere {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32, rec: &mut HitRecord) -> bool {
        let oc = *r.orig() - self.center;
        let a = r.dir().len_squared();
        let half_b = oc.dot(r.dir());
        let c = oc.len_squared() - self.radius * self.radius;
        let discriminant = half_b * half_b - a * c;
        if discriminant < 0.0 {
            return false;
        }

        let sqrtd = discriminant.sqrt();
        
        // Finding root
        let mut root = (-half_b - sqrtd) / a;
        if root < t_min || t_max < root {
            root = (-half_b + sqrtd) / a;
            if root < t_min || t_max < root {
                return false;
            }
        }

        rec.t = root;
        rec.p = r.at(rec.t);
        let outward_normal = (rec.p - self.center) / self.radius;
        rec.set_face_normal(r, outward_normal);
        rec.mat_ptr = self.mat_ptr.clone(); 

        true
    }
}




pub struct HittableList {
    objects: Vec<Box<dyn Hittable>>,
}

impl HittableList {
    pub fn new() -> HittableList {
        HittableList{objects: Vec::new()}
    }

    pub fn add(&mut self, object: Box<dyn Hittable>) {
        self.objects.push(object);
    }

    pub fn clear(&mut self) {
        self.objects.clear();
    }
}

// TODO don't think I need a temp_rec here 
impl Hittable for HittableList {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32, rec: &mut HitRecord) -> bool {
        let mut hit_anything = false;
        let mut closest_so_far = t_max;
        for object in &self.objects {
            if object.hit(r, t_min, closest_so_far, rec) {
                hit_anything = true;
                closest_so_far = rec.t;
            }
        }
        hit_anything
    }
}




