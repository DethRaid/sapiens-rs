use sapiens_sys::*;
use std::ops::{Add, Deref, Div, Mul, Neg, Sub};

#[derive(Debug)]
pub struct Vec2(SPVec2);

#[derive(Debug)]
pub struct Vec3(SPVec3);

#[derive(Debug)]
pub struct Vec4(SPVec4);

pub type Quat = Vec4;

#[derive(Debug)]
pub struct Mat3(SPMat3);

#[derive(Debug)]
pub struct Mat4(SPMat4);

impl Vec2 {
    pub fn new(x: f64, y: f64) -> Self {
        Vec2(SPVec2 { x, y })
    }

    pub fn as_sp_vec(&self) -> SPVec2 {
        self.0
    }
}

impl From<SPVec2> for Vec2 {
    fn from(sp_vec: SPVec2) -> Self {
        Vec2(sp_vec)
    }
}

impl Deref for Vec2 {
    type Target = SPVec2;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Default for Vec2 {
    fn default() -> Self {
        Vec2::new(0.0, 0.0)
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

#[allow(clippy::len_without_is_empty)]
impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Vec3(SPVec3 { x, y, z })
    }

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

    /// Checks if this Vec3 is to the left of the line from a to b
    pub fn is_left_of_line(&self, a: &Vec3, b: &Vec3) -> bool {
        unsafe { spPointIsLeftOfLine(self.0, a.0, b.0) }
    }

    pub fn as_sp_vec(&self) -> SPVec3 {
        self.0
    }
}

impl Deref for Vec3 {
    type Target = SPVec3;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Default for Vec3 {
    fn default() -> Self {
        Vec3::new(0.0, 0.0, 0.0)
    }
}

impl From<SPVec3> for Vec3 {
    fn from(sp_vec: SPVec3) -> Self {
        Vec3(sp_vec)
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
    pub fn new(x: f64, y: f64, z: f64, w: f64) -> Self {
        Vec4(SPVec4 { x, y, z, w })
    }

    pub fn as_sp_vec(&self) -> SPVec4 {
        self.0
    }
}

impl Default for Vec4 {
    fn default() -> Self {
        Vec4::new(0.0, 0.0, 0.0, 0.0)
    }
}

impl From<SPVec4> for Vec4 {
    fn from(sp_vec: SPVec4) -> Self {
        Vec4(sp_vec)
    }
}

impl Deref for Vec4 {
    type Target = SPVec4;

    fn deref(&self) -> &Self::Target {
        &self.0
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

impl Quat {
    /// Spherical linear interpolation between two quaternions
    ///
    /// aka interpolates between two rotations
    pub fn slerp(a: &Quat, b: &Quat, x: f64) -> Quat {
        Vec4(unsafe { spQuatSlerp(a.0, b.0, x) })
    }
}

impl Mat3 {
    /// Constructs a new Mat4 from the provided rows
    ///
    /// TODO: Verify that this is what Sapiens expects
    pub fn new(row0: &Vec3, row1: &Vec3, row2: &Vec3) -> Self {
        Mat3(unsafe { spMat3FromVec3s(row0.0, row1.0, row2.0) })
    }

    pub fn identity() -> Self {
        Mat3(unsafe { spMat3Identity() })
    }

    pub fn look_at_inverse(look: &Vec3, up: &Vec3) -> Mat3 {
        Mat3(unsafe { spMat3LookAtInverse(look.0, up.0) })
    }

    /// Spherical interpolation between two Mat3
    ///
    /// AKA interpolates between two rotation matrices
    pub fn slerp(lhs: &Mat3, rhs: &Mat3, fraction: f64) -> Mat3 {
        Mat3(unsafe { spMat3Slerp(lhs.0, rhs.0, fraction) })
    }

    pub fn as_sp_mat(&self) -> SPMat3 {
        self.0
    }

    /// Rotates this matrix by some amount around an axis, and returns the result
    ///
    /// This method doesn't modify the matrix itself
    pub fn rotate(&self, angle: f64, axis: &Vec3) -> Mat3 {
        Mat3(unsafe { spMat3Rotate(self.0, angle, axis.0) })
    }

    /// calculates the inverse of this matrix
    pub fn inverse(&self) -> Mat3 {
        Mat3(unsafe { spMat3Inverse(self.0) })
    }

    pub fn get_row(&self, row: u32) -> Vec3 {
        Vec3::from(unsafe { spMat3GetRow(self.0, row as i32) })
    }
}

impl Deref for Mat3 {
    type Target = SPMat3;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Default for Mat3 {
    /// Creates an 3x3 identity matrix
    fn default() -> Self {
        Mat3::identity()
    }
}

impl From<SPMat3> for Mat3 {
    fn from(sp_map: SPMat3) -> Self {
        Mat3(sp_map)
    }
}

impl From<&mut Quat> for Mat3 {
    fn from(quat: &mut Quat) -> Self {
        let mut mat: Mat3 = Default::default();

        unsafe { spMat3Cast(&mut quat.0, &mut mat.0) }

        mat
    }
}

impl Into<Quat> for Mat3 {
    fn into(mut self) -> Quat {
        Vec4(unsafe { spQuatCast(&mut self.0) })
    }
}

impl Mul for Mat3 {
    type Output = Mat3;

    fn mul(self, rhs: Self) -> Self::Output {
        Mat3(unsafe { spMat3Multiply(self.0, rhs.0) })
    }
}

impl Mat4 {
    /// Constructs a new Mat4 from the provided rows
    ///
    /// TODO: Verify that this is what Sapiens expects
    pub fn new(row0: &Vec4, row1: &Vec4, row2: &Vec4, row3: &Vec4) -> Self {
        Mat4(SPMat4 {
            m0: row0.0.x,
            m1: row0.0.y,
            m2: row0.0.z,
            m3: row0.0.w,
            m4: row1.0.x,
            m5: row1.0.y,
            m6: row1.0.z,
            m7: row1.0.w,
            m8: row2.0.x,
            m9: row2.0.y,
            m10: row2.0.z,
            m11: row2.0.w,
            m12: row3.0.x,
            m13: row3.0.y,
            m14: row3.0.z,
            m15: row3.0.w,
        })
    }

    pub fn as_sp_mat(&self) -> SPMat4 {
        self.0
    }
}

impl Deref for Mat4 {
    type Target = SPMat4;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Default for Mat4 {
    /// Creates a 4x4 identity matrix
    fn default() -> Self {
        Mat4::new(
            &Vec4::new(1.0, 0.0, 0.0, 0.0),
            &Vec4::new(0.0, 1.0, 0.0, 0.0),
            &Vec4::new(0.0, 0.0, 1.0, 0.0),
            &Vec4::new(0.0, 0.0, 0.0, 1.0),
        )
    }
}

impl From<SPMat4> for Mat4 {
    fn from(sp_mat: SPMat4) -> Self {
        Mat4(sp_mat)
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
