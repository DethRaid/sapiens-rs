use crate::sp::math::Vec3;
use sapiens_sys::{spRandDelete, spRandGetValue, spRandGetVec3, spRandNew, SPRand};

pub struct Rand(*mut SPRand);

impl Rand {
    /// Creates a new rng with the specified seed
    pub fn new(seed: u32) -> Self {
        Rand(unsafe { spRandNew(seed) })
    }

    pub fn from_ptr(ptr: *mut SPRand) -> Self {
        Rand(ptr)
    }

    /// Destroys the rng
    pub fn delete(self) {
        unsafe { spRandDelete(self.0) }
    }

    /// Gets a random Vec3 from this rng
    pub fn get_vec3(&self) -> Vec3 {
        Vec3::from(unsafe { spRandGetVec3(self.0) })
    }

    /// Gets a random float from this rng
    pub fn get_float(&self) -> f64 {
        unsafe { spRandGetValue(self.0) }
    }
}
