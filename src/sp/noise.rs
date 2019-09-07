use sapiens_sys::{SPNoise, spNoiseNew, spNoiseDelete, spNoiseGet};
use crate::sp::math::Vec3;

pub struct Noise {
    pub noise: *mut SPNoise,
}

impl Noise {
    fn new(seed: i32, persistance: f64) -> Self {
        unsafe {
            let noise = spNoiseNew(seed, persistance);

            Noise{noise}
        }
    }

    /// Basically a destructor. After you call this method on a Noise, you can't do anything else with the Noise
    fn delete(&mut self) {
        unsafe { spNoiseDelete(self.noise) };
    }

    fn get(&mut self, vec: &mut Vec3, end_octave: i32) -> f64 {
        unsafe { spNoiseGet(self.noise, vec.vec, end_octave) }
    }
}