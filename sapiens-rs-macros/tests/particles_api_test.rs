extern crate sapiens_rs_macros;
extern crate sapiens_sys;

use sapiens_rs::sp::particles::RenderGroupVertexDescriptionType;
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
fn get_emitter_types() -> Vec<SPParticleEmitterTypeInfo> {
    vec![
        SPParticleEmitterTypeInfo {
            name: CString::new("campfire").unwrap().into_raw(),
            localID: VanillaEmitterType::Campfire as u32,
        },
        SPParticleEmitterTypeInfo {
            name: CString::new("woodChop").unwrap().into_raw(),
            localID: VanillaEmitterType::WoodChop as u32,
        },
        SPParticleEmitterTypeInfo {
            name: CString::new("feathers").unwrap().into_raw(),
            localID: VanillaEmitterType::Feathers as u32,
        },
    ]
}

#[export_to_sapiens]
fn get_render_group_types_count() -> u32 {
    4
}

static VERTEX_DESCRIPTION_TYPES: [RenderGroupVertexDescriptionType; 3] = [
    RenderGroupVertexDescriptionType::Vec3,
    RenderGroupVertexDescriptionType::Vec2,
    RenderGroupVertexDescriptionType::Vec4,
];

#[export_to_sapiens]
fn get_render_group_types() -> Vec<SPParticleRenderGroupInfo> {
    vec![
        SPParticleRenderGroupInfo {
            shaderName: CString::new("smokeParticle").unwrap().into_raw(),
            localID: VanillaRenderGroups::Smoke as u32,
            vertexDescriptionTypeCount: VERTEX_DESCRIPTION_TYPES.len() as i32,
            vertexDescriptionTypes: VERTEX_DESCRIPTION_TYPES.as_ptr() as _,
        },
        SPParticleRenderGroupInfo {
            shaderName: CString::new("fireParticle").unwrap().into_raw(),
            localID: VanillaRenderGroups::Fire as u32,
            vertexDescriptionTypeCount: VERTEX_DESCRIPTION_TYPES.len() as i32,
            vertexDescriptionTypes: VERTEX_DESCRIPTION_TYPES.as_ptr() as _,
        },
        SPParticleRenderGroupInfo {
            shaderName: CString::new("particle").unwrap().into_raw(),
            localID: VanillaRenderGroups::Standard as u32,
            vertexDescriptionTypeCount: VERTEX_DESCRIPTION_TYPES.len() as i32,
            vertexDescriptionTypes: VERTEX_DESCRIPTION_TYPES.as_ptr() as _,
        },
        SPParticleRenderGroupInfo {
            shaderName: CString::new("spark").unwrap().into_raw(),
            localID: VanillaRenderGroups::Spark as u32,
            vertexDescriptionTypeCount: VERTEX_DESCRIPTION_TYPES.len() as i32,
            vertexDescriptionTypes: VERTEX_DESCRIPTION_TYPES.as_ptr() as _,
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
