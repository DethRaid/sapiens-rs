#![allow(non_snake_case)]

mod sp;
mod usermod;

extern crate num_derive;

use sapiens_sys::{SPParticleEmitterTypeInfo, SPParticleRenderGroupInfo};
use std::convert::TryInto;
use std::os::raw::c_int;
use std::{mem, ops};

#[no_mangle]
#[cfg(feature = "particle")]
pub extern "C" fn spGetEmitterTypesCount() -> c_int {
    usermod::particles::get_emitter_types().len() as c_int
}

#[no_mangle]
#[cfg(feature = "particle")]
pub extern "C" fn spGetEmitterTypes() -> *mut SPParticleEmitterTypeInfo {
    let mut emitters: Vec<SPParticleEmitterTypeInfo> = usermod::particles::get_emitter_types()
        .drain(ops::RangeFull)
        .map(|emitter_type| TryInto::try_into(emitter_type).unwrap())
        .collect();

    emitters.shrink_to_fit();
    let ptr = emitters.as_mut_ptr();
    mem::forget(emitters);

    ptr
}

#[no_mangle]
#[cfg(feature = "particle")]
pub extern "C" fn spGetRenderGroupTypesCount() -> c_int {
    usermod::particles::get_render_group_types_count() as c_int
}

#[no_mangle]
#[cfg(feature = "particle")]
pub extern "C" fn spGetRenderGroupTypes() -> *mut SPParticleRenderGroupInfo {
    unimplemented!();
}
