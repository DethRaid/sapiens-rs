use crate::sp::particles::{EmitterTypeInfo, RenderGroupInfo, VertexAttributeType};
use num_derive::{FromPrimitive, ToPrimitive};

/// Base of a particles mod
///
/// Particles mods define how particle systems act. You can add more particles to existing emitters, change the behavior
/// of individual particles, etc

#[derive(Debug, PartialEq, FromPrimitive, ToPrimitive)]
#[repr(i32)]
pub enum EmitterType {
    Campfire,
    WoodChop,
    Feathers,

    // Add new emitter types here. Be sure to update get_emitter_types to return data for any emitter types you add

    // Final value so we know the number of items in the enum
    Count,
}

#[derive(Debug, PartialEq, FromPrimitive, ToPrimitive)]
#[repr(i32)]
pub enum RenderGroup {
    Smoke,
    Fire,
    Standard,
    Spark,
}

const VERTEX_DESCRIPTION: [VertexAttributeType; 3] = [
    VertexAttributeType::Vec3,
    VertexAttributeType::Vec2,
    VertexAttributeType::Vec4,
];

/// Returns a list of all the emitter types that your mod defines
#[cfg(feature = "particle")]
pub fn get_emitter_types() -> Vec<EmitterTypeInfo<EmitterType>> {
    // TODO: Figure out how to make this static data that doesn't get created on every invocation
    vec![
        EmitterTypeInfo {
            name: "campfire".to_string(),
            id: EmitterType::Campfire,
        },
        EmitterTypeInfo {
            name: "woodChop".to_string(),
            id: EmitterType::WoodChop,
        },
        EmitterTypeInfo {
            name: "feathers".to_string(),
            id: EmitterType::Feathers,
        },
    ]
}

#[cfg(feature = "particle")]
pub fn get_render_group_types() -> Vec<RenderGroupInfo<RenderGroup>> {
    vec![
        RenderGroupInfo {
            shader_name: "smokeParticle".to_string(),
            id: RenderGroup::Smoke,
            vertex_descriptions: &VERTEX_DESCRIPTION,
        },
        RenderGroupInfo {
            shader_name: "fireParticle".to_string(),
            id: RenderGroup::Fire,
            vertex_descriptions: &VERTEX_DESCRIPTION,
        },
        RenderGroupInfo {
            shader_name: "particle".to_string(),
            id: RenderGroup::Standard,
            vertex_descriptions: &VERTEX_DESCRIPTION,
        },
        RenderGroupInfo {
            shader_name: "spark".to_string(),
            id: RenderGroup::Spark,
            vertex_descriptions: &VERTEX_DESCRIPTION,
        },
    ]
}
