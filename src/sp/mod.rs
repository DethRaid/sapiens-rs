pub mod common;
pub mod math;
pub mod noise;
pub mod particles;
pub mod rand;

// Sapiens Math Library
//
// This is a _very_ thin wrapper around Sapiens' vector math API. I made a mode idiomatic wrapper but found that
// converting to/from Sapiens FFI types was just too cumbersome. Usage of this API is recommended for most mods,
// especially ones which don't perform much vector math

use sapiens_sys::*;

pub type SPQuat = SPVec4;

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

/// Adds two vectors together
pub fn vec2_add(a: SPVec2, b: SPVec2) -> SPVec2 {
    unsafe { spVec2Add(a, b) }
}

/// Subtracts b from a
pub fn vec2_sub(a: SPVec2, b: SPVec2) -> SPVec2 {
    unsafe { spVec2Sub(a, b) }
}

/// Multiplies a by b
pub fn vec2_mul(a: SPVec2, b: f64) -> SPVec2 {
    unsafe { spVec2Mul(a, b) }
}

/// Divides a by b
pub fn vec2_div(a: SPVec2, b: f64) -> SPVec2 {
    unsafe { spVec2Div(a, b) }
}

/// Negates a
pub fn vec2_neg(a: SPVec2) -> SPVec2 {
    unsafe { spVec2Neg(a) }
}

/// Adds a to be
pub fn vec3_add(a: SPVec3, b: SPVec3) -> SPVec3 {
    unsafe { spVec3Add(a, b) }
}

/// Subtracts b from a
pub fn vec3_sub(a: SPVec3, b: SPVec3) -> SPVec3 {
    unsafe { spVec3Sub(a, b) }
}

/// Multiplies a by b
pub fn vec3_mul(a: SPVec3, b: f64) -> SPVec3 {
    unsafe { spVec3Mul(a, b) }
}

/// Divides a by b
pub fn vec3_div(a: SPVec3, b: f64) -> SPVec3 {
    unsafe { spVec3Div(a, b) }
}

/// Negates a
pub fn vec3_neg(a: SPVec3) -> SPVec3 {
    unsafe { spVec3Neg(a) }
}

/// Adds a to b
pub fn vec4_add(a: SPVec4, b: SPVec4) -> SPVec4 {
    unsafe { spVec4Add(a, b) }
}

/// Subtracts b from a
pub fn vec4_sub(a: SPVec4, b: SPVec4) -> SPVec4 {
    unsafe { spVec4Sub(a, b) }
}

/// Multiplies a by b
pub fn vec4_mul(a: SPVec4, b: f64) -> SPVec4 {
    unsafe { spVec4Mul(a, b) }
}

/// Divides every component of a by b
pub fn vec4_div(a: SPVec4, b: f64) -> SPVec4 {
    unsafe { spVec4Div(a, b) }
}

/// Negates a
pub fn vec4_neg(a: SPVec4) -> SPVec4 {
    unsafe { spVec4Neg(a) }
}

/// Normalizes a
pub fn vec3_normalize(a: SPVec3) -> SPVec3 {
    unsafe { spVec3Normalize(a) }
}

/// Calculates the dot product of a and b
pub fn vec3_dot(a: SPVec3, b: SPVec3) -> f64 {
    unsafe { spVec3Dot(a, b) }
}

/// Calculates the cross product of a and b
pub fn vec3_cross(a: SPVec3, b: SPVec3) -> SPVec3 {
    unsafe { spVec3Cross(a, b) }
}

/// Calculates the length of a vector
pub fn vec3_length(v: SPVec3) -> f64 {
    unsafe { spVec3Length(v) }
}

/// Calculates the squared length of a vector
///
/// This function avoids a square root and thus is faster in many algorithms
pub fn vec3_length2(v: SPVec3) -> f64 {
    unsafe { spVec3Length2(v) }
}

/// Calculates the distance between two points
pub fn vec3_distance(a: SPVec3, b: SPVec3) -> f64 {
    unsafe { spVec3Distance(a, b) }
}

/// Calculates the squared distance between two points
///
/// This function avoids a square root, and thus is faster in many algorithms
pub fn vec3_distance2(a: SPVec3, b: SPVec3) -> f64 {
    unsafe { spVec3Distance2(a, b) }
}

/// Multiplies v by m
pub fn vec3_x_mat3(v: SPVec3, m: SPMat3) -> SPVec3 {
    unsafe { spVec3xMat3(v, m) }
}

/// Casts a 3x3 matrix to a quaternion
pub fn quat_cast(m: *mut SPMat3) -> SPQuat {
    unsafe { spQuatCast(m) }
}

/// Casts a quaternion to a 3x3 matrix
pub fn mat3_cast(q: *mut SPQuat, m: *mut SPMat3) {
    unsafe { spMat3Cast(q, m) }
}

/// Spherically interpolates between two quaternions, effectively producing a rotation some percentage of the way
/// between a and b
pub fn quat_slerp(a: SPQuat, b: SPQuat, x: f64) -> SPQuat {
    unsafe { spQuatSlerp(a, b, x) }
}

/// Gets a 3x3 identity matrix
pub fn mat3_identity() -> SPMat3 {
    unsafe { spMat3Identity() }
}

/// Encodes a rotate of `angle` (degrees or radians) around axis `axis` into matrix `m`
pub fn mat3_rotate(m: SPMat3, angle: f64, axis: SPVec3) -> SPMat3 {
    unsafe { spMat3Rotate(m, angle, axis) }
}

/// Calculates the inverse of matrix `m`
pub fn mat3_inverse(m: SPMat3) -> SPMat3 {
    unsafe { spMat3Inverse(m) }
}

/// Multiplies a by b
pub fn mat3_multiply(a: SPMat3, b: SPMat3) -> SPMat3 {
    unsafe { spMat3Multiply(a, b) }
}

/// Spherically interpolates between a and b, effectively returning a rotation that's `fraction` percent of the way
/// between `a` and `b`
pub fn mat3_slerp(a: SPMat3, b: SPMat3, fraction: f64) -> SPMat3 {
    unsafe { spMat3Slerp(a, b, fraction) }
}

/// Retrieves a row from a matrix
pub fn mat3_get_row(m: SPMat3, row_index: i32) -> SPVec3 {
    unsafe { spMat3GetRow(m, row_index) }
}

/// Constructs a 3x3 matrix from its rows
pub fn mat3_from_vec3s(a: SPVec3, b: SPVec3, c: SPVec3) -> SPMat3 {
    unsafe { spMat3FromVec3s(a, b, c) }
}

/// Calculates the inverse of a matrix which would look in the direction `look`, using `up` as its up vector
pub fn mat3_look_at_inverse(look: SPVec3, up: SPVec3) -> SPMat3 {
    unsafe { spMat3LookAtInverse(look, up) }
}
