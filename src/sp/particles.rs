//! Rusty structs for particle things

use crate::sp::math::{Mat3, Vec3, Vec4};
use sapiens_sys::*;
use std::convert::TryFrom;
use std::ffi::{CStr, CString};
use std::mem;
use std::os::raw;

#[repr(i32)]
pub enum RenderGroupVertexDescriptionType {
    Float = SPRenderGroupVertexDescriptionType_SPRenderGroupVertexDescriptionType_float,
    Vec2 = SPRenderGroupVertexDescriptionType_SPRenderGroupVertexDescriptionType_vec2,
    Vec3 = SPRenderGroupVertexDescriptionType_SPRenderGroupVertexDescriptionType_vec3,
    Vec4 = SPRenderGroupVertexDescriptionType_SPRenderGroupVertexDescriptionType_vec4,
}

impl TryFrom<i32> for RenderGroupVertexDescriptionType {
    type Error = ();

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
            SPRenderGroupVertexDescriptionType_SPRenderGroupVertexDescriptionType_float => {
                Ok(RenderGroupVertexDescriptionType::Float)
            }
            SPRenderGroupVertexDescriptionType_SPRenderGroupVertexDescriptionType_vec2 => {
                Ok(RenderGroupVertexDescriptionType::Vec2)
            }
            SPRenderGroupVertexDescriptionType_SPRenderGroupVertexDescriptionType_vec3 => {
                Ok(RenderGroupVertexDescriptionType::Vec3)
            }
            SPRenderGroupVertexDescriptionType_SPRenderGroupVertexDescriptionType_vec4 => {
                Ok(RenderGroupVertexDescriptionType::Vec4)
            }
            _ => Err(()),
        }
    }
}

pub struct RenderGroupInfo {
    pub shader_name: String,
    pub id: u32,
    pub vertex_descriptions: Vec<RenderGroupVertexDescriptionType>,
}

impl From<SPParticleRenderGroupInfo> for RenderGroupInfo {
    fn from(sp_info: SPParticleRenderGroupInfo) -> Self {
        RenderGroupInfo {
            shader_name: unsafe { CStr::from_ptr(sp_info.shaderName) }
                .to_str()
                .unwrap()
                .to_owned(),
            id: sp_info.localID,
            vertex_descriptions: unsafe {
                Vec::from_raw_parts(
                    sp_info.vertexDescriptionTypes as _,
                    sp_info.vertexDescriptionTypeCount as _,
                    sp_info.vertexDescriptionTypeCount as _,
                )
            },
        }
    }
}

impl Into<SPParticleRenderGroupInfo> for RenderGroupInfo {
    fn into(mut self) -> SPParticleRenderGroupInfo {
        self.vertex_descriptions.shrink_to_fit();
        let ptr = self.vertex_descriptions.as_mut_ptr() as *mut raw::c_int;
        let len = self.vertex_descriptions.len() as raw::c_int;
        mem::forget(self.vertex_descriptions);

        SPParticleRenderGroupInfo {
            shaderName: CString::new(self.shader_name).unwrap(),
            localID: self.id,
            vertexDescriptionTypeCount: len,
            vertexDescriptionTypes: ptr,
        }
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
