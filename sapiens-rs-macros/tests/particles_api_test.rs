extern crate sapiens_rs_macros;
extern crate sapiens_sys;

use sapiens_rs::sp::particles::{EmitterTypeInfo, RenderGroupInfo, VertexAttributeType};
use sapiens_rs_macros::export_to_sapiens;
use sapiens_sys::{SPParticleEmitterTypeInfo, SPParticleRenderGroupInfo};
use std::ffi::CString;

#[repr(u32)]
enum VanillaEmitterType {
    Campfire = 0,
    WoodChop = 1,
    Feathers = 2,
}

#[repr(u32)]
enum VanillaRenderGroups {
    Smoke = 0,
    Fire = 1,
    Standard = 2,
    Spark = 3,
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

static VERTEX_DESCRIPTION_TYPES: [VertexAttributeType; 3] = [
    VertexAttributeType::Vec3,
    VertexAttributeType::Vec2,
    VertexAttributeType::Vec4,
];

#[export_to_sapiens]
fn get_render_group_types() -> Vec<RenderGroupInfo<VanillaRenderGroups>> {
    vec![
        RenderGroupInfo {
            shader_name: "smokeParticle".to_string(),
            id: VanillaRenderGroups::Smoke,
            vertex_descriptions: vec![
                VertexAttributeType::Vec3,
                VertexAttributeType::Vec2,
                VertexAttributeType::Vec4,
            ],
        },
        RenderGroupInfo {
            shader_name: "fireParticle".to_string(),
            id: VanillaRenderGroups::Fire,
            vertex_descriptions: vec![
                VertexAttributeType::Vec3,
                VertexAttributeType::Vec2,
                VertexAttributeType::Vec4,
            ],
        },
        RenderGroupInfo {
            shader_name: "particle".to_string(),
            id: VanillaRenderGroups::Standard,
            vertex_descriptions: vec![
                VertexAttributeType::Vec3,
                VertexAttributeType::Vec2,
                VertexAttributeType::Vec4,
            ],
        },
        RenderGroupInfo {
            shader_name: "spark".to_string(),
            id: VanillaRenderGroups::Spark,
            vertex_descriptions: vec![
                VertexAttributeType::Vec3,
                VertexAttributeType::Vec2,
                VertexAttributeType::Vec4,
            ],
        },
    ]
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

        let ffi_emitters: Vec<(CString, u32)> = unsafe {
            Vec::<SPParticleEmitterTypeInfo>::from_raw_parts(
                raw_ffi_emitters,
                emitter_types_count,
                emitter_types_count,
            )
        }
        .iter()
        .map(|emitter| (unsafe { CString::from_raw(emitter.name) }, emitter.localID))
        .collect();

        let emitters: Vec<(CString, u32)> = get_emitter_types()
            .iter()
            .map(|emitter| (unsafe { CString::from_raw(emitter.name) }, emitter.localID))
            .collect();

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

        let ffi_render_types: Vec<(CString, u32, i32, *mut i32)> = unsafe {
            Vec::<SPParticleRenderGroupInfo>::from_raw_parts(
                raw_ffi_render_types,
                render_group_types_count,
                render_group_types_count,
            )
        }
        .iter()
        .map(|render_group| {
            (
                unsafe { CString::from_raw(render_group.shaderName) },
                render_group.localID,
                render_group.vertexDescriptionTypeCount,
                render_group.vertexDescriptionTypes,
            )
        })
        .collect();

        let render_types: Vec<(CString, u32, i32, *mut i32)> = get_render_group_types()
            .iter()
            .map(|render_group| {
                (
                    unsafe { CString::from_raw(render_group.shaderName) },
                    render_group.localID,
                    render_group.vertexDescriptionTypeCount,
                    render_group.vertexDescriptionTypes,
                )
            })
            .collect();

        assert_eq!(render_types, ffi_render_types);
    }
}
