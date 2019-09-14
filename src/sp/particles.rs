//! Rusty structs for particle things

use crate::sp::math::{Mat3, Vec3, Vec4};
use sapiens_sys::*;
use std::convert::TryFrom;
use std::ffi::{CStr, CString};
use std::mem;
use std::os::raw;

/// All the types of vertex attributes that Sapiens supports
#[repr(i32)]
pub enum VertexAttributeType {
    Float = SPRenderGroupVertexDescriptionType_SPRenderGroupVertexDescriptionType_float,
    Vec2 = SPRenderGroupVertexDescriptionType_SPRenderGroupVertexDescriptionType_vec2,
    Vec3 = SPRenderGroupVertexDescriptionType_SPRenderGroupVertexDescriptionType_vec3,
    Vec4 = SPRenderGroupVertexDescriptionType_SPRenderGroupVertexDescriptionType_vec4,
}

impl TryFrom<i32> for VertexAttributeType {
    type Error = ();

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
            SPRenderGroupVertexDescriptionType_SPRenderGroupVertexDescriptionType_float => {
                Ok(VertexAttributeType::Float)
            }
            SPRenderGroupVertexDescriptionType_SPRenderGroupVertexDescriptionType_vec2 => {
                Ok(VertexAttributeType::Vec2)
            }
            SPRenderGroupVertexDescriptionType_SPRenderGroupVertexDescriptionType_vec3 => {
                Ok(VertexAttributeType::Vec3)
            }
            SPRenderGroupVertexDescriptionType_SPRenderGroupVertexDescriptionType_vec4 => {
                Ok(VertexAttributeType::Vec4)
            }
            _ => Err(()),
        }
    }
}

/// Idiomatic struct for a information about a render group
///
/// TODO: Generizise this so people can use an enum for their render group IDs
pub struct RenderGroupInfo<RenderGroupIdType> {
    /// Name of the shader to use to render this render group
    pub shader_name: String,

    /// Identifier for the render group
    pub id: RenderGroupIdType,

    /// All the vertex attributes that make up the vertex data that this render group renders
    pub vertex_descriptions: Vec<VertexAttributeType>,
}

impl<RenderGroupIdType> From<SPParticleRenderGroupInfo> for RenderGroupInfo<RenderGroupIdType>
where
    RenderGroupIdType: From<u32> + Into<u32>,
{
    fn from(sp_info: SPParticleRenderGroupInfo) -> Self {
        RenderGroupInfo {
            shader_name: unsafe { CStr::from_ptr(sp_info.shaderName) }
                .to_str()
                .unwrap()
                .to_owned(),
            id: From::from(sp_info.localID),
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

impl<RenderGroupTypeId> Into<SPParticleRenderGroupInfo> for RenderGroupInfo<RenderGroupTypeId>
where
    RenderGroupTypeId: From<u32> + Into<u32>,
{
    fn into(mut self) -> SPParticleRenderGroupInfo {
        self.vertex_descriptions.shrink_to_fit();
        let ptr = self.vertex_descriptions.as_mut_ptr() as *mut raw::c_int;
        let len = self.vertex_descriptions.len() as raw::c_int;
        mem::forget(self.vertex_descriptions);

        SPParticleRenderGroupInfo {
            shaderName: CString::new(self.shader_name).unwrap().into_raw() as _,
            localID: Into::into(self.id),
            vertexDescriptionTypeCount: len,
            vertexDescriptionTypes: ptr,
        }
    }
}

pub struct EmitterTypeInfo<EmitterTypeId> {
    pub name: String,
    pub id: EmitterTypeId,
}

impl<EmitterTypeId> From<SPParticleEmitterTypeInfo> for EmitterTypeInfo<EmitterTypeId>
where
    EmitterTypeId: From<u32> + Into<u32>,
{
    fn from(sp_emitter_type: SPParticleEmitterTypeInfo) -> Self {
        EmitterTypeInfo {
            name: unsafe { CStr::from_ptr(sp_emitter_type.name) }
                .to_str()
                .unwrap()
                .to_owned(),
            id: From::from(sp_emitter_type.localID),
        }
    }
}

impl<EmitterTypeId> Into<SPParticleEmitterTypeInfo> for EmitterTypeInfo<EmitterTypeId>
where
    EmitterTypeId: From<u32> + Into<u32>,
{
    fn into(self) -> SPParticleEmitterTypeInfo {
        SPParticleEmitterTypeInfo {
            name: CString::new(self.name).unwrap().into_raw() as _,
            localID: Into::into(self.id),
        }
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
