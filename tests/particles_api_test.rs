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
use std::ptr::slice_from_raw_parts_mut;

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

            let pos_length = sp::vec3_length(&emitter_state.p);
            let normalized_pos = sp::vec3_div(&emitter_state.p, pos_length);
            let gravity = sp::vec3_mul(&normalized_pos, sp_meters_to_prerender!(-10.0));

            for _ in 0..32 {
                let rand_pos_vec =
                    sp::vec3_mul(&thread_state.rand.get_vec3(), sp_meters_to_prerender!(0.2));
                let rand_vel_vec = thread_state.rand.get_vec3();

                let mut state = ParticleState {
                    p: sp::vec3_add(&sp::vec3_mul(&normalized_pos, pos_length), &rand_pos_vec),
                    v: sp::vec3_mul(
                        &sp::vec3_add(&normalized_pos, &rand_vel_vec),
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
            let pos_length = sp::vec3_length(&emitter_state.p);
            let normalized_pos = sp::vec3_div(&emitter_state.p, pos_length);
            let gravity = sp::vec3_mul(&normalized_pos, sp_meters_to_prerender!(-2.0));

            for _ in 0..32 {
                let rand_pos_vec =
                    sp::vec3_mul(&thread_state.rand.get_vec3(), sp_meters_to_prerender!(0.2));
                let rand_vel_vec = thread_state.rand.get_vec3();

                let mut state = ParticleState {
                    p: sp::vec3_add(
                        &sp::vec3_mul(&normalized_pos, pos_length + sp_meters_to_prerender!(0.25)),
                        &rand_pos_vec,
                    ),
                    v: sp::vec3_mul(
                        &sp::vec3_add(&normalized_pos, &rand_vel_vec),
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
                // smoke
                if emitter_state.counters[0] == 0 {
                    let pos_length = sp::vec3_length(&emitter_state.p);
                    let normalized_pos = sp::vec3_div(&emitter_state.p, pos_length);

                    let lookup = SPVec3 {
                        x: (normalized_pos.x + 1.2) * 99999.9,
                        y: (normalized_pos.y * 4.5 + normalized_pos.z + 2.4) * 99999.9,
                        z: emitter_state.timeAccumulatorB * 0.1,
                    };
                    let lookup_b = SPVec3 {
                        x: (normalized_pos.x + 1.4) * 99999.9,
                        y: (normalized_pos.y * 4.6 + normalized_pos.z + 2.8) * 99999.9,
                        z: emitter_state.timeAccumulatorB * 0.1,
                    };
                    let lookup_c = SPVec3 {
                        x: (normalized_pos.x + 1.8) * 99999.9,
                        y: (normalized_pos.y * 4.8 + normalized_pos.z + 2.9) * 99999.9,
                        z: emitter_state.timeAccumulatorB * 0.5,
                    };

                    let noise_value = thread_state.noise.get(&lookup, 2);
                    let noise_value_b = thread_state.noise.get(&lookup_b, 2);
                    let noise_value_c = thread_state.noise.get(&lookup_c, 2);

                    let intermediate_gravity = sp::vec3_mul(
                        &sp::mat3_get_row(&emitter_state.rot, 0),
                        sp_meters_to_prerender!(noise_value) * 0.5,
                    );

                    let mut state = SPParticleState {
                        p: sp::vec3_mul(&normalized_pos, pos_length + sp_meters_to_prerender!(0.2)),
                        v: sp::vec3_mul(
                            &normalized_pos,
                            sp_meters_to_prerender!(0.5 + noise_value_c) * 0.5,
                        ),
                        particleTextureType: 2,
                        lifeLeft: 1.0,
                        scale: 0.2 + rand.get_float() * 0.2,
                        randomValueA: rand.get_float(),
                        randomValueB: 0.0,
                        gravity: sp::vec3_add(
                            &intermediate_gravity,
                            &sp::vec3_mul(
                                &sp::mat3_get_row(&emitter_state.rot, 2),
                                sp_meters_to_prerender!(noise_value_b) * 0.5,
                            ),
                        ),
                        userData: Default::default(),
                    };

                    thread_state.add_particle(emitter_state, VanillaRenderType::Smoke, &mut state);

                    emitter_state.counters[0] = (1.0 + (20.0 * (1.0 - noise_value_c))) as u8;
                } else {
                    emitter_state.counters[0] -= 1;
                }

                // Flame 1
                if emitter_state.counters[1] == 0 {
                    let rand_pos_vec =
                        sp::vec3_mul(&rand.get_vec3(), sp_meters_to_prerender!(0.04));
                    let scale_average = 0.5;

                    emit_fire_particle(thread_state, emitter_state, scale_average, &rand_pos_vec);

                    emitter_state.counters[1] = (5.0 + (20.0 * rand.get_float())) as u8;
                } else {
                    emitter_state.counters[1] -= 1;
                }

                // Flame 2
                if emitter_state.counters[2] == 0 {
                    let rand_pos_vec =
                        sp::vec3_mul(&rand.get_vec3(), sp_meters_to_prerender!(0.04));
                    let rand_pos_vec = sp::vec3_add(
                        &rand_pos_vec,
                        &sp::vec3_mul(
                            &sp::mat3_get_row(&emitter_state.rot, 0),
                            sp_meters_to_prerender!(0.2),
                        ),
                    );

                    let scale_average = 0.35;

                    emit_fire_particle(thread_state, emitter_state, scale_average, &rand_pos_vec);

                    emitter_state.counters[2] = (5.0 + (20.0 * rand.get_float())) as u8;
                } else {
                    emitter_state.counters[2] -= 1;
                }

                // Flame 3
                if emitter_state.counters[4] == 0 {
                    let rand_pos_vec =
                        sp::vec3_mul(&rand.get_vec3(), sp_meters_to_prerender!(0.04));
                    let rand_pos_vec = sp::vec3_add(
                        &rand_pos_vec,
                        &sp::vec3_mul(
                            &sp::mat3_get_row(&emitter_state.rot, 2),
                            sp_meters_to_prerender!(0.1),
                        ),
                    );
                    let scale_average = 0.2;

                    emit_fire_particle(thread_state, emitter_state, scale_average, &rand_pos_vec);

                    emitter_state.counters[3] = (5.0 + (20.0 * rand.get_float())) as u8;
                } else {
                    if emitter_state.counters[3] == 18 {
                        // Spark
                        let pos_length = sp::vec3_length(&emitter_state.p);
                        let normalized_pos = sp::vec3_div(&emitter_state.p, pos_length);

                        let rand_vec = rand.get_vec3();
                        let rand_pos_vec = sp::vec3_mul(&rand_vec, sp_meters_to_prerender!(0.1));
                        let rand_vel_vec = sp::vec3_mul(&rand_vec, sp_meters_to_prerender!(0.4));

                        let mut state = SPParticleState {
                            p: sp::vec3_add(
                                &sp::vec3_mul(
                                    &normalized_pos,
                                    pos_length + sp_meters_to_prerender!(0.1),
                                ),
                                &rand_pos_vec,
                            ),
                            v: sp::vec3_mul(
                                &sp::vec3_add(&normalized_pos, &rand_vel_vec),
                                sp_meters_to_prerender!(2.0 + rand.get_float() * 0.5),
                            ),
                            gravity: sp::vec3_mul(&rand.get_vec3(), sp_meters_to_prerender!(1.0)),
                            lifeLeft: 1.0,
                            scale: 0.01 + rand.get_float() * 0.02,
                            randomValueA: rand.get_float(),
                            randomValueB: rand.get_float(),
                            userData: Default::default(),
                            particleTextureType: 3,
                        };

                        thread_state.add_particle(
                            emitter_state,
                            VanillaRenderType::Spark,
                            &mut state,
                        );
                    }

                    emitter_state.counters[3] -= 1;
                }
            }
            VanillaEmitterType::WoodChop => {}
            VanillaEmitterType::Feathers => {}
        }
    }
}

fn emit_fire_particle(
    thread_state: &ThreadState,
    emitter_state: &mut EmitterState,
    scale_average: f64,
    rand_pos_vec: &SPVec3,
) {
    let rand = &thread_state.rand;

    let pos_length = sp::vec3_length(&emitter_state.p);
    let normalized_pos = sp::vec3_div(&emitter_state.p, pos_length);

    let mut state = SPParticleState {
        p: sp::vec3_add(
            &sp::vec3_mul(&normalized_pos, pos_length + sp_meters_to_prerender!(0.1)),
            rand_pos_vec,
        ),
        v: sp::vec3_mul(
            &normalized_pos,
            sp_meters_to_prerender!(0.2 + rand.get_float() * 0.2),
        ),
        gravity: Default::default(),
        lifeLeft: 1.0,
        scale: scale_average + rand.get_float() * 0.2,
        randomValueA: rand.get_float(),
        randomValueB: rand.get_float(),
        userData: Default::default(),
        particleTextureType: match rand.get_float() {
            x if x < 0.5 => 1,
            _ => 4,
        },
    };

    thread_state.add_particle(emitter_state, VanillaRenderType::Fire, &mut state);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_emitter_types_count() {
        let emitter_types_count = get_emitter_types_count();
        let ffi_emitter_types_count = unsafe { spGetEmitterTypesCount() };

        assert_eq!(emitter_types_count, ffi_emitter_types_count);
    }

    #[test]
    fn test_get_emitter_types() {
        let emitter_types_count = get_emitter_types_count() as usize;
        let raw_ffi_emitters = unsafe { spGetEmitterTypes() };

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
        let ffi_render_group_types_count = unsafe { spGetRenderGroupTypesCount() } as u32;

        assert_eq!(render_group_types_count, ffi_render_group_types_count);
    }

    #[test]
    fn test_get_render_group_types() {
        let render_group_types_count = get_render_group_types_count() as usize;
        let raw_ffi_render_types = unsafe { spGetRenderGroupTypes() };

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
