//! Reimplementation of
//! https://github.com/mjdave/sapiens-mod-creation/blob/master/SPlugins/Examples/SPVanilla/src/SPParticles.c to show how
//! to use the Rust API to make Sapiens particles

extern crate num_derive;
extern crate num_traits;
extern crate sapiens_rs;
extern crate sapiens_rs_macros;
extern crate sapiens_sys;

use num_derive::{FromPrimitive, ToPrimitive};
use sapiens_rs::sp;
use sapiens_rs::sp::particles::*;
use sapiens_rs::sp_meters_to_prerender;
use sapiens_rs_macros::export_to_sapiens;
use sapiens_sys::*;

#[derive(Debug, PartialEq, FromPrimitive, ToPrimitive)]
enum VanillaEmitterType {
    Campfire,
    WoodChop,
    Feathers,
}

#[derive(Debug, PartialEq, FromPrimitive, ToPrimitive)]
enum VanillaRenderType {
    Smoke,
    Fire,
    Standard,
    Spark,
}

#[export_to_sapiens]
fn get_emitter_types_count() -> i32 {
    3
}

#[export_to_sapiens]
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

#[export_to_sapiens]
fn get_render_group_types_count() -> u32 {
    4
}

#[export_to_sapiens]
fn get_render_group_types() -> Vec<RenderGroupInfo<VanillaRenderType>> {
    vec![
        RenderGroupInfo {
            shader_name: "smokeParticle".to_string(),
            id: VanillaRenderType::Smoke,
            vertex_descriptions: vec![
                VertexAttributeType::Vec3,
                VertexAttributeType::Vec2,
                VertexAttributeType::Vec4,
            ],
        },
        RenderGroupInfo {
            shader_name: "fireParticle".to_string(),
            id: VanillaRenderType::Fire,
            vertex_descriptions: vec![
                VertexAttributeType::Vec3,
                VertexAttributeType::Vec2,
                VertexAttributeType::Vec4,
            ],
        },
        RenderGroupInfo {
            shader_name: "particle".to_string(),
            id: VanillaRenderType::Standard,
            vertex_descriptions: vec![
                VertexAttributeType::Vec3,
                VertexAttributeType::Vec2,
                VertexAttributeType::Vec4,
            ],
        },
        RenderGroupInfo {
            shader_name: "spark".to_string(),
            id: VanillaRenderType::Spark,
            vertex_descriptions: vec![
                VertexAttributeType::Vec3,
                VertexAttributeType::Vec2,
                VertexAttributeType::Vec4,
            ],
        },
    ]
}

#[export_to_sapiens]
fn emitter_was_added(
    thread_state: &ThreadState,
    emitter_state: &mut EmitterState,
    emitter_type: VanillaEmitterType,
) -> bool {
    let mut remove_immediately = false;

    match emitter_type {
        VanillaEmitterType::Campfire => {}
        VanillaEmitterType::WoodChop => {
            remove_immediately = true;

            let pos_length = sp::vec3_length(emitter_state.p);
            let normalized_pos = sp::vec3_div(emitter_state.p, pos_length);
            let gravity = sp::vec3_mul(normalized_pos, sp_meters_to_prerender!(-10.0));

            for _ in 0..32 {
                let rand_pos_vec =
                    sp::vec3_mul(thread_state.rand.get_vec3(), sp_meters_to_prerender!(0.2));
                let rand_vel_vec = thread_state.rand.get_vec3();

                let mut state = ParticleState {
                    p: sp::vec3_add(sp::vec3_mul(normalized_pos, pos_length), rand_pos_vec),
                    v: sp::vec3_mul(
                        sp::vec3_add(normalized_pos, rand_vel_vec),
                        sp_meters_to_prerender!(1.0),
                    ),
                    gravity,
                    lifeLeft: 1.0,
                    scale: 1.0,
                    randomValueA: thread_state.rand.get_float(),
                    randomValueB: 0.0,
                    userData: SPVec4::default(),
                    particleTextureType: 3,
                };

                thread_state.add_particle(emitter_state, VanillaRenderType::Standard, &mut state);
            }
        }
        VanillaEmitterType::Feathers => {
            remove_immediately = true;
            let pos_length = sp::vec3_length(emitter_state.p);
            let normalized_pos = sp::vec3_div(emitter_state.p, pos_length);
            let gravity = sp::vec3_mul(normalized_pos, sp_meters_to_prerender!(-2.0));

            for _ in 0..32 {
                let rand_pos_vec =
                    sp::vec3_mul(thread_state.rand.get_vec3(), sp_meters_to_prerender!(0.2));
                let rand_vel_vec = thread_state.rand.get_vec3();

                let mut state = ParticleState {
                    p: sp::vec3_add(
                        sp::vec3_mul(normalized_pos, pos_length + sp_meters_to_prerender!(0.25)),
                        rand_pos_vec,
                    ),
                    v: sp::vec3_mul(
                        sp::vec3_add(normalized_pos, rand_vel_vec),
                        sp_meters_to_prerender!(1.0),
                    ),
                    gravity,
                    lifeLeft: 1.0,
                    scale: 1.0,
                    randomValueA: thread_state.rand.get_float(),
                    randomValueB: 0.0,
                    userData: SPVec4::default(),
                    particleTextureType: 3,
                };

                thread_state.add_particle(emitter_state, VanillaRenderType::Standard, &mut state);
            }
        }
    }

    if !remove_immediately {
        for i in 0..4 {
            emitter_state.counters[i] = 0;
        }

        emitter_state.timeAccumulatorA = 0.0;
        emitter_state.timeAccumulatorB = 0.0;
    }

    remove_immediately
}

const FIXED_TIME_STEP: f64 = 1.0 / 60.0;

fn update_emitter(
    thread_state: &mut ThreadState,
    emitter_state: &mut EmitterState,
    emitter_type: VanillaEmitterType,
    delta_time: f64,
) {
    let rand = &thread_state.rand;

    emitter_state.timeAccumulatorA += delta_time;

    // Run particle simulations at a fixed time step
    while emitter_state.timeAccumulatorA > 0.0 {
        emitter_state.timeAccumulatorA -= FIXED_TIME_STEP;
        emitter_state.timeAccumulatorB += FIXED_TIME_STEP;

        match emitter_type {
            VanillaEmitterType::Campfire => {
                if emitter_state.counters[0] == 0 {
                    // smoke
                    let pos_length = sp::vec3_length(emitter_state.p);
                    let normalized_pos = sp::vec3_div(emitter_state.p, pos_length);
                    let state = SPParticleState {};
                }
            }
            VanillaEmitterType::WoodChop => {}
            VanillaEmitterType::Feathers => {}
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_emitter_types_count() {
        let emitter_types_count = get_emitter_types_count();
        let ffi_emitter_types_count = spGetEmitterTypesCount();

        assert_eq!(emitter_types_count, ffi_emitter_types_count);
    }

    #[test]
    fn test_get_emitter_types() {
        let emitter_types_count = get_emitter_types_count() as usize;
        let raw_ffi_emitters = spGetEmitterTypes();

        let ffi_emitters = unsafe {
            Vec::<SPParticleEmitterTypeInfo>::from_raw_parts(
                raw_ffi_emitters,
                emitter_types_count,
                emitter_types_count,
            )
        };

        let emitters = get_emitter_types();

        assert_eq!(emitters, ffi_emitters);
    }

    #[test]
    fn test_render_group_types_count() {
        let render_group_types_count = get_render_group_types_count();
        let ffi_render_group_types_count = spGetRenderGroupTypesCount() as u32;

        assert_eq!(render_group_types_count, ffi_render_group_types_count);
    }

    #[test]
    fn test_get_render_group_types() {
        let render_group_types_count = get_render_group_types_count() as usize;
        let raw_ffi_render_types = spGetRenderGroupTypes();

        let ffi_render_types = unsafe {
            Vec::<SPParticleRenderGroupInfo>::from_raw_parts(
                raw_ffi_render_types,
                render_group_types_count,
                render_group_types_count,
            )
        };

        let render_types = get_render_group_types();

        assert_eq!(render_types, ffi_render_types);
    }
}
