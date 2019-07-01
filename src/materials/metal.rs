use crate::util::random_in_unit_sphere;
use crate::{HitRecord, Ray, Vector};

use super::Material;

#[derive(Debug, Copy, Clone)]
pub struct Metal {
    albedo: Vector,
    fuzz: f32,
}

impl Metal {
    pub fn new(r: f32, g: f32, b: f32, fuzz: f32) -> Self {
        Self {
            albedo: Vector::new(r, g, b),
            fuzz,
        }
    }
}

fn reflect(v: Vector, n: Vector) -> Vector {
    v - 2.0 * Vector::dot(v, n) * n
}

impl Material for Metal {
    fn scatter(&self, r_in: Ray, hit_record: HitRecord) -> (bool, Vector, Ray) {
        let reflected = reflect(r_in.dir().normalize(), hit_record.normal);
        let scattered = Ray::new(
            hit_record.p,
            reflected + self.fuzz * random_in_unit_sphere(),
        );

        return (
            Vector::dot(scattered.dir(), hit_record.normal) > 0.0,
            self.albedo,
            scattered,
        );
    }
}
