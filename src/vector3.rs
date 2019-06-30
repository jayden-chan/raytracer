use std::ops;

use crate::util::fast_inv_sqrt;
use rand::prelude::*;

#[derive(Debug)]
pub struct Vector {
    x: f32,
    y: f32,
    z: f32,
}

/// Constructor-like
impl Vector {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }

    pub fn unit() -> Self {
        Self {
            x: 1.0,
            y: 1.0,
            z: 1.0,
        }
    }

    pub fn rand() -> Self {
        Self {
            x: random::<f32>(),
            y: random::<f32>(),
            z: random::<f32>(),
        }
    }
}

/// Vector math implementations
impl Vector {
    /// Set the length of the vector to 1, maintaining its direction
    pub fn normalize(self) -> Self {
        let mag_inv = fast_inv_sqrt(
            (self.x * self.x) + (self.y * self.y) + (self.z * self.z),
        );

        Self {
            x: self.x * mag_inv,
            y: self.y * mag_inv,
            z: self.z * mag_inv,
        }
    }

    /// Compute the dot product of two vectors
    pub fn dot(lhs: Vector, rhs: Vector) -> f32 {
        lhs.x * rhs.x + lhs.y * rhs.y + lhs.z * rhs.z
    }

    /// Compute the cross product of two vectors
    pub fn cross(lhs: Vector, rhs: Vector) -> Self {
        Self {
            x: lhs.y * rhs.z - lhs.z * rhs.y,
            y: -lhs.x * rhs.z - lhs.z * rhs.x,
            z: lhs.x * rhs.y - lhs.y * rhs.x,
        }
    }
}

/// Add two vectors together
impl ops::Add<Vector> for Vector {
    type Output = Vector;

    fn add(self, rhs: Vector) -> Vector {
        Vector {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

/// Subtract two vectors
impl ops::Sub<Vector> for Vector {
    type Output = Vector;

    fn sub(self, rhs: Vector) -> Vector {
        Vector {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

/// Multiply each element of a vector by a float
impl ops::Mul<f32> for Vector {
    type Output = Vector;

    fn mul(self, rhs: f32) -> Vector {
        Vector {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

/// Multiply-asign each element of a vector by a float
impl ops::MulAssign<f32> for Vector {
    fn mul_assign(&mut self, rhs: f32) {
        self.x *= rhs;
        self.y *= rhs;
        self.z *= rhs;
    }
}
