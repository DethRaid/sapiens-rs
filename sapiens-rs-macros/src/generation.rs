//! Actually generates the FFI code

use crate::particles::{
    generate_get_emitter_type_count_func, generate_get_emitter_types,
    generate_get_render_group_types, generate_get_render_group_types_count,
};
use proc_macro::TokenStream;
use quote::quote;
use std::convert::TryFrom;

#[allow(non_camel_case_types)]
#[derive(Debug)]
pub(crate) enum SapiensApiFunctions {
    spGetEmitterTypesCount,
    spGetEmitterTypes,
    spGetRenderGroupTypesCount,
    spGetRenderGroupTypes,
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
        } else if &value == "get_render_group_types" {
            Ok(SapiensApiFunctions::spGetRenderGroupTypes)
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
pub fn generate_binding(func: syn::ItemFn) -> TokenStream {
    let func_name = SapiensApiFunctions::try_from(format!("{}", func.sig.ident)).unwrap();
    match func_name {
        SapiensApiFunctions::spGetEmitterTypesCount => generate_get_emitter_type_count_func(func),
        SapiensApiFunctions::spGetEmitterTypes => generate_get_emitter_types(func),
        SapiensApiFunctions::spGetRenderGroupTypesCount => {
            generate_get_render_group_types_count(func)
        }
        SapiensApiFunctions::spGetRenderGroupTypes => generate_get_render_group_types(func),
    }
}

pub fn generate_enum_try_from_and_try_into(item_enum: syn::ItemEnum) -> TokenStream {
    let enum_name = item_enum.ident.clone();

    let mut enum_members = Vec::with_capacity(item_enum.variants.len());
    let mut enum_values = Vec::with_capacity(item_enum.variants.len());
    let mut enum_value: u32 = 0;

    item_enum.variants.iter().for_each(|variant| {
        enum_members.push(variant);

        enum_values.push(enum_value);
        enum_value += 1;
    });

    let ast = quote! {
        #[repr(u32)]
        #[derive(Debug, PartialEq)]
        #item_enum

        impl TryFrom<u32> for #enum_name {
            type Error = ();

            fn try_from(val: u32) -> Self {
                #( if val == #enum_values { Ok(#enum_members) } )*
                else {
                    Err(())
                }
            }
        }

        impl TryInto<u32> for #enum_name {
            type Error = ();

            fn try_into(self) -> u32 {
                match self {
                    #( #enum_members = Ok(#enum_values), )*
                }
            }
        }
    };

    ast.into()
}
