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
