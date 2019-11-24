use crate::sp::particles::EmitterTypeInfo;
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

/// Returns the number of emitter types that your mod defines
#[cfg(feature = "particle")]
pub fn get_emitter_types_count() -> i32 {
    EmitterType::Count as i32
}

#[cfg(feature = "particle")]
pub fn get_emitter_types() -> Vec<EmitterTypeInfo<EmitterType>> {
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
fn get_render_group_types_count() -> u32 {
    4
}
