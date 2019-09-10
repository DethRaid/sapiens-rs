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

pub fn vec2_add(a: SPVec2, b: SPVec2) -> SPVec2 {
    unsafe { spVec2Add(a, b) }
}

pub fn vec2_sub(a: SPVec2, b: SPVec2) -> SPVec2 {
    unsafe { spVec2Sub(a, b) }
}

pub fn vec2_mul(a: SPVec2, b: f64) -> SPVec2 {
    unsafe { spVec2Mul(a, b) }
}

pub fn vec2_div(a: SPVec2, b: f64) -> SPVec2 {
    unsafe { spVec2Div(a, b) }
}

pub fn vec2_neg(a: SPVec2) -> SPVec2 {
    unsafe { spVec2Neg(a) }
}

pub fn vec3_add(a: SPVec3, b: SPVec3) -> SPVec3 {
    unsafe { spVec3Add(a, b) }
}

pub fn vec3_sub(a: SPVec3, b: SPVec3) -> SPVec3 {
    unsafe { spVec3Sub(a, b) }
}

pub fn vec3_mul(a: SPVec3, b: f64) -> SPVec3 {
    unsafe { spVec3Mul(a, b) }
}

pub fn vec3_div(a: SPVec3, b: f64) -> SPVec3 {
    unsafe { spVec3Div(a, b) }
}

pub fn vec3_neg(a: SPVec3) -> SPVec3 {
    unsafe { spVec3Neg(a) }
}

pub fn vec4_add(a: SPVec4, b: SPVec4) -> SPVec4 {
    unsafe { spVec4Add(a, b) }
}

pub fn vec4_sub(a: SPVec4, b: SPVec4) -> SPVec4 {
    unsafe { spVec4Sub(a, b) }
}

pub fn vec4_mul(a: SPVec4, b: f64) -> SPVec4 {
    unsafe { spVec4Mul(a, b) }
}

pub fn vec4_div(a: SPVec4, b: f64) -> SPVec4 {
    unsafe { spVec4Div(a, b) }
}

pub fn vec4_neg(a: SPVec4) -> SPVec4 {
    unsafe { spVec4Neg(a) }
}

pub fn vec3_normalize(a: SPVec3) -> SPVec3 {
    unsafe { spVec3Normalize(a) }
}

pub fn vec3_dot(a: SPVec3, b: SPVec3) -> f64 {
    unsafe { spVec3Dot(a, b) }
}

pub fn vec3_cross(a: SPVec3, b: SPVec3) -> SPVec3 {
    unsafe { spVec3Cross(a, b) }
}

pub fn vec3_length(a: SPVec3) -> f64 {
    unsafe { spVec3Length(a) }
}

pub fn vec3_length2(a: SPVec3) -> f64 {
    unsafe { spVec3Length2(a) }
}

pub fn vec3_distance(a: SPVec3, b: SPVec3) -> f64 {
    unsafe { spVec3Distance(a, b) }
}

pub fn vec3_distance2(a: SPVec3, b: SPVec3) -> f64 {
    unsafe { spVec3Distance2(a, b) }
}

pub fn vec3_x_mat3(v: SPVec3, m: SPMat3) -> SPVec3 {
    unsafe { spVec3xMat(v, m) }
}
