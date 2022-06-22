// vec3 struct with functions to add, subtract, multiply, divide, and scale
use std::fs::File;
use std::io::{Write};
use std::ops:: {Add, Sub, Mul, Div};

#[derive(Debug)]
pub struct Vec3 {
    x: f32,
    y: f32,
    z: f32,
}

impl Vec3 {

    pub fn new(x: f32, y: f32, z: f32) -> Vec3 {
        Vec3 { x: x, y: y, z: z }
    }

    
    pub fn x (&self) -> f32 {
        self.x
    }

    pub fn y (&self) -> f32 {
        self.y
    }

    pub fn z (&self) -> f32 {
        self.z
    }

    pub fn len_squared(&self) -> f32 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    pub fn len(&self) -> f32 {
        (self.len_squared()).sqrt()
    }

    pub fn dot(&self, other: &Vec3) -> f32 {
        self.x() * other.x() + self.y() * other.y() + self.z() * other.z()
    }

    // Function to calculate the cross product of two vectors
    pub fn cross(&self, other: &Vec3) -> Vec3 {
        Vec3::new( self.y() * other.z() - self.z() * other.y(),
                self.z() * other.x() - self.x() * other.z(),
                self.x() * other.y() - self.y() * other.x())
    }

    // calc unit vector 
    pub fn unit_vector(&self) -> Vec3 {
        let l = self.len();
        Vec3::new(self.x() / l, self.y() / l, self.z() / l)
    }

    // function to write the Vec3 to a ppm file
    pub fn write_color(&self, out: &mut File) {
        let r = (self.x() * 255.999) as f32;
        let g = (self.y() * 255.999) as f32;
        let b = (self.z() * 255.999) as f32;
        out.write_all(format!("{} {} {}\n", r, g, b).as_bytes()).unwrap();
    }
}

// Overloading +, -, *, / operators for Vec3

impl Add for Vec3 {
    type Output = Vec3;

    fn add(self, other: Vec3) -> Vec3 {
        Vec3::new(self.x() + other.x(), self.y() + other.y(), self.z() + other.z())
    }
}

impl Sub for Vec3 {
    type Output = Vec3;

    fn sub(self, other: Vec3) -> Vec3 {
        Vec3::new(self.x() - other.x(), self.y() - other.y(), self.z() - other.z())
    }
}

impl Mul for Vec3 {
    type Output = Vec3;

    fn mul(self, other: Vec3) -> Vec3 {
        Vec3::new(self.x() * other.x(), self.y() * other.y(), self.z() * other.z())
    }
}

impl Mul<f32> for Vec3 {
    type Output = Vec3;

    fn mul(self, scaler: f32) -> Vec3 {
        Vec3::new(self.x() * scaler, self.y() * scaler, self.z() * scaler)
    }
}

impl Div for Vec3 {
    type Output = Vec3;

    fn div(self, other: Vec3) -> Vec3 {
        Vec3::new(self.x() / other.x(), self.y() / other.y(), self.z() / other.z())
    }
}

impl Div<f32> for Vec3 {
    type Output = Vec3;

    fn div(self, scaler: f32) -> Vec3 {
        self * (1.0 / scaler)
    }
}

// Implement clone 
impl Clone for Vec3 {
    fn clone(&self) -> Vec3 {
        Vec3::new(self.x(), self.y(), self.z())
    }
}

// Implement copy
impl Copy for Vec3 {}