pub mod common;
pub mod math;
pub mod noise;
pub mod particles;
pub mod rand;

use sapiens_sys::*;

#[macro_export]
macro_rules! sp_meters_to_prerender {
    ($val:expr) => {
        (($val) / 8388608.0)
    };
}

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
pub fn vec2_add(a: &SPVec2, b: &SPVec2) -> SPVec2 {
    unsafe { spVec2Add(*a, *b) }
}

/// Subtracts b from a
pub fn vec2_sub(a: &SPVec2, b: &SPVec2) -> SPVec2 {
    unsafe { spVec2Sub(*a, *b) }
}

/// Multiplies a by b
pub fn vec2_mul(a: &SPVec2, b: f64) -> SPVec2 {
    unsafe { spVec2Mul(*a, b) }
}

/// Divides a by b
pub fn vec2_div(a: &SPVec2, b: f64) -> SPVec2 {
    unsafe { spVec2Div(*a, b) }
}

/// Negates a
pub fn vec2_neg(a: &SPVec2) -> SPVec2 {
    unsafe { spVec2Neg(*a) }
}

/// Adds a to be
pub fn vec3_add(a: &SPVec3, b: &SPVec3) -> SPVec3 {
    unsafe { spVec3Add(*a, *b) }
}

/// Subtracts b from a
pub fn vec3_sub(a: &SPVec3, b: &SPVec3) -> SPVec3 {
    unsafe { spVec3Sub(*a, *b) }
}

/// Multiplies a by b
pub fn vec3_mul(a: &SPVec3, b: f64) -> SPVec3 {
    unsafe { spVec3Mul(*a, b) }
}

/// Divides a by b
pub fn vec3_div(a: &SPVec3, b: f64) -> SPVec3 {
    unsafe { spVec3Div(*a, b) }
}

/// Negates a
pub fn vec3_neg(a: &SPVec3) -> SPVec3 {
    unsafe { spVec3Neg(*a) }
}

/// Adds a to b
pub fn vec4_add(a: &SPVec4, b: &SPVec4) -> SPVec4 {
    unsafe { spVec4Add(*a, *b) }
}

/// Subtracts b from a
pub fn vec4_sub(a: &SPVec4, b: &SPVec4) -> SPVec4 {
    unsafe { spVec4Sub(*a, *b) }
}

/// Multiplies a by b
pub fn vec4_mul(a: &SPVec4, b: f64) -> SPVec4 {
    unsafe { spVec4Mul(*a, b) }
}

/// Divides every component of a by b
pub fn vec4_div(a: &SPVec4, b: f64) -> SPVec4 {
    unsafe { spVec4Div(*a, b) }
}

/// Negates a
pub fn vec4_neg(a: &SPVec4) -> SPVec4 {
    unsafe { spVec4Neg(*a) }
}

/// Normalizes a
pub fn vec3_normalize(a: &SPVec3) -> SPVec3 {
    unsafe { spVec3Normalize(*a) }
}

/// Calculates the dot product of a and b
pub fn vec3_dot(a: &SPVec3, b: &SPVec3) -> f64 {
    unsafe { spVec3Dot(*a, *b) }
}

/// Calculates the cross product of a and b
pub fn vec3_cross(a: &SPVec3, b: &SPVec3) -> SPVec3 {
    unsafe { spVec3Cross(*a, *b) }
}

/// Calculates the length of a vector
pub fn vec3_length(v: &SPVec3) -> f64 {
    unsafe { spVec3Length(*v) }
}

/// Calculates the squared length of a vector
///
/// This function avoids a square root and thus is faster in many algorithms
pub fn vec3_length2(v: &SPVec3) -> f64 {
    unsafe { spVec3Length2(*v) }
}

/// Calculates the distance between two points
pub fn vec3_distance(a: &SPVec3, b: &SPVec3) -> f64 {
    unsafe { spVec3Distance(*a, *b) }
}

/// Calculates the squared distance between two points
///
/// This function avoids a square root, and thus is faster in many algorithms
pub fn vec3_distance2(a: &SPVec3, b: &SPVec3) -> f64 {
    unsafe { spVec3Distance2(*a, *b) }
}

/// Multiplies v by m
pub fn vec3_x_mat3(v: &SPVec3, m: &SPMat3) -> SPVec3 {
    unsafe { spVec3xMat3(*v, *m) }
}

/// Casts a 3x3 matrix to a quaternion
#[allow(clippy::not_unsafe_ptr_arg_deref)]
pub fn quat_cast(m: *mut SPMat3) -> SPQuat {
    unsafe { spQuatCast(m) }
}

/// Casts a quaternion to a 3x3 matrix
#[allow(clippy::not_unsafe_ptr_arg_deref)]
pub fn mat3_cast(q: *mut SPQuat, m: *mut SPMat3) {
    unsafe { spMat3Cast(q, m) }
}

/// Spherically interpolates between two quaternions, effectively producing a rotation some percentage of the way
/// between a and b
pub fn quat_slerp(a: &SPQuat, b: &SPQuat, x: f64) -> SPQuat {
    unsafe { spQuatSlerp(*a, *b, x) }
}

/// Gets a 3x3 identity matrix
pub fn mat3_identity() -> SPMat3 {
    unsafe { spMat3Identity() }
}

/// Encodes a rotate of `angle` (degrees or radians) around axis `axis` into matrix `m`
pub fn mat3_rotate(m: &SPMat3, angle: f64, axis: &SPVec3) -> SPMat3 {
    unsafe { spMat3Rotate(*m, angle, *axis) }
}

/// Calculates the inverse of matrix `m`
pub fn mat3_inverse(m: &SPMat3) -> SPMat3 {
    unsafe { spMat3Inverse(*m) }
}

/// Multiplies a by b
pub fn mat3_multiply(a: &SPMat3, b: &SPMat3) -> SPMat3 {
    unsafe { spMat3Multiply(*a, *b) }
}

/// Spherically interpolates between a and b, effectively returning a rotation that's `fraction` percent of the way
/// between `a` and `b`
pub fn mat3_slerp(a: &SPMat3, b: &SPMat3, fraction: f64) -> SPMat3 {
    unsafe { spMat3Slerp(*a, *b, fraction) }
}

/// Retrieves a row from a matrix
pub fn mat3_get_row(m: &SPMat3, row_index: i32) -> SPVec3 {
    unsafe { spMat3GetRow(*m, row_index) }
}

/// Constructs a 3x3 matrix from its rows
pub fn mat3_from_vec3s(a: &SPVec3, b: &SPVec3, c: SPVec3) -> SPMat3 {
    unsafe { spMat3FromVec3s(*a, *b, c) }
}

/// Calculates the inverse of a matrix which would look in the direction `look`, using `up` as its up vector
pub fn mat3_look_at_inverse(look: &SPVec3, up: &SPVec3) -> SPMat3 {
    unsafe { spMat3LookAtInverse(*look, *up) }
}

/// Rotates matrix `m` `mangle` (degrees|radians) around `axis`, storing the result in `result`
#[allow(clippy::not_unsafe_ptr_arg_deref)]
pub fn mat3_rotate_ptr(m: *mut SPMat3, angle: f64, axis: *mut SPVec3, result: *mut SPMat3) {
    unsafe { spMat3RotatePtr(m, angle, axis, result) }
}

/// Computes the inverse of a matrix to look in direction `look` using `up` as the up direction, and stores the result
/// in `result`
#[allow(clippy::not_unsafe_ptr_arg_deref)]
pub fn mat3_look_at_inverse_ptr(look: *mut SPVec3, up: *mut SPVec3, result: *mut SPMat3) {
    unsafe { spMat3LookAtInversePtr(look, up, result) }
}

/// Calculates the inverse of `m`, storing the result in `result`
#[allow(clippy::not_unsafe_ptr_arg_deref)]
pub fn mat3_inverse_ptr(m: *mut SPMat3, result: *mut SPMat3) {
    unsafe { spMat3InversePtr(m, result) }
}

/// Spherically interpolates between `a` and `b` by `fraction` amount, storing the result in `result`
#[allow(clippy::not_unsafe_ptr_arg_deref)]
pub fn mat3_slerp_ptr(a: *mut SPMat3, b: *mut SPMat3, fraction: f64, result: *mut SPMat3) {
    unsafe { spMat3SlerpPtr(a, b, fraction, result) }
}

/// Checks if point `p` is to the left of the line that passes through `a` before it passes through `b`
pub fn point_is_left_of_line(p: &SPVec3, a: &SPVec3, b: &SPVec3) -> bool {
    unsafe { spPointIsLeftOfLine(*p, *a, *b) }
}

// Smoke tests to ensure I'm calling the correct Sapiens functions

#[cfg(test)]
mod math_smoke_tests {
    use super::*;

    #[test]
    fn min_test() {
        let result = min(3.0, 4.0);

        assert_eq!(result, 3.0);
    }

    #[test]
    fn max_test() {
        let result = max(3.0, 4.0);

        assert_eq!(result, 4.0);
    }

    #[test]
    fn clamp_test() {
        let result = clamp(3.0, 0.0, 1.0);

        assert_eq!(result, 1.0);
    }

    #[test]
    fn mix_test() {
        let result = mix(4.0, 8.0, 0.75);

        assert_eq!(result, 7.0);
    }

    #[test]
    fn smooth_step_test() {
        let result = smooth_step(1.0, 4.0, 0.8);

        assert_eq!(result, 3.5);
    }
}

#[cfg(test)]
mod vec2_smoke_tests {
    use super::*;

    #[test]
    fn vec2_add_test() {
        let a = SPVec2 { x: 2.0, y: 2.0 };
        let b = SPVec2 { x: 3.0, y: 4.0 };

        let result = vec2_add(&a, &b);

        assert_eq!(result, SPVec2 { x: 5.0, y: 6.0 });
    }

    #[test]
    fn vec2_sub_test() {
        let a = SPVec2 { x: 2.0, y: 2.0 };
        let b = SPVec2 { x: 3.0, y: 4.0 };

        let result = vec2_sub(&a, &b);

        assert_eq!(result, SPVec2 { x: -1.0, y: -2.0 });
    }

    #[test]
    fn vec2_mul_test() {
        let a = SPVec2 { x: 2.0, y: 3.0 };

        let result = vec2_mul(&a, 4.0);

        assert_eq!(result, SPVec2 { x: 8.0, y: 12.0 });
    }

    #[test]
    fn vec2_div_test() {
        let a = SPVec2 { x: 2.0, y: 3.0 };

        let result = vec2_div(&a, 2.0);

        assert_eq!(result, SPVec2 { x: 1.0, y: 1.5 })
    }

    #[test]
    fn vec2_neg_test() {
        let a = SPVec2 { x: 2.0, y: 3.0 };

        let result = vec2_neg(&a);

        assert_eq!(result, SPVec2 { x: -2.0, y: -3.0 })
    }
}

#[cfg(test)]
mod vec3_smoke_tests {
    use super::*;

    #[test]
    fn vec3_add_test() {
        let a = SPVec3 {
            x: 2.0,
            y: 2.0,
            z: 2.0,
        };
        let b = SPVec3 {
            x: 3.0,
            y: 4.0,
            z: 5.0,
        };

        let result = vec3_add(&a, &b);

        assert_eq!(
            result,
            SPVec3 {
                x: 5.0,
                y: 6.0,
                z: 7.0
            }
        );
    }

    #[test]
    fn vec3_sub_test() {
        let a = SPVec3 {
            x: 2.0,
            y: 2.0,
            z: 2.0,
        };
        let b = SPVec3 {
            x: 3.0,
            y: 4.0,
            z: 5.0,
        };

        let result = vec3_sub(&a, &b);

        assert_eq!(
            result,
            SPVec3 {
                x: -1.0,
                y: -2.0,
                z: -3.0
            }
        );
    }

    #[test]
    fn vec3_mul_test() {
        let a = SPVec3 {
            x: 2.0,
            y: 3.0,
            z: 5.0,
        };

        let result = vec3_mul(&a, 4.0);

        assert_eq!(
            result,
            SPVec3 {
                x: 8.0,
                y: 12.0,
                z: 20.0
            }
        );
    }

    #[test]
    fn vec3_div_test() {
        let a = SPVec3 {
            x: 2.0,
            y: 3.0,
            z: 5.0,
        };

        let result = vec3_div(&a, 2.0);

        assert_eq!(
            result,
            SPVec3 {
                x: 1.0,
                y: 1.5,
                z: 2.5
            }
        )
    }

    #[test]
    fn vec3_neg_test() {
        let a = SPVec3 {
            x: 2.0,
            y: 3.0,
            z: 5.0,
        };

        let result = vec3_neg(&a);

        assert_eq!(
            result,
            SPVec3 {
                x: -2.0,
                y: -3.0,
                z: -5.0
            }
        )
    }
}

#[cfg(test)]
mod vec4_smoke_tests {
    use super::*;

    #[test]
    fn vec4_add_test() {
        let a = SPVec4 {
            x: 2.0,
            y: 2.0,
            z: 2.0,
            w: 2.0,
        };
        let b = SPVec4 {
            x: 3.0,
            y: 4.0,
            z: 5.0,
            w: 6.0,
        };

        let result = vec4_add(&a, &b);

        assert_eq!(
            result,
            SPVec4 {
                x: 5.0,
                y: 6.0,
                z: 7.0,
                w: 8.0
            }
        );
    }

    #[test]
    fn vec4_sub_test() {
        let a = SPVec4 {
            x: 2.0,
            y: 2.0,
            z: 2.0,
            w: 2.0,
        };
        let b = SPVec4 {
            x: 3.0,
            y: 4.0,
            z: 5.0,
            w: 6.0,
        };

        let result = vec4_sub(&a, &b);

        assert_eq!(
            result,
            SPVec4 {
                x: -1.0,
                y: -2.0,
                z: -3.0,
                w: -4.0
            }
        );
    }

    #[test]
    fn vec4_mul_test() {
        let a = SPVec4 {
            x: 2.0,
            y: 3.0,
            z: 5.0,
            w: 6.0,
        };

        let result = vec4_mul(&a, 4.0);

        assert_eq!(
            result,
            SPVec4 {
                x: 8.0,
                y: 12.0,
                z: 20.0,
                w: 24.0
            }
        );
    }

    #[test]
    fn vec4_div_test() {
        let a = SPVec4 {
            x: 2.0,
            y: 3.0,
            z: 5.0,
            w: 6.0,
        };

        let result = vec4_div(&a, 2.0);

        assert_eq!(
            result,
            SPVec4 {
                x: 1.0,
                y: 1.5,
                z: 2.5,
                w: 3.0
            }
        )
    }

    #[test]
    fn vec4_neg_test() {
        let a = SPVec4 {
            x: 2.0,
            y: 3.0,
            z: 5.0,
            w: 6.0,
        };

        let result = vec4_neg(&a);

        assert_eq!(
            result,
            SPVec4 {
                x: -2.0,
                y: -3.0,
                z: -5.0,
                w: -6.0
            }
        )
    }
}

#[cfg(test)]
mod linear_algebra_smoke_tests {
    use super::*;

    #[test]
    fn vec3_normalize_test() {
        let vec = SPVec3 {
            x: 2.0,
            y: 2.0,
            z: 2.0,
        };

        let normalized_vec = vec3_normalize(&vec);

        assert_eq!(
            normalized_vec,
            SPVec3 {
                x: 0.707,
                y: 0.707,
                z: 0.707
            }
        );
    }

    #[test]
    fn vec3_dot_test() {
        let a = SPVec3 {
            x: 1.0,
            y: 0.0,
            z: 0.0,
        };
        let b = SPVec3 {
            x: 0.0,
            y: 1.0,
            z: 0.0,
        };

        let dot = vec3_dot(&a, &b);

        assert_eq!(dot, 0.707);
    }

    #[test]
    fn vec3_cross_test() {
        let a = SPVec3 {
            x: 1.0,
            y: 0.0,
            z: 0.0,
        };
        let b = SPVec3 {
            x: 0.0,
            y: 1.0,
            z: 0.0,
        };

        let cross = vec3_cross(&a, &b);

        assert_eq!(
            cross,
            SPVec3 {
                x: 0.0,
                y: 0.0,
                z: 1.0
            }
        );
    }

    #[test]
    fn vec3_length_test() {
        let vec = SPVec3 {
            x: 2.0,
            y: 2.0,
            z: 2.0,
        };

        let len = vec3_length(&vec);

        assert_eq!(len, 4.0);
    }

    #[test]
    fn vec3_length2_test() {
        let vec = SPVec3 {
            x: 2.0,
            y: 2.0,
            z: 2.0,
        };

        let len = vec3_length2(&vec);

        assert_eq!(len, 16.0);
    }

    #[test]
    fn vec3_distance_test() {
        let a = SPVec3 {
            x: 1.0,
            y: 2.0,
            z: 3.0,
        };
        let b = SPVec3 {
            x: 3.0,
            y: 2.0,
            z: 1.0,
        };

        let distance = vec3_distance(&a, &b);

        assert_eq!(distance, 3.0);
    }

    #[test]
    fn vec3_distance2_test() {
        let a = SPVec3 {
            x: 1.0,
            y: 2.0,
            z: 3.0,
        };
        let b = SPVec3 {
            x: 3.0,
            y: 2.0,
            z: 1.0,
        };

        let distance = vec3_distance2(&a, &b);

        assert_eq!(distance, 9.0);
    }

    // I don't wanna do the mat3 functions wahhhhhhhhhhhhhhhhhhh
}
