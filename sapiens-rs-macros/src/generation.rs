//! Actually generates the FFI code

use crate::particles::{
    generate_get_emitter_type_count_func, generate_get_emitter_types,
    generate_get_render_group_types_count,
};
use std::convert::TryFrom;

#[allow(non_camel_case_types)]
#[derive(Debug)]
pub(crate) enum SapiensApiFunctions {
    spGetEmitterTypesCount,
    spGetEmitterTypes,
    spGetRenderGroupTypesCount,
}

impl TryFrom<String> for SapiensApiFunctions {
    type Error = ();

    fn try_from(value: String) -> Result<Self, Self::Error> {
        if &value == "get_emitter_types_count" {
            Ok(SapiensApiFunctions::spGetEmitterTypesCount)
        } else if &value == "get_emitter_types" {
            Ok(SapiensApiFunctions::spGetEmitterTypes)
        } else if &value == "get_render_group_types_count" {
            Ok(SapiensApiFunctions::spGetRenderGroupTypesCount)
        } else {
            Err(())
        }
    }
}

/// Generates FFI code so that Sapiens can call functions that we expose
///
/// This macro looks at your function name to determine which Sapiens hook to generate code for. The following function
/// names are supported:
/// - get_emitter_types_count -> spGetEmitterTypesCount
/// - get_emitter_types -> spGetEmitterTypes
/// - get_render_group_types_count -> spGetRenderGroupTypesCount
/// - get_render_group_types -> spGetRenderGroupTypes
/// - emitter_was_added -> spEmitterWasAdded
/// - update_emitter -> spUpdateEmitter
/// - update_particle -> spUpdateParticle
pub fn generate_binding(func: syn::ItemFn) -> proc_macro::TokenStream {
    let func_name = SapiensApiFunctions::try_from(format!("{}", func.sig.ident)).unwrap();
    match func_name {
        SapiensApiFunctions::spGetEmitterTypesCount => generate_get_emitter_type_count_func(func),
        SapiensApiFunctions::spGetEmitterTypes => generate_get_emitter_types(func),
        SapiensApiFunctions::spGetRenderGroupTypesCount => {
            generate_get_render_group_types_count(func)
        }
    }
}
