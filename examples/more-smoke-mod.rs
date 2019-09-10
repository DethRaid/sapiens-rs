extern crate sapiens_rs;
extern crate sapiens_sys;

use sapiens_sys::*;

use sapiens_rs::sp::math::Vec3;
use sapiens_rs::sp::{vec3_mul, vec3_add};

/// Sapiens calls this function once per particle to update that particle's position, velocity, and rendering
/// information
///
/// Rather than
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

enum EmitterType {
    Campfire = 1,
    WoodChop = 2,
    Feathers = 3,
}

#[repr(u32)]
enum ParticleRenderType {
    Smoke = 1,
    Fire = 2,
    Standard = 3,
    Spark = 4,
}

/// Updates a single particle
pub fn update_particle(
    thread_state: &mut SPParticleThreadState,
    particle_state: &mut SPParticleState,
    render_group: u32,
    delta_time: f64,
    origin: SPVec3,
    render_buffer: *mut f32,
) -> bool {
    let life_left_multiplier = match render_group {
        ParticleRenderType::Smoke as u32 => 0.01,
        ParticleRenderType::Fire as u32 => 1.5 - particle_state.randomValueB * 0.5,
        ParticleRenderType::Spark as u32 => 1.5 - particle_state.randomValueB * 1.0,
        _ => 1.0,
    };

    let life_left = particle_state.lifeLeft - delta_time * life_left_multiplier;

    particle_state.lifeLeft = life_left;
    if life_left < 0.0 {
        return false;
    }

    match render_group as ParticleRenderType {
        ParticleRenderType::Smoke => {
            particle_state.v = vec3_mul(particle_state.v, 1.0 - delta_time * 0.05);

            let vel = vec3_add(particle_state.v, particle_state.gravity);

            particle_state.p = vec3_add(particle_state.p, vec3_mul(vel, delta_time));
            particle_state.scale = particle_state.scale
                + delta_time * particle_state.lifeLeft * (1.0 + particle_state.randomValueA) * 0.15;
        }
        ParticleRenderType::Fire => {
            particle_state.p = vec3_mul(particle_state.p,2.0 - life_left) * delta_time);
        }
        _ => {
            particle_state.v = vec3_add(particle_state.v,vec3_mul(particle_state.gravity, delta_time));
            particle_state.p = vec3_add(particle_state.p, vec3_mul(particle_state.v, delta_time));
        }
    }

    true
}
