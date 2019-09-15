//! Reimplementation of
//! https://github.com/mjdave/sapiens-mod-creation/blob/master/SPlugins/Examples/SPVanilla/src/SPParticles.c to show how
//! to use the Rust API to make Sapiens particles

extern crate num_derive;
extern crate num_traits;
extern crate sapiens_rs_macros;
extern crate sapiens_sys;

use num_derive::{FromPrimitive, ToPrimitive};
use num_traits::{FromPrimitive, ToPrimitive};
use sapiens_rs::sp::particles::{
    EmitterState, EmitterTypeInfo, RenderGroupInfo, VertexAttributeType,
};
use sapiens_rs_macros::export_to_sapiens;
use sapiens_sys::{SPParticleEmitterTypeInfo, SPParticleRenderGroupInfo};

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
    thread_state: &mut ThreadState<_, VanillaRenderType>,
    emitter_state: &EmitterState,
    emitter_type: VanillaEmitterType,
) -> bool {
    let mut remove_immediately = false;

    match emitter_type {
        VanillaEmitterType::Campfire => {}
        VanillaEmitterType::WoodChop => {
            remove_immediately = true;

            let pos_length = spVec3Length(emitter_state.p);
        }
        VanillaEmitterType::Feathers => {}
    }

    true
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
