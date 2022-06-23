// use vec3 and ray structs 

use crate::ray::Ray;
use crate::vec3::Vec3;

struct HitRecord {
    t: f32,
    p: Vec3,
    normal: Vec3,
}

impl HitRecord {
    fn new(t: f32, p: Vec3, normal: Vec3) -> HitRecord {
        HitRecord { t, p, normal }
    }
}


struct Sphere {
    center: Vec3,
    radius: f32,
}

impl Sphere {
    pub fn new(center: Vec3, radius: f32) -> Sphere {
        Sphere{center: center, radius: radius}
    }

    pub fn hit(&self, r: &Ray, t_min: f32, t_max: f32, rec: &mut HitRecord) -> bool {
        let oc = *r.orig() - self.center;
        let a = r.dir().len_squared();
        let half_b = oc.dot(r.dir());
        let c = oc.len_squared() - self.radius * self.radius;
        let discriminant = half_b * half_b - a * c;
        if discriminant < 0.0 {
            return false;
        }

        let sqrtd = discriminant.sqrt();
        
        let mut root = (-half_b - sqrtd) / a;
        if root < t_min || t_max < root {
            root = (-half_b + sqrtd) / a;
            if root < t_min || t_max < root {
                return false;
            }
        }

        rec.t = root;
        rec.p = r.at(rec.t);
        rec.normal = (rec.p - self.center) / self.radius;

        true
    }
    




}


