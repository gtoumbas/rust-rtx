use crate::hittable::HitRecord;
use crate::ray::Ray;
use crate::vec3::{Vec3, rand_unit_vector, reflect};


pub trait Material {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord, attenuation: &mut Vec3, scattered: &mut Ray) -> bool;
}

pub struct Lambertian {
    albedo: Vec3,
}

impl Lambertian {
    pub fn new(albedo: Vec3) -> Lambertian {
        Lambertian { albedo }
    }
}

impl Material for Lambertian {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord, attenuation: &mut Vec3, scattered: &mut Ray) -> bool {
        let mut scatter_dir = rec.normal() + rand_unit_vector();

        // Catch if uv is opposite of normal
        if scatter_dir.near_zero() { scatter_dir = rec.normal() }

        *scattered = Ray::new(rec.p(), scatter_dir);
        *attenuation = self.albedo;
        true
    }
}


pub struct Metal {
    albedo: Vec3,
}

impl Metal {
    pub fn new(albedo: Vec3) -> Metal {
        Metal { albedo }
    }
}

impl Material for Metal {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord, attenuation: &mut Vec3, scattered: &mut Ray) -> bool {
        let reflected = reflect(r_in.dir().unit_vector(), rec.normal());
        *scattered = Ray::new(rec.p(), reflected);
        *attenuation = self.albedo;
        scattered.dir().dot(&rec.normal()) > 0.0
    }
}