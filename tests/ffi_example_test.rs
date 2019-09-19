use num_derive::{FromPrimitive, ToPrimitive};
use sapiens_rs::sp::particles::EmitterTypeInfo;
use sapiens_sys::SPParticleEmitterTypeInfo;
use std::convert::TryInto;

#[allow(non_snake_case)]
pub extern "C" fn spGetEmitterTypes() -> *mut SPParticleEmitterTypeInfo {
    let mut emitters: Vec<SPParticleEmitterTypeInfo> = get_emitter_types()
        .drain(::std::ops::RangeFull)
        .map(|emitter_type| emitter_type.try_into().unwrap())
        .collect();

    emitters.shrink_to_fit();
    let ptr = emitters.as_mut_ptr();
    ::std::mem::forget(emitters);

    ptr
}

#[derive(Debug, PartialEq, FromPrimitive, ToPrimitive)]
enum VanillaEmitterType {
    Campfire = 0,
    WoodChop = 1,
    Feathers = 2,
}

fn get_emitter_types() -> Vec<EmitterTypeInfo<VanillaEmitterType>> {
    vec![
        EmitterTypeInfo {
            name: "campfire".to_string(),
            id: VanillaEmitterType::Campfire,
        },
        EmitterTypeInfo {
            name: "woodChop".to_string(),
            id: VanillaEmitterType::WoodChop,
        },
        EmitterTypeInfo {
            name: "feathers".to_string(),
            id: VanillaEmitterType::Feathers,
        },
    ]
}
