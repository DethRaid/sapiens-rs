//! Rusty structs for particle things

#![allow(non_upper_case_globals)]

use num_derive::{FromPrimitive, ToPrimitive};
use num_traits::{FromPrimitive, ToPrimitive};
use sapiens_sys::*;
use std::convert::{TryFrom, TryInto};
use std::ffi::{CStr, CString};
use std::mem;
use std::os::raw;

/// All the types of vertex attributes that Sapiens supports
#[repr(i32)]
#[derive(Debug, PartialEq, FromPrimitive, ToPrimitive)]
pub enum VertexAttributeType {
    Float = SPRenderGroupVertexDescriptionType_SPRenderGroupVertexDescriptionType_float,
    Vec2 = SPRenderGroupVertexDescriptionType_SPRenderGroupVertexDescriptionType_vec2,
    Vec3 = SPRenderGroupVertexDescriptionType_SPRenderGroupVertexDescriptionType_vec3,
    Vec4 = SPRenderGroupVertexDescriptionType_SPRenderGroupVertexDescriptionType_vec4,
}

/// Idiomatic struct for a information about a render group
pub struct RenderGroupInfo<RenderGroupIdType>
where
    RenderGroupIdType: FromPrimitive + ToPrimitive,
{
    /// Name of the shader to use to render this render group
    pub shader_name: String,

    /// Identifier for the render group
    pub id: RenderGroupIdType,

    /// All the vertex attributes that make up the vertex data that this render group renders
    pub vertex_descriptions: Vec<VertexAttributeType>,
}

impl<RenderGroupIdType> TryFrom<SPParticleRenderGroupInfo> for RenderGroupInfo<RenderGroupIdType>
where
    RenderGroupIdType: FromPrimitive + ToPrimitive,
{
    type Error = ();

    fn try_from(sp_info: SPParticleRenderGroupInfo) -> Result<Self, Self::Error> {
        Ok(RenderGroupInfo {
            shader_name: unsafe { CStr::from_ptr(sp_info.shaderName) }
                .to_str()
                .unwrap()
                .to_owned(),
            id: match FromPrimitive::from_u32(sp_info.localID) {
                Some(val) => val,
                None => panic!("it broke"),
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
    RenderGroupTypeId: FromPrimitive + ToPrimitive,
{
    type Error = ();

    fn try_into(mut self) -> Result<SPParticleRenderGroupInfo, Self::Error> {
        self.vertex_descriptions.shrink_to_fit();
        let ptr = self.vertex_descriptions.as_mut_ptr() as *mut raw::c_int;
        let len = self.vertex_descriptions.len() as raw::c_int;
        mem::forget(self.vertex_descriptions);

        Ok(SPParticleRenderGroupInfo {
            shaderName: CString::new(self.shader_name).unwrap().into_raw() as _,
            localID: match ToPrimitive::to_u32(&self.id) {
                Some(val) => val,
                None => panic!("it broke"),
            },
            vertexDescriptionTypeCount: len,
            vertexDescriptionTypes: ptr,
        })
    }
}

impl<RenderGroupTypeId> PartialEq<SPParticleRenderGroupInfo> for RenderGroupInfo<RenderGroupTypeId>
where
    RenderGroupTypeId: FromPrimitive + ToPrimitive + PartialEq,
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
                == (match FromPrimitive::from_u32(sp_info.localID) {
                    Some(val) => val,
                    None => panic!("it broke"),
                })
            && self.vertex_descriptions == ffi_vertex_desc
    }
}

pub struct EmitterTypeInfo<EmitterTypeId> {
    pub name: String,
    pub id: EmitterTypeId,
}

impl<EmitterTypeId> TryFrom<SPParticleEmitterTypeInfo> for EmitterTypeInfo<EmitterTypeId>
where
    EmitterTypeId: FromPrimitive + ToPrimitive,
{
    type Error = ();

    fn try_from(sp_emitter_type: SPParticleEmitterTypeInfo) -> Result<Self, Self::Error> {
        Ok(EmitterTypeInfo {
            name: unsafe { CStr::from_ptr(sp_emitter_type.name) }
                .to_str()
                .unwrap()
                .to_owned(),
            id: FromPrimitive::from_u32(sp_emitter_type.localID).unwrap(),
        })
    }
}

impl<EmitterTypeId> TryInto<SPParticleEmitterTypeInfo> for EmitterTypeInfo<EmitterTypeId>
where
    EmitterTypeId: FromPrimitive + ToPrimitive,
{
    type Error = ();

    fn try_into(self) -> Result<SPParticleEmitterTypeInfo, Self::Error> {
        Ok(SPParticleEmitterTypeInfo {
            name: CString::new(self.name).unwrap().into_raw() as _,
            localID: ToPrimitive::to_u32(&self.id).unwrap(),
        })
    }
}

pub type EmitterState = SPParticleEmitterState;

pub type ParticleState = SPParticleState;

/*
 * Tests
 */

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, PartialEq, FromPrimitive, ToPrimitive)]
    enum RenderGroupId {
        // TODO: Some way to put the render group type data in here
        Fire,
        Smoke,
    }

    #[derive(Debug, PartialEq, FromPrimitive, ToPrimitive)]
    enum EmitterTypeId {
        Campfire,
    }

    #[test]
    fn test_render_group_info_from_sp() {
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

    #[test]
    fn test_sp_particle_render_group_info_from_rs() {
        let render_group = RenderGroupInfo {
            shader_name: "fire".to_string(),
            id: RenderGroupId::Fire,
            vertex_descriptions: vec![
                VertexAttributeType::Vec3,
                VertexAttributeType::Vec2,
                VertexAttributeType::Vec4,
            ],
        };

        let sp_render_group: SPParticleRenderGroupInfo = render_group.try_into().unwrap();

        assert_eq!(
            unsafe { CStr::from_ptr(sp_render_group.shaderName) }
                .to_str()
                .unwrap(),
            "fire"
        );
        assert_eq!(sp_render_group.localID, 0);

        let sp_vertex_description_types = unsafe {
            Vec::from_raw_parts(
                sp_render_group.vertexDescriptionTypes,
                sp_render_group.vertexDescriptionTypeCount as _,
                sp_render_group.vertexDescriptionTypeCount as _,
            )
        };

        assert_eq!(sp_render_group.vertexDescriptionTypeCount, 3);
        assert_eq!(
            sp_vertex_description_types[0],
            SPRenderGroupVertexDescriptionType_SPRenderGroupVertexDescriptionType_vec3
        );
        assert_eq!(
            sp_vertex_description_types[1],
            SPRenderGroupVertexDescriptionType_SPRenderGroupVertexDescriptionType_vec2
        );
        assert_eq!(
            sp_vertex_description_types[2],
            SPRenderGroupVertexDescriptionType_SPRenderGroupVertexDescriptionType_vec4
        );
    }

    #[test]
    fn test_emitter_type_info_from_sp() {
        let name = CString::new("campfire").unwrap();

        let sp_emitter_type_info = SPParticleEmitterTypeInfo {
            name: name.into_raw(),
            localID: ToPrimitive::to_u32(&EmitterTypeId::Campfire).unwrap(),
        };

        let emitter_type_info: EmitterTypeInfo<EmitterTypeId> =
            TryFrom::try_from(sp_emitter_type_info).unwrap();

        assert_eq!(emitter_type_info.name, "campfire".to_string());
        assert_eq!(emitter_type_info.id, EmitterTypeId::Campfire);
    }

    #[test]
    fn test_sp_emitter_type_info_from_rs() {
        let emitter_type_info = EmitterTypeInfo {
            name: "campfire".to_string(),
            id: EmitterTypeId::Campfire,
        };

        let sp_emitter_type_info: SPParticleEmitterTypeInfo = emitter_type_info.try_into().unwrap();

        assert_eq!(
            unsafe { CStr::from_ptr(sp_emitter_type_info.name) }
                .to_str()
                .unwrap(),
            "campfire"
        );
        assert_eq!(sp_emitter_type_info.localID, 0);
    }
}
