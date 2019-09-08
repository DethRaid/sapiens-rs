use sapiens_sys::*;
use std::ops::{Add, Div, Mul, Neg, Sub};

#[derive(Debug)]
pub struct Vec2(SPVec2);

pub struct Vec3(SPVec3);

pub struct Vec4(SPVec4);

pub struct Mat3(SPMat3);

pub struct Mat4(SPMat4);

impl Vec2 {
    pub fn new(x: f64, y: f64) -> Self {
        Vec2(SPVec2 { x, y })
    }

    pub fn as_sp_vec(&self) -> SPVec2 {
        self.0
    }
}

impl Add for Vec2 {
    type Output = Vec2;

    fn add(self, rhs: Self) -> Self::Output {
        Vec2(unsafe { spVec2Add(self.0, rhs.0) })
    }
}

impl Sub for Vec2 {
    type Output = Vec2;

    fn sub(self, rhs: Self) -> Self::Output {
        Vec2(unsafe { spVec2Sub(self.0, rhs.0) })
    }
}

impl Mul<f64> for Vec2 {
    type Output = Vec2;

    fn mul(self, rhs: f64) -> Self::Output {
        Vec2(unsafe { spVec2Mul(self.0, rhs) })
    }
}

impl Div<f64> for Vec2 {
    type Output = Vec2;

    fn div(self, rhs: f64) -> Self::Output {
        Vec2(unsafe { spVec2Div(self.0, rhs) })
    }
}

impl Neg for Vec2 {
    type Output = Vec2;

    fn neg(self) -> Self::Output {
        Vec2(unsafe { spVec2Neg(self.0) })
    }
}

impl Vec3 {
    /// Normalizes the provided vector
    pub fn normalize(vec: &Vec3) -> Vec3 {
        Vec3(unsafe { spVec3Normalize(vec.0) })
    }

    /// Calculates the dot product of two vectors
    pub fn dot(lhs: &Vec3, rhs: &Vec3) -> f64 {
        unsafe { spVec3Dot(lhs.0, rhs.0) }
    }

    /// Calculates the dot product of two vectors
    pub fn cross(lhs: &Vec3, rhs: &Vec3) -> Vec3 {
        Vec3(unsafe { spVec3Cross(lhs.0, rhs.0) })
    }

    /// Calculates the distance between two points
    pub fn distance(lhs: &Vec3, rhs: &Vec3) -> f64 {
        unsafe { spVec3Distance(lhs.0, rhs.0) }
    }

    /// Calculates the squared distance between two points
    ///
    /// This method avoid a square root and is thus faster in many algorithms
    pub fn squared_distance(lhs: &Vec3, rhs: &Vec3) -> f64 {
        unsafe { spVec3Distance2(lhs.0, rhs.0) }
    }

    /// Gets the length of this vector
    pub fn len(&self) -> f64 {
        unsafe { spVec3Length(self.0) }
    }

    /// Gets the squared length of this vector
    ///
    /// This method avoids a square root and is thus faster in many algorithms
    pub fn squared_len(&self) -> f64 {
        unsafe { spVec3Length2(self.0) }
    }

    pub fn as_sp_vec(&self) -> SPVec3 {
        self.0
    }
}

impl Add for Vec3 {
    type Output = Vec3;

    fn add(self, rhs: Self) -> Vec3 {
        Vec3(unsafe { spVec3Add(self.0, rhs.0) })
    }
}

impl Sub for Vec3 {
    type Output = Vec3;

    fn sub(self, rhs: Self) -> Vec3 {
        Vec3(unsafe { spVec3Sub(self.0, rhs.0) })
    }
}

impl Mul<f64> for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: f64) -> Self::Output {
        Vec3(unsafe { spVec3Mul(self.0, rhs) })
    }
}

impl Mul<Mat3> for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: Mat3) -> Self::Output {
        Vec3(unsafe { spVec3xMat3(self.0, rhs.0) })
    }
}

impl Div<f64> for Vec3 {
    type Output = Vec3;

    fn div(self, rhs: f64) -> Self::Output {
        Vec3(unsafe { spVec3Div(self.0, rhs) })
    }
}

impl Neg for Vec3 {
    type Output = Vec3;

    fn neg(self) -> Self::Output {
        Vec3(unsafe { spVec3Neg(self.0) })
    }
}

impl Vec4 {
    pub fn as_sp_vec(&self) -> SPVec4 {
        self.0
    }
}

impl Add for Vec4 {
    type Output = Vec4;

    fn add(self, rhs: Self) -> Vec4 {
        Vec4(unsafe { spVec4Add(self.0, rhs.0) })
    }
}

impl Sub for Vec4 {
    type Output = Vec4;

    fn sub(self, rhs: Self) -> Vec4 {
        Vec4(unsafe { spVec4Sub(self.0, rhs.0) })
    }
}

impl Mul<f64> for Vec4 {
    type Output = Vec4;

    fn mul(self, rhs: f64) -> Self::Output {
        Vec4(unsafe { spVec4Mul(self.0, rhs) })
    }
}

impl Div<f64> for Vec4 {
    type Output = Vec4;

    fn div(self, rhs: f64) -> Self::Output {
        Vec4(unsafe { spVec4Div(self.0, rhs) })
    }
}

impl Neg for Vec4 {
    type Output = Vec4;

    fn neg(self) -> Self::Output {
        Vec4(unsafe { spVec4Neg(self.0) })
    }
}

impl Mat3 {
    pub fn as_sp_mat(&self) -> SPMat3 {
        self.0
    }
}

impl Mat4 {
    pub fn as_sp_mat(&self) -> SPMat4 {
        self.0
    }
}

/// Calculates the minimum of two numbers
pub fn min(x: f64, y: f64) -> f64 {
    unsafe { spMin(x, y) }
}

/// Calculates the maximum of two numbers
pub fn max(x: f64, y: f64) -> f64 {
    unsafe { spMax(x, y) }
}

/// Clamps x between min and max
pub fn clamp(x: f64, min: f64, max: f64) -> f64 {
    unsafe { spClamp(x, min, max) }
}

/// Mixes x and y, using a as the mix factor
pub fn mix(x: f64, y: f64, a: f64) -> f64 {
    unsafe { spMix(x, y, a) }
}

/// Smooth steps x between edge0 and edge1
pub fn smooth_step(edge0: f64, edge1: f64, x: f64) -> f64 {
    unsafe { spSmoothStep(edge0, edge1, x) }
}
