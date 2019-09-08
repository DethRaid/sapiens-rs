use crate::sp::math::Vec3;
use sapiens_sys::{spNoiseDelete, spNoiseGet, spNoiseNew, SPNoise};

pub struct Noise(*mut SPNoise);

impl Noise {
    /// Creates a new Noise with the provided parameters
    ///
    /// # Parameters
    ///
    /// * `seed` - Noise seed
    /// * `persistance` -
    pub fn new(seed: i32, persistance: f64) -> Self {
        unsafe { Noise(spNoiseNew(seed, persistance)) }
    }

    pub fn delete(self) {
        unsafe { spNoiseDelete(self.as_sp_noise()) };
    }

    /// Evaluates this Noise at the given position, using end_octave octaves
    ///
    /// # Parameters
    ///
    /// * `pos` - The position to evaluate the noise function at
    /// * `end_octave` - The maximum noise octave to evaluate. Higher numbers give higher-frequency noise at the cost of
    ///     performance
    pub fn get(&self, pos: &Vec3, end_octave: i32) -> f64 {
        unsafe { spNoiseGet(self.as_sp_noise(), pos.as_sp_vec(), end_octave) }
    }

    pub fn as_sp_noise(&self) -> *mut SPNoise {
        self.0
    }
}
