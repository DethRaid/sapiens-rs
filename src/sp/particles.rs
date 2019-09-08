//! Rusty structs for particle things

use sapiens_sys::*;
use crate::sp::math::{Vec3, Vec4, Mat3};

#[repr(i32)]
pub enum RenderGroupVertexDescriptionType {
    Float = SPRenderGroupVertexDescriptionType_SPRenderGroupVertexDescriptionType_float,
    Vec2 = SPRenderGroupVertexDescriptionType_SPRenderGroupVertexDescriptionType_vec2,
    Vec3 = SPRenderGroupVertexDescriptionType_SPRenderGroupVertexDescriptionType_vec3,
    Vec4 = SPRenderGroupVertexDescriptionType_SPRenderGroupVertexDescriptionType_vec4,
}

pub struct RenderGroupInfo(SPParticleRenderGroupInfo);

impl RenderGroupInfo {
    pub fn new(shader_name: &mut str, local_id: u32, vertex_description_types: &mut [RenderGroupVertexDescriptionType]) -> Self {
        RenderGroupInfo(SPParticleRenderGroupInfo {
            shaderName: unsafe { shader_name.as_mut_ptr() as _ },
            localID: local_id,
            vertexDescriptionTypeCount: vertex_description_types.len() as i32,
            vertexDescriptionTypes: unsafe { vertex_description_types.as_mut_ptr() as I }
        })
    }
}

pub struct EmitterTypeInfo(SPParticleEmitterTypeInfo);

impl EmitterTypeInfo {
    pub fn new(name: &mut str, local_id: u32) -> Self {
        EmitterTypeInfo(SPParticleEmitterTypeInfo {
            name: unsafe { name.as_mut_ptr() as _ },
            localID: local_id
        })
    }
}

pub struct EmitterState(SPParticleEmitterState);

impl EmitterState {
    pub fn new(position: Vec3, rotation: Mat3, time_accumulator_a: f64, time_accumulator_b: f64, user_data: Vec4, global_type: u32, counters: [u8; 4]) -> Self {
        EmitterState(SPParticleEmitterState {
            p: position.to_sp_vec3(),
            rot: rotation.to_sp_mat3(),
            timeAccumulatorA: time_accumulator_a,
            timeAccumulatorB: time_accumulator_b,
            userData: user_data.to_sp_vec4(),
            globalType: global_type,
            counters: counters
        })
    }
}
