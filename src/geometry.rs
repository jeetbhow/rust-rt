use std::{
    cmp::{Eq, PartialEq},
    ops::{Add, Div, Mul, Neg, Sub},
};

const EPSILON: f64 = 1e-6;

#[derive(Debug, Clone, Copy)]
pub struct Vector3(pub f64, pub f64, pub f64);

impl Vector3 {
    pub fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }

    pub fn length_squared(&self) -> f64 {
        self.0.powi(2) + self.1.powi(2) + self.2.powi(2)
    }

    pub fn normalize(&self) -> Vector3 {
        let len = self.length();
        Vector3(self.0 / len, self.1 / len, self.2 / len)
    }

    pub fn dot(v1: &Vector3, v2: &Vector3) -> f64 {
        v1.0 * v2.0 + v1.1 * v2.1 + v1.2 * v2.2
    }

    pub fn cross(v1: &Vector3, v2: &Vector3) -> Vector3 {
        Vector3(
            v1.1 * v2.2 - v1.2 * v2.1,
            -(v1.0 * v2.2 - v1.2 * v2.0),
            v1.0 * v2.1 - v1.1 * v2.0,
        )
    }
}

impl PartialEq for Vector3 {
    fn eq(&self, other: &Self) -> bool {
        (self.0 - other.0).abs() < EPSILON
            && (self.1 - other.1).abs() < EPSILON
            && (self.2 - other.2).abs() < EPSILON
    }
}

impl Eq for Vector3 {}

impl Neg for Vector3 {
    type Output = Vector3;

    fn neg(self) -> Vector3 {
        Vector3(-self.0, -self.1, -self.2)
    }
}

impl Add for Vector3 {
    type Output = Vector3;

    fn add(self, rhs: Self) -> Vector3 {
        Vector3(self.0 + rhs.0, self.1 + rhs.1, self.2 + rhs.2)
    }
}

impl Sub for Vector3 {
    type Output = Vector3;

    fn sub(self, rhs: Self) -> Vector3 {
        Vector3(self.0 - rhs.0, self.1 - rhs.1, self.2 - rhs.2)
    }
}

impl Mul<f64> for Vector3 {
    type Output = Vector3;

    fn mul(self, rhs: f64) -> Vector3 {
        Vector3(self.0 * rhs, self.1 * rhs, self.2 * rhs)
    }
}

impl Mul<Vector3> for f64 {
    type Output = Vector3;

    fn mul(self, rhs: Vector3) -> Vector3 {
        Vector3(self * rhs.0, self * rhs.1, self * rhs.2)
    }
}

impl Div<f64> for Vector3 {
    type Output = Vector3;

    fn div(self, rhs: f64) -> Vector3 {
        Vector3(self.0 / rhs, self.1 / rhs, self.2 / rhs)
    }
}

impl Div<Vector3> for f64 {
    type Output = Vector3;

    fn div(self, rhs: Vector3) -> Vector3 {
        Vector3(self / rhs.0, self / rhs.1, self / rhs.2)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Point3(pub f64, pub f64, pub f64);

impl PartialEq for Point3 {
    fn eq(&self, other: &Self) -> bool {
        (self.0 - other.0).abs() < EPSILON
            && (self.1 - other.1).abs() < EPSILON
            && (self.2 - other.2).abs() < EPSILON
    }
}

impl Eq for Point3 {}

impl Add<Vector3> for Point3 {
    type Output = Point3;

    fn add(self, rhs: Vector3) -> Point3 {
        Point3(self.0 + rhs.0, self.1 + rhs.1, self.2 + rhs.2)
    }
}

impl Sub for Point3 {
    type Output = Vector3;

    fn sub(self, rhs: Point3) -> Vector3 {
        Vector3(self.0 - rhs.0, self.1 - rhs.1, self.2 - rhs.2)
    }
}
