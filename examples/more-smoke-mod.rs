// Cargo thinks that my extern "C" code is dead code, but it's wrong
#![allow(dead_code)]

extern crate sapiens_rs;
extern crate sapiens_sys;

extern crate num_derive;

use num_derive::FromPrimitive;
use num_traits::FromPrimitive;

use sapiens_sys::*;

use sapiens_rs::sp::{vec3_add, vec3_mul};

/// Sapiens calls this function once per particle to update that particle's position, velocity, and rendering
/// information
#[allow(non_snake_case)]
extern "C" fn spUpdateParticle(
    thread_state: *mut SPParticleThreadState,
    particle_state: *mut SPParticleState,
    local_render_group_type_id: u32,
    delta_time: f64,
    origin: SPVec3,
    render_buffer: *mut f32,
) -> bool {
    update_particle(
        unsafe { &mut *thread_state },
        unsafe { &mut *particle_state },
        local_render_group_type_id,
        delta_time,
        origin,
        render_buffer,
    )
}

/// All the types of emitters that your particles mod supports
#[derive(FromPrimitive)]
enum EmitterType {
    Campfire = 1,
    WoodChop = 2,
    Feathers = 3,
}

/// ALl the different ways your mod can render a particle
#[repr(u32)]
#[derive(FromPrimitive)]
enum ParticleRenderType {
    Smoke = 1,
    Fire = 2,
    Standard = 3,
    Spark = 4,
}

/// Updates a single particle
pub fn update_particle(
    _: &mut SPParticleThreadState,
    particle_state: &mut SPParticleState,
    render_group: u32,
    delta_time: f64,
    _: SPVec3,
    render_buffer: *mut f32,
) -> bool {
    let life_left_multiplier = match FromPrimitive::from_u32(render_group) {
        Some(ParticleRenderType::Smoke) => 0.01,
        Some(ParticleRenderType::Fire) => 1.5 - particle_state.randomValueB * 0.5,
        Some(ParticleRenderType::Spark) => 1.5 - particle_state.randomValueB * 1.0,
        _ => 1.0,
    };

    let life_left = particle_state.lifeLeft - delta_time * life_left_multiplier;

    particle_state.lifeLeft = life_left;
    if life_left < 0.0 {
        return false;
    }

    match FromPrimitive::from_u32(render_group) {
        Some(ParticleRenderType::Smoke) => {
            particle_state.v = vec3_mul(particle_state.v, 1.0 - delta_time * 0.05);

            let vel = vec3_add(particle_state.v, particle_state.gravity);

            particle_state.p = vec3_add(particle_state.p, vec3_mul(vel, delta_time));
            particle_state.scale +=
                delta_time * particle_state.lifeLeft * (1.0 + particle_state.randomValueA) * 0.15;
        }
        Some(ParticleRenderType::Fire) => {
            particle_state.p = vec3_mul(particle_state.p, 2.0 - life_left * delta_time);
        }
        _ => {
            particle_state.v = vec3_add(
                particle_state.v,
                vec3_mul(particle_state.gravity, delta_time),
            );
            particle_state.p = vec3_add(particle_state.p, vec3_mul(particle_state.v, delta_time));
        }
    }

    true
}
