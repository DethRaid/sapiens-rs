//! Rusty structs for particle things

#![allow(non_upper_case_globals)]

use crate::sp::math::{Mat3, Vec3, Vec4};
use sapiens_sys::*;
use std::convert::{TryFrom, TryInto};
use std::ffi::{CStr, CString};
use std::mem;
use std::os::raw;

/// All the types of vertex attributes that Sapiens supports
#[repr(i32)]
#[derive(Debug, PartialEq)]
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

impl<RenderGroupIdType> TryFrom<SPParticleRenderGroupInfo> for RenderGroupInfo<RenderGroupIdType>
where
    RenderGroupIdType: TryFrom<u32>,
{
    type Error = ();

    fn try_from(sp_info: SPParticleRenderGroupInfo) -> Result<Self, Self::Error> {
        Ok(RenderGroupInfo {
            shader_name: unsafe { CStr::from_ptr(sp_info.shaderName) }
                .to_str()
                .unwrap()
                .to_owned(),
            id: match TryFrom::try_from(sp_info.localID) {
                Ok(val) => val,
                Err(_) => panic!("it broke"),
            },
            vertex_descriptions: unsafe {
                Vec::from_raw_parts(
                    sp_info.vertexDescriptionTypes as _,
                    sp_info.vertexDescriptionTypeCount as _,
                    sp_info.vertexDescriptionTypeCount as _,
                )
            },
        })
    }
}

impl<RenderGroupTypeId> TryInto<SPParticleRenderGroupInfo> for RenderGroupInfo<RenderGroupTypeId>
where
    RenderGroupTypeId: TryInto<u32>,
{
    type Error = ();

    fn try_into(mut self) -> Result<SPParticleRenderGroupInfo, Self::Error> {
        self.vertex_descriptions.shrink_to_fit();
        let ptr = self.vertex_descriptions.as_mut_ptr() as *mut raw::c_int;
        let len = self.vertex_descriptions.len() as raw::c_int;
        mem::forget(self.vertex_descriptions);

        Ok(SPParticleRenderGroupInfo {
            shaderName: CString::new(self.shader_name).unwrap().into_raw() as _,
            localID: match TryInto::try_into(self.id) {
                Ok(val) => val,
                Err(_) => panic!("it broke"),
            },
            vertexDescriptionTypeCount: len,
            vertexDescriptionTypes: ptr,
        })
    }
}

impl<RenderGroupTypeId> PartialEq<SPParticleRenderGroupInfo> for RenderGroupInfo<RenderGroupTypeId>
where
    RenderGroupTypeId: TryFrom<u32> + PartialEq,
{
    fn eq(&self, sp_info: &SPParticleRenderGroupInfo) -> bool {
        let ffi_vertex_desc = unsafe {
            Vec::from_raw_parts(
                sp_info.vertexDescriptionTypes as *mut VertexAttributeType,
                sp_info.vertexDescriptionTypeCount as _,
                sp_info.vertexDescriptionTypeCount as _,
            )
        };

        let ffi_shader_name = unsafe { CStr::from_ptr(sp_info.shaderName) }
            .to_str()
            .unwrap()
            .to_owned();

        self.shader_name == ffi_shader_name
            && self.id
                == (match TryFrom::try_from(sp_info.localID) {
                    Ok(val) => val,
                    Err(_) => panic!("it broke"),
                })
            && self.vertex_descriptions == ffi_vertex_desc
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

/*
 * Tests
 */

#[cfg(test)]
mod tests {
    use super::*;

    #[repr(u32)]
    #[derive(Debug)]
    enum RenderGroupId {
        Fire,
        Smoke,
    }

    #[test]
    fn i32_into_vertex_attribute_type() {
        let sp_type: i32 =
            SPRenderGroupVertexDescriptionType_SPRenderGroupVertexDescriptionType_float;
        let rs_type: VertexAttributeType = TryFrom::try_from(sp_type).unwrap();

        assert_eq!(rs_type, VertexAttributeType::Float);

        let sp_type: i32 =
            SPRenderGroupVertexDescriptionType_SPRenderGroupVertexDescriptionType_vec2;
        let rs_type: VertexAttributeType = TryFrom::try_from(sp_type).unwrap();

        assert_eq!(rs_type, VertexAttributeType::Vec2);

        let sp_type: i32 =
            SPRenderGroupVertexDescriptionType_SPRenderGroupVertexDescriptionType_vec3;
        let rs_type: VertexAttributeType = TryFrom::try_from(sp_type).unwrap();

        assert_eq!(rs_type, VertexAttributeType::Vec3);

        let sp_type: i32 =
            SPRenderGroupVertexDescriptionType_SPRenderGroupVertexDescriptionType_vec4;
        let rs_type: VertexAttributeType = TryFrom::try_from(sp_type).unwrap();

        assert_eq!(rs_type, VertexAttributeType::Vec4);
    }

    #[test]
    fn test_render_group_info_from_sp_particle_render_group_info() {
        let shader_name = CString::new("fire").unwrap();

        let mut vertex_description_types: [VertexAttributeType; 3] = [
            VertexAttributeType::Vec3,
            VertexAttributeType::Vec2,
            VertexAttributeType::Vec4,
        ];

        let sp_render_group = SPParticleRenderGroupInfo {
            shaderName: shader_name.into_raw(),
            localID: RenderGroupId::Fire as _,
            vertexDescriptionTypeCount: vertex_description_types.len() as _,
            vertexDescriptionTypes: vertex_description_types.as_mut_ptr() as _,
        };

        let render_group: RenderGroupInfo<RenderGroupId> =
            TryFrom::try_from(sp_render_group).unwrap();

        assert_eq!(render_group.shader_name, "fire");
        assert_eq!(render_group.id, RenderGroupId::Fire);
        assert_eq!(
            render_group.vertex_descriptions.len(),
            vertex_description_types.len()
        );
        assert_eq!(
            render_group.vertex_descriptions[0],
            vertex_description_types[0]
        );
        assert_eq!(
            render_group.vertex_descriptions[1],
            vertex_description_types[1]
        );
        assert_eq!(
            render_group.vertex_descriptions[2],
            vertex_description_types[2]
        );
    }
}
