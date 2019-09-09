//! Rusty structs for particle things

use crate::sp::math::{Mat3, Vec3, Vec4};
use sapiens_sys::*;

#[repr(i32)]
pub enum RenderGroupVertexDescriptionType {
    Float = SPRenderGroupVertexDescriptionType_SPRenderGroupVertexDescriptionType_float,
    Vec2 = SPRenderGroupVertexDescriptionType_SPRenderGroupVertexDescriptionType_vec2,
    Vec3 = SPRenderGroupVertexDescriptionType_SPRenderGroupVertexDescriptionType_vec3,
    Vec4 = SPRenderGroupVertexDescriptionType_SPRenderGroupVertexDescriptionType_vec4,
}

pub struct RenderGroupInfo(SPParticleRenderGroupInfo);

impl RenderGroupInfo {
    pub fn new(
        shader_name: &mut str,
        local_id: u32,
        vertex_description_types: &mut [RenderGroupVertexDescriptionType],
    ) -> Self {
        RenderGroupInfo(SPParticleRenderGroupInfo {
            shaderName: shader_name.as_mut_ptr() as _,
            localID: local_id,
            vertexDescriptionTypeCount: vertex_description_types.len() as i32,
            vertexDescriptionTypes: vertex_description_types.as_mut_ptr() as _,
        })
    }
}

pub struct EmitterTypeInfo(SPParticleEmitterTypeInfo);

impl EmitterTypeInfo {
    pub fn new(name: &mut str, local_id: u32) -> Self {
        EmitterTypeInfo(SPParticleEmitterTypeInfo {
            name: name.as_mut_ptr() as _,
            localID: local_id,
        })
    }
}

pub struct EmitterState(SPParticleEmitterState);

impl EmitterState {
    pub fn new(
        position: Vec3,
        rotation: Mat3,
        time_accumulator_a: f64,
        time_accumulator_b: f64,
        user_data: Vec4,
        global_type: u32,
        counters: [u8; 4],
    ) -> Self {
        EmitterState(SPParticleEmitterState {
            p: position.as_sp_vec(),
            rot: rotation.as_sp_mat(),
            timeAccumulatorA: time_accumulator_a,
            timeAccumulatorB: time_accumulator_b,
            userData: user_data.as_sp_vec(),
            globalType: global_type,
            counters,
        })
    }
}

pub struct ParticleState(SPParticleState);

impl ParticleState {
    pub fn new(
        position: Vec3,
        velocity: Vec3,
        gravity: Vec3,
        life_left: f64,
        scale: f64,
        random_value_a: f64,
        random_value_b: f64,
        user_data: Vec4,
        particle_texture_type: u32,
    ) -> Self {
        ParticleState(SPParticleState {
            p: position.as_sp_vec(),
            v: velocity.as_sp_vec(),
            gravity: gravity.as_sp_vec(),
            lifeLeft: life_left,
            scale,
            randomValueA: random_value_a,
            randomValueB: random_value_b,
            userData: user_data.as_sp_vec(),
            particleTextureType: particle_texture_type,
        })
    }
}
